// AppSetup - インフラ層のセットアップ
// 責務: リポジトリ、Interactor、コントローラの初期化

use std::{path::Path, sync::Arc};

use javelin_adapter::{
    PresenterRegistry,
    controller::{
        AccountMasterController, ApplicationSettingsController, BatchHistoryController,
        ClosingController, CompanyMasterController, JournalEntryController, LedgerController,
        SearchController, SubsidiaryAccountMasterController,
    },
    navigation::Controllers,
    presenter::LedgerPresenter,
};
use javelin_application::{
    interactor::{
        AdjustAccountsInteractor, ApplyIfrsValuationInteractor, ConsolidateLedgerInteractor,
        GenerateFinancialStatementsInteractor, GenerateNoteDraftInteractor,
        GenerateTrialBalanceInteractor, LockClosingPeriodInteractor, PrepareClosingInteractor,
    },
    projection_builder::ProjectionBuilder,
    query_service::MasterDataLoaderService,
};
use javelin_infrastructure::{
    read::{
        batch_history::BatchHistoryQueryServiceImpl,
        infrastructure::{ProjectionBuilderImpl, ProjectionDb},
        journal_entry::JournalEntrySearchQueryServiceImpl,
        ledger::LedgerQueryServiceImpl,
        master_data::MasterDataLoaderImpl,
    },
    write::{
        event_store::{ClosingEventStore, EventStore},
        repositories::SubsidiaryAccountMasterRepositoryImpl,
    },
};
use tokio::sync::mpsc;

use crate::app_error::{AppError, AppResult};

/// インフラ層のセットアップ結果
pub struct InfrastructureComponents {
    pub event_store: Arc<EventStore>,
    pub projection_db: Arc<ProjectionDb>,
    pub projection_builder: Arc<ProjectionBuilderImpl>,
    pub master_data_loader: Arc<MasterDataLoaderImpl>,
    pub infra_error_receiver: mpsc::UnboundedReceiver<String>,
}

/// コントローラのセットアップ結果
pub struct ControllerComponents {
    pub controllers: Controllers,
    pub presenter_registry: Arc<PresenterRegistry>,
}

/// インフラ層をセットアップ
pub async fn setup_infrastructure(data_dir: &Path) -> AppResult<InfrastructureComponents> {
    // データディレクトリの作成
    if !data_dir.exists() {
        tokio::fs::create_dir_all(&data_dir).await.map_err(|e| {
            AppError::DataDirectoryCreationFailed {
                path: data_dir.display().to_string(),
                source: e,
            }
        })?;
    }

    // Infrastructure層の構築
    let event_store = Arc::new(EventStore::new(&data_dir.join("events")).await?);
    let projection_db = Arc::new(ProjectionDb::new(&data_dir.join("projections")).await?);

    // インフラエラー通知チャネル
    let (infra_error_sender, infra_error_receiver) = mpsc::unbounded_channel();

    // ProjectionBuilderの構築
    let projection_builder =
        Arc::new(ProjectionBuilderImpl::new(Arc::clone(&projection_db), Arc::clone(&event_store)));

    // イベント通知ハンドラを登録
    let notification_handler =
        projection_builder.clone().create_event_notification_handler(infra_error_sender);
    event_store.set_notification_callback(notification_handler);

    // Projection再構築チェック
    check_and_rebuild_projections(&event_store, &projection_db, &projection_builder).await?;

    // マスタデータローダー
    let master_db_path = data_dir.join("master_data");
    let master_data_loader = Arc::new(
        MasterDataLoaderImpl::new(&master_db_path)
            .await
            .map_err(AppError::InitializationFailed)?,
    );

    // 初期データロード確認
    let master_data = master_data_loader.load_master_data().await?;
    println!("✓ Master data loaded successfully");
    println!("  - Accounts: {}", master_data.accounts.len());
    println!("  - Companies: {}", master_data.companies.len());
    println!("  - Language: {}", master_data.user_options.language);

    Ok(InfrastructureComponents {
        event_store,
        projection_db,
        projection_builder,
        master_data_loader,
        infra_error_receiver,
    })
}

/// Projection再構築チェック
async fn check_and_rebuild_projections(
    event_store: &Arc<EventStore>,
    projection_db: &Arc<ProjectionDb>,
    projection_builder: &Arc<ProjectionBuilderImpl>,
) -> AppResult<()> {
    let latest_sequence =
        event_store.get_latest_sequence().await.map(|seq| seq.as_u64()).unwrap_or(0);
    let projection_position = projection_db.get_position("main", 1).await.unwrap_or(0);

    if projection_position < latest_sequence {
        println!("✓ Projection rebuild required");
        println!("  - Latest event sequence: {}", latest_sequence);
        println!("  - Projection position: {}", projection_position);
        println!("  - Rebuilding projections...");

        projection_builder.rebuild_all_projections().await?;

        println!("✓ Projection rebuild completed");
    } else {
        println!("✓ Projections are up to date");
        println!("  - Latest event sequence: {}", latest_sequence);
        println!("  - Projection position: {}", projection_position);
    }

    Ok(())
}

/// コントローラをセットアップ
pub async fn setup_controllers(
    data_dir: &Path,
    event_store: Arc<EventStore>,
    projection_db: Arc<ProjectionDb>,
    master_data_loader: Arc<MasterDataLoaderImpl>,
) -> AppResult<ControllerComponents> {
    // LedgerPresenterチャネル
    let (trial_balance_tx, _trial_balance_rx) = {
        let (_, _, tb_tx, tb_rx) = LedgerPresenter::create_channels();
        (tb_tx, tb_rx)
    };
    let (dummy_ledger_tx, _) =
        tokio::sync::mpsc::unbounded_channel::<javelin_adapter::presenter::LedgerViewModel>();
    let _ledger_presenter = Arc::new(LedgerPresenter::new(dummy_ledger_tx, trial_balance_tx));

    // QueryService構築
    let ledger_query_service = Arc::new(LedgerQueryServiceImpl::new(Arc::clone(&projection_db)));
    let search_query_service =
        Arc::new(JournalEntrySearchQueryServiceImpl::new(Arc::clone(&projection_db)));
    let batch_history_query_service =
        Arc::new(BatchHistoryQueryServiceImpl::new(Arc::clone(&projection_db)));

    // PresenterRegistry
    let presenter_registry = Arc::new(PresenterRegistry::new());

    // マスタリポジトリの作成（補助科目のみ個別に必要）
    let master_db_path = data_dir.join("master_data");
    let subsidiary_account_master_repository = Arc::new(
        SubsidiaryAccountMasterRepositoryImpl::new(&master_db_path.join("subsidiary_accounts"))
            .await
            .map_err(AppError::InitializationFailed)?,
    );

    // マスタコントローラ構築（master_data_loaderとpresenter_registryを使用）
    let account_master_controller = Arc::new(AccountMasterController::new(
        Arc::clone(&master_data_loader),
        Arc::clone(&presenter_registry),
    ));
    let application_settings_controller = Arc::new(ApplicationSettingsController::new(
        Arc::clone(&master_data_loader),
        Arc::clone(&presenter_registry),
    ));
    let company_master_controller = Arc::new(CompanyMasterController::new(
        Arc::clone(&master_data_loader),
        Arc::clone(&presenter_registry),
    ));
    let subsidiary_account_master_controller = Arc::new(SubsidiaryAccountMasterController::new(
        Arc::clone(&subsidiary_account_master_repository),
        Arc::clone(&presenter_registry),
    ));

    // 業務コントローラ構築
    let journal_entry_controller = Arc::new(JournalEntryController::new(
        Arc::clone(&event_store),
        Arc::clone(&search_query_service),
        Arc::clone(&presenter_registry),
    ));

    let ledger_controller = Arc::new(LedgerController::new(Arc::clone(&ledger_query_service)));

    // 月次決算Interactor構築
    let closing_event_store = Arc::new(ClosingEventStore(Arc::clone(&event_store)));

    let consolidate_ledger_interactor =
        Arc::new(ConsolidateLedgerInteractor::new(Arc::clone(&ledger_query_service)));
    let prepare_closing_interactor =
        Arc::new(PrepareClosingInteractor::new(Arc::clone(&ledger_query_service)));
    let lock_closing_period_interactor =
        Arc::new(LockClosingPeriodInteractor::new(Arc::clone(&closing_event_store)));
    let generate_trial_balance_interactor =
        Arc::new(GenerateTrialBalanceInteractor::new(Arc::clone(&ledger_query_service)));
    let generate_note_draft_interactor =
        Arc::new(GenerateNoteDraftInteractor::new(Arc::clone(&ledger_query_service)));
    let adjust_accounts_interactor = Arc::new(AdjustAccountsInteractor::new(
        Arc::clone(&closing_event_store),
        Arc::clone(&ledger_query_service),
    ));
    let apply_ifrs_valuation_interactor = Arc::new(ApplyIfrsValuationInteractor::new(
        Arc::clone(&closing_event_store),
        Arc::clone(&ledger_query_service),
        Arc::clone(&_ledger_presenter),
    ));
    let generate_financial_statements_interactor =
        Arc::new(GenerateFinancialStatementsInteractor::new(Arc::clone(&ledger_query_service)));

    // ClosingController構築
    let closing_controller = Arc::new(ClosingController::new(
        consolidate_ledger_interactor,
        prepare_closing_interactor,
        lock_closing_period_interactor,
        generate_trial_balance_interactor,
        generate_note_draft_interactor,
        adjust_accounts_interactor,
        apply_ifrs_valuation_interactor,
        generate_financial_statements_interactor,
    ));

    // SearchController構築
    let search_controller = Arc::new(SearchController::new(
        Arc::clone(&search_query_service),
        Arc::clone(&presenter_registry),
    ));

    // BatchHistoryController構築
    let batch_history_controller = Arc::new(BatchHistoryController::new(
        Arc::clone(&batch_history_query_service),
        Arc::clone(&presenter_registry),
    ));

    // Controllers container
    let controllers = Controllers::new(
        account_master_controller,
        application_settings_controller,
        company_master_controller,
        subsidiary_account_master_controller,
        journal_entry_controller,
        closing_controller,
        ledger_controller,
        search_controller,
        batch_history_controller,
    );

    // View層の構築

    println!("✓ Application components initialized");
    println!("  - Controllers: AccountMaster, JournalEntry, Closing, Search");
    println!("  - Navigation: Stack-based architecture");
    println!("  - PresenterRegistry: Ready");

    Ok(ControllerComponents { controllers, presenter_registry })
}
