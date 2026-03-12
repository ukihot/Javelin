// AppSetup - インフラ層のセットアップ
// 責務: リポジトリ、Interactor、コントローラの初期化

use std::{path::Path, sync::Arc};

use javelin_adapter::{
    PresenterRegistry, controller::LedgerController, navigation::Controllers,
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
use javelin_infrastructure::{
    read::{
        // batch_history::BatchHistoryQueryServiceImpl, // Disabled
        infrastructure::{ConcreteProjectionBuilder, ProjectionDb},
        invoice::MockInvoiceQueryService,
        journal_entry::JournalEntrySearchQueryServiceImpl,
        ledger::LedgerQueryServiceImpl,
    },
    shared::typst_invoice_printer::TypstInvoicePrinter,
    write::event_store::EventStore, // ClosingEventStore removed
};
use tokio::sync::mpsc;

use crate::app_error::{AppError, AppResult};

/// インフラ層のセットアップ結果
pub struct InfrastructureComponents {
    pub event_store: Arc<EventStore>,
    pub projection_db: Arc<ProjectionDb>,
    pub projection_builder: Arc<ConcreteProjectionBuilder>,
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
    let projection_builder = Arc::new(ConcreteProjectionBuilder::new(
        Arc::clone(&projection_db),
        Arc::clone(&event_store),
    ));

    // イベント通知ハンドラを登録
    let notification_handler =
        projection_builder.clone().create_event_notification_handler(infra_error_sender);
    event_store.set_notification_callback(notification_handler);

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
    projection_builder: &Arc<ConcreteProjectionBuilder>,
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
    // let batch_history_query_service =
    //     Arc::new(BatchHistoryQueryServiceImpl::new(Arc::clone(&projection_db))); // Disabled

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
    // let application_settings_master_query_service = Arc::new(
    //     javelin_infrastructure::read::application_settings_master::ApplicationSettingsMasterQueryServiceImpl::new(
    //         Arc::clone(&projection_db),
    //     ),
    // ); // Disabled: ApplicationSettings aggregate removed
    let subsidiary_account_master_query_service = Arc::new(
        javelin_infrastructure::read::subsidiary_account_master::SubsidiaryAccountMasterQueryServiceImpl::new(
            Arc::clone(&projection_db),
        ),
    );

    // PresenterRegistry
    let presenter_registry = Arc::new(PresenterRegistry::new());

    // マスタコントローラ構築（QueryServiceを渡す）
    let account_master_controller =
        Arc::new(javelin_adapter::controller::AccountMasterController::new(
            Arc::clone(&account_master_query_service),
            Arc::clone(&presenter_registry),
        ));

    let company_master_controller =
        Arc::new(javelin_adapter::controller::CompanyMasterController::new(
            Arc::clone(&company_master_query_service),
            Arc::clone(&presenter_registry),
        ));
    let subsidiary_account_master_controller =
        Arc::new(javelin_adapter::controller::SubsidiaryAccountMasterController::new(
            Arc::clone(&subsidiary_account_master_query_service),
            Arc::clone(&presenter_registry),
        ));

    // 業務コントローラ構築（QueryServiceを渡す）
    let journal_entry_repository =
        Arc::new(javelin_infrastructure::write::repositories::JournalEntryRepositoryImpl::new(
            Arc::clone(&event_store),
        ));

    let journal_entry_controller =
        Arc::new(javelin_adapter::controller::JournalEntryController::new(
            Arc::clone(&journal_entry_repository),
            Arc::clone(&search_query_service),
            Arc::clone(&presenter_registry),
        ));

    let journal_detail_controller =
        Arc::new(javelin_adapter::controller::JournalDetailController::new(
            Arc::clone(&search_query_service),
            Arc::clone(&presenter_registry),
        ));

    let search_controller = Arc::new(javelin_adapter::controller::SearchController::new(
        Arc::clone(&search_query_service),
        Arc::clone(&presenter_registry),
    ));

    let ledger_controller = Arc::new(LedgerController::new(Arc::clone(&ledger_query_service)));

    let consolidate_ledger_interactor =
        Arc::new(ConsolidateLedgerInteractor::new(Arc::clone(&ledger_query_service)));
    let prepare_closing_interactor =
        Arc::new(PrepareClosingInteractor::new(Arc::clone(&ledger_query_service)));
    let lock_closing_period_interactor = Arc::new(LockClosingPeriodInteractor::new()); // No arguments
    let generate_trial_balance_interactor =
        Arc::new(GenerateTrialBalanceInteractor::new(Arc::clone(&ledger_query_service)));
    let generate_note_draft_interactor =
        Arc::new(GenerateNoteDraftInteractor::new(Arc::clone(&ledger_query_service)));
    let adjust_accounts_interactor = Arc::new(AdjustAccountsInteractor::new(
        Arc::clone(&ledger_query_service), // Only 1 argument
    ));
    let apply_ifrs_valuation_interactor = Arc::new(ApplyIfrsValuationInteractor::new(
        Arc::clone(&ledger_query_service), // Only 1 argument
    ));
    let generate_financial_statements_interactor =
        Arc::new(GenerateFinancialStatementsInteractor::new(Arc::clone(&ledger_query_service)));

    // Phase 3 Interactors
    let evaluate_materiality_interactor = Arc::new(EvaluateMaterialityInteractor::new(
        Arc::clone(&ledger_query_service), // Needs 1 argument
    ));
    let verify_ledger_consistency_interactor = Arc::new(VerifyLedgerConsistencyInteractor::new(
        Arc::clone(&ledger_query_service), // Needs 1 argument
    ));
    let generate_comprehensive_financial_statements_interactor =
        Arc::new(GenerateComprehensiveFinancialStatementsInteractor::new(
            Arc::clone(&ledger_query_service), // Needs 1 argument
        ));

    // 11個の個別Closing Controller構築
    let consolidate_ledger_controller =
        Arc::new(javelin_adapter::controller::ConsolidateLedgerController::new(
            consolidate_ledger_interactor,
        ));
    let prepare_closing_controller = Arc::new(
        javelin_adapter::controller::PrepareClosingController::new(prepare_closing_interactor),
    );
    let _lock_closing_period_controller =
        Arc::new(javelin_adapter::controller::LockClosingPeriodController::new(
            lock_closing_period_interactor,
        ));
    let generate_trial_balance_controller =
        Arc::new(javelin_adapter::controller::GenerateTrialBalanceController::new(
            generate_trial_balance_interactor,
        ));
    let generate_note_draft_controller =
        Arc::new(javelin_adapter::controller::GenerateNoteDraftController::new(
            generate_note_draft_interactor,
        ));
    let _adjust_accounts_controller = Arc::new(
        javelin_adapter::controller::AdjustAccountsController::new(adjust_accounts_interactor),
    );
    let _apply_ifrs_valuation_controller =
        Arc::new(javelin_adapter::controller::ApplyIfrsValuationController::new(
            apply_ifrs_valuation_interactor,
        ));
    let generate_financial_statements_controller =
        Arc::new(javelin_adapter::controller::GenerateFinancialStatementsController::new(
            generate_financial_statements_interactor,
        ));
    let evaluate_materiality_controller =
        Arc::new(javelin_adapter::controller::EvaluateMaterialityController::new(
            evaluate_materiality_interactor,
        ));
    let verify_ledger_consistency_controller =
        Arc::new(javelin_adapter::controller::VerifyLedgerConsistencyController::new(
            verify_ledger_consistency_interactor,
        ));
    let generate_comprehensive_financial_statements_controller = Arc::new(
        javelin_adapter::controller::GenerateComprehensiveFinancialStatementsController::new(
            generate_comprehensive_financial_statements_interactor,
        ),
    );

    // InvoicePrint構築
    // MockInvoiceQueryService
    let mock_invoice_query_service = Arc::new(MockInvoiceQueryService);
    // TypstInvoicePrinter
    let typst_invoice_printer = Arc::new(TypstInvoicePrinter::default());
    // InvoicePrintController（プレゼンターは動的に注入されるため、ここでは作成しない）
    let invoice_print_controller =
        Arc::new(javelin_adapter::controller::InvoicePrintController::new(
            mock_invoice_query_service,
            typst_invoice_printer,
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
        company_master_controller,
        subsidiary_account_master_controller,
        journal_entry_controller,
        journal_detail_controller,
        consolidate_ledger_controller,
        prepare_closing_controller,
        generate_trial_balance_controller,
        generate_note_draft_controller,
        generate_financial_statements_controller,
        evaluate_materiality_controller,
        verify_ledger_consistency_controller,
        generate_comprehensive_financial_statements_controller,
        ledger_controller,
        search_controller,
        invoice_print_controller,
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
