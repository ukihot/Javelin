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
        closing::{
            EvaluateMaterialityInteractor, GenerateComprehensiveFinancialStatementsInteractor,
            VerifyLedgerConsistencyInteractor,
        },
    },
    projection_builder::ProjectionBuilder,
};
use javelin_domain::event::DomainEvent;
use javelin_infrastructure::{
    read::{
        batch_history::BatchHistoryQueryServiceImpl,
        infrastructure::{ProjectionBuilderImpl, ProjectionDb},
        journal_entry::JournalEntrySearchQueryServiceImpl,
        ledger::LedgerQueryServiceImpl,
    },
    write::event_store::{ClosingEventStore, EventStore},
};
use tokio::sync::mpsc;

use crate::app_error::{AppError, AppResult};

/// マスタデータの初期化（イベントが0件の場合のみ）
async fn initialize_master_data(event_store: &Arc<EventStore>) -> AppResult<()> {
    use javelin_domain::masters::{AccountMasterEvent, AccountType};

    // イベントが既に存在する場合はスキップ
    let latest_sequence =
        event_store.get_latest_sequence().await.map(|seq| seq.as_u64()).unwrap_or(0);
    if latest_sequence > 0 {
        return Ok(());
    }

    println!("✓ Initializing master data with IFRS template...");

    // IFRSベースの勘定科目テンプレート
    let accounts = vec![
        // 資産の部 - 流動資産
        ("1100", "現金及び現金同等物", AccountType::Asset),
        ("1110", "営業債権及びその他の債権", AccountType::Asset),
        ("1120", "棚卸資産", AccountType::Asset),
        ("1130", "その他の金融資産（流動）", AccountType::Asset),
        ("1140", "その他の流動資産", AccountType::Asset),
        // 資産の部 - 非流動資産
        ("1200", "有形固定資産", AccountType::Asset),
        ("1210", "使用権資産", AccountType::Asset),
        ("1220", "のれん", AccountType::Asset),
        ("1230", "無形資産", AccountType::Asset),
        ("1240", "持分法で会計処理されている投資", AccountType::Asset),
        ("1250", "その他の金融資産（非流動）", AccountType::Asset),
        ("1260", "繰延税金資産", AccountType::Asset),
        ("1270", "その他の非流動資産", AccountType::Asset),
        // 負債の部 - 流動負債
        ("2100", "営業債務及びその他の債務", AccountType::Liability),
        ("2110", "社債及び借入金（流動）", AccountType::Liability),
        ("2120", "リース負債（流動）", AccountType::Liability),
        ("2130", "その他の金融負債（流動）", AccountType::Liability),
        ("2140", "未払法人所得税", AccountType::Liability),
        ("2150", "引当金（流動）", AccountType::Liability),
        ("2160", "その他の流動負債", AccountType::Liability),
        // 負債の部 - 非流動負債
        ("2200", "社債及び借入金（非流動）", AccountType::Liability),
        ("2210", "リース負債（非流動）", AccountType::Liability),
        ("2220", "その他の金融負債（非流動）", AccountType::Liability),
        ("2230", "退職給付に係る負債", AccountType::Liability),
        ("2240", "引当金（非流動）", AccountType::Liability),
        ("2250", "繰延税金負債", AccountType::Liability),
        ("2260", "その他の非流動負債", AccountType::Liability),
        // 資本の部
        ("3100", "資本金", AccountType::Equity),
        ("3110", "資本剰余金", AccountType::Equity),
        ("3120", "利益剰余金", AccountType::Equity),
        ("3130", "自己株式", AccountType::Equity),
        ("3140", "その他の資本の構成要素", AccountType::Equity),
        ("3150", "非支配持分", AccountType::Equity),
        // 収益の部
        ("4100", "売上収益", AccountType::Revenue),
        ("4200", "その他の収益", AccountType::Revenue),
        ("4300", "金融収益", AccountType::Revenue),
        ("4400", "持分法による投資利益", AccountType::Revenue),
        // 費用の部
        ("5100", "売上原価", AccountType::Expense),
        ("5200", "販売費及び一般管理費", AccountType::Expense),
        ("5300", "研究開発費", AccountType::Expense),
        ("5400", "その他の費用", AccountType::Expense),
        ("5500", "金融費用", AccountType::Expense),
        ("5600", "持分法による投資損失", AccountType::Expense),
        ("5700", "法人所得税費用", AccountType::Expense),
    ];

    for (code, name, account_type) in &accounts {
        let event = AccountMasterEvent::AccountMasterCreated {
            code: code.to_string(),
            name: name.to_string(),
            account_type: *account_type,
            is_active: true,
        };

        let payload = serde_json::to_vec(&event).map_err(|e| {
            AppError::InitializationFailed(Box::new(std::io::Error::other(e.to_string())))
        })?;

        event_store
            .append_event(
                event.event_type(),
                event.aggregate_id(),
                event.version(),
                javelin_infrastructure::shared::types::ExpectedVersion::any(),
                &payload,
            )
            .await
            .map_err(|e| {
                AppError::InitializationFailed(Box::new(std::io::Error::other(e.to_string())))
            })?;
    }

    println!("✓ Master data initialized ({} accounts)", accounts.len());

    Ok(())
}

/// インフラ層のセットアップ結果
pub struct InfrastructureComponents {
    pub event_store: Arc<EventStore>,
    pub projection_db: Arc<ProjectionDb>,
    pub projection_builder: Arc<ProjectionBuilderImpl>,
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

    // 初期データの投入（イベントが0件の場合のみ）
    initialize_master_data(&event_store).await?;

    // Projection再構築チェック
    check_and_rebuild_projections(&event_store, &projection_db, &projection_builder).await?;

    Ok(InfrastructureComponents {
        event_store,
        projection_db,
        projection_builder,
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
    _data_dir: &Path,
    event_store: Arc<EventStore>,
    projection_db: Arc<ProjectionDb>,
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

    // 個別マスタQueryService構築
    let account_master_query_service = Arc::new(
        javelin_infrastructure::read::account_master::AccountMasterQueryServiceImpl::new(
            Arc::clone(&projection_db),
        ),
    );
    let company_master_query_service = Arc::new(
        javelin_infrastructure::read::company_master::CompanyMasterQueryServiceImpl::new(
            Arc::clone(&projection_db),
        ),
    );
    let application_settings_master_query_service = Arc::new(
        javelin_infrastructure::read::application_settings_master::ApplicationSettingsMasterQueryServiceImpl::new(
            Arc::clone(&projection_db),
        ),
    );
    let subsidiary_account_master_query_service = Arc::new(
        javelin_infrastructure::read::subsidiary_account_master::SubsidiaryAccountMasterQueryServiceImpl::new(
            Arc::clone(&projection_db),
        ),
    );

    // PresenterRegistry
    let presenter_registry = Arc::new(PresenterRegistry::new());

    // マスタコントローラ構築（個別QueryServiceを使用）
    let account_master_controller = Arc::new(AccountMasterController::new(
        Arc::clone(&account_master_query_service),
        Arc::clone(&presenter_registry),
    ));
    let application_settings_controller = Arc::new(ApplicationSettingsController::new(
        Arc::clone(&application_settings_master_query_service),
        Arc::clone(&presenter_registry),
    ));
    let company_master_controller = Arc::new(CompanyMasterController::new(
        Arc::clone(&company_master_query_service),
        Arc::clone(&presenter_registry),
    ));
    let subsidiary_account_master_controller = Arc::new(SubsidiaryAccountMasterController::new(
        Arc::clone(&subsidiary_account_master_query_service),
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

    // Phase 3 Interactors
    let evaluate_materiality_interactor = Arc::new(EvaluateMaterialityInteractor::new());
    let verify_ledger_consistency_interactor = Arc::new(VerifyLedgerConsistencyInteractor::new());
    let generate_comprehensive_financial_statements_interactor =
        Arc::new(GenerateComprehensiveFinancialStatementsInteractor::new());

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
        evaluate_materiality_interactor,
        verify_ledger_consistency_interactor,
        generate_comprehensive_financial_statements_interactor,
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

    // Phase 3 Presenters構築
    let (
        materiality_evaluation_result_tx,
        _materiality_evaluation_result_rx,
        materiality_evaluation_progress_tx,
        _materiality_evaluation_progress_rx,
    ) = javelin_adapter::presenter::MaterialityEvaluationPresenter::create_channels();
    let materiality_evaluation_presenter =
        Arc::new(javelin_adapter::presenter::MaterialityEvaluationPresenter::new(
            materiality_evaluation_result_tx,
            materiality_evaluation_progress_tx,
        ));

    let (
        ledger_consistency_result_tx,
        _ledger_consistency_result_rx,
        ledger_consistency_progress_tx,
        _ledger_consistency_progress_rx,
    ) = javelin_adapter::presenter::LedgerConsistencyVerificationPresenter::create_channels();
    let ledger_consistency_verification_presenter =
        Arc::new(javelin_adapter::presenter::LedgerConsistencyVerificationPresenter::new(
            ledger_consistency_result_tx,
            ledger_consistency_progress_tx,
        ));

    let (
        comprehensive_fs_result_tx,
        _comprehensive_fs_result_rx,
        comprehensive_fs_progress_tx,
        _comprehensive_fs_progress_rx,
    ) = javelin_adapter::presenter::ComprehensiveFinancialStatementsPresenter::create_channels();
    let comprehensive_financial_statements_presenter =
        Arc::new(javelin_adapter::presenter::ComprehensiveFinancialStatementsPresenter::new(
            comprehensive_fs_result_tx,
            comprehensive_fs_progress_tx,
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
        materiality_evaluation_presenter,
        ledger_consistency_verification_presenter,
        comprehensive_financial_statements_presenter,
    );

    // View層の構築

    println!("✓ Application components initialized");
    println!("  - Controllers: AccountMaster, JournalEntry, Closing, Search");
    println!("  - Navigation: Stack-based architecture");
    println!("  - PresenterRegistry: Ready");

    Ok(ControllerComponents { controllers, presenter_registry })
}
