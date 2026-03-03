// Controllers container - Bundles all controllers for easy passing to pages
// Simplifies PageState::run() signature by grouping controllers

use std::sync::Arc;

use crate::{
    controller::{
        AccountMasterController, AdjustAccountsController, ApplicationSettingsController,
        ApplyIfrsValuationController, BatchHistoryController, CompanyMasterController,
        ConsolidateLedgerController, EvaluateMaterialityController,
        GenerateComprehensiveFinancialStatementsController, GenerateFinancialStatementsController,
        GenerateNoteDraftController, GenerateTrialBalanceController, JournalDetailController,
        JournalEntryController, LedgerController, LockClosingPeriodController,
        PrepareClosingController, SearchController, SubsidiaryAccountMasterController,
        VerifyLedgerConsistencyController,
    },
    presenter::{
        ComprehensiveFinancialStatementsPresenter, LedgerConsistencyVerificationPresenter,
        MaterialityEvaluationPresenter,
    },
};

/// Type alias for AccountMasterController with concrete QueryService
pub type AccountMasterControllerType = AccountMasterController<
    javelin_infrastructure::read::account_master::AccountMasterQueryServiceImpl,
>;

/// Type alias for ApplicationSettingsController with concrete QueryService
pub type ApplicationSettingsControllerType = ApplicationSettingsController<
    javelin_infrastructure::read::application_settings_master::ApplicationSettingsMasterQueryServiceImpl,
>;

/// Type alias for CompanyMasterController with concrete QueryService
pub type CompanyMasterControllerType = CompanyMasterController<
    javelin_infrastructure::read::company_master::CompanyMasterQueryServiceImpl,
>;

/// Type alias for SubsidiaryAccountMasterController with concrete QueryService
pub type SubsidiaryAccountMasterControllerType = SubsidiaryAccountMasterController<
    javelin_infrastructure::read::subsidiary_account_master::SubsidiaryAccountMasterQueryServiceImpl,
>;

/// Type alias for JournalEntryController with concrete QueryService
pub type JournalEntryControllerType = JournalEntryController<
    javelin_infrastructure::read::journal_entry::JournalEntrySearchQueryServiceImpl,
>;

/// Type alias for JournalDetailController with concrete QueryService
pub type JournalDetailControllerType = JournalDetailController<
    javelin_infrastructure::read::journal_entry::JournalEntrySearchQueryServiceImpl,
>;

/// Type alias for SearchController with concrete QueryService
pub type SearchControllerType = SearchController<
    javelin_infrastructure::read::journal_entry::JournalEntrySearchQueryServiceImpl,
>;

/// Type alias for BatchHistoryController with concrete UseCase
pub type BatchHistoryControllerType = BatchHistoryController<
    javelin_application::interactor::GetBatchHistoryInteractor<
        javelin_infrastructure::read::batch_history::BatchHistoryQueryServiceImpl,
    >,
>;

/// Type alias for ConsolidateLedgerController with concrete UseCase
pub type ConsolidateLedgerControllerType = ConsolidateLedgerController<
    javelin_application::interactor::ConsolidateLedgerInteractor<
        javelin_infrastructure::read::ledger::LedgerQueryServiceImpl,
    >,
>;

/// Type alias for PrepareClosingController with concrete UseCase
pub type PrepareClosingControllerType = PrepareClosingController<
    javelin_application::interactor::PrepareClosingInteractor<
        javelin_infrastructure::read::ledger::LedgerQueryServiceImpl,
    >,
>;

/// Type alias for LockClosingPeriodController with concrete UseCase
pub type LockClosingPeriodControllerType = LockClosingPeriodController<
    javelin_application::interactor::LockClosingPeriodInteractor<
        javelin_infrastructure::write::event_store::ClosingEventStore,
    >,
>;

/// Type alias for GenerateTrialBalanceController with concrete UseCase
pub type GenerateTrialBalanceControllerType = GenerateTrialBalanceController<
    javelin_application::interactor::GenerateTrialBalanceInteractor<
        javelin_infrastructure::read::ledger::LedgerQueryServiceImpl,
    >,
>;

/// Type alias for GenerateNoteDraftController with concrete UseCase
pub type GenerateNoteDraftControllerType = GenerateNoteDraftController<
    javelin_application::interactor::GenerateNoteDraftInteractor<
        javelin_infrastructure::read::ledger::LedgerQueryServiceImpl,
    >,
>;

/// Type alias for AdjustAccountsController with concrete UseCase
pub type AdjustAccountsControllerType = AdjustAccountsController<
    javelin_application::interactor::AdjustAccountsInteractor<
        javelin_infrastructure::write::event_store::ClosingEventStore,
        javelin_infrastructure::read::ledger::LedgerQueryServiceImpl,
    >,
>;

/// Type alias for ApplyIfrsValuationController with concrete UseCase
pub type ApplyIfrsValuationControllerType = ApplyIfrsValuationController<
    javelin_application::interactor::ApplyIfrsValuationInteractor<
        javelin_infrastructure::write::event_store::ClosingEventStore,
        javelin_infrastructure::read::ledger::LedgerQueryServiceImpl,
        crate::presenter::LedgerPresenter,
    >,
>;

/// Type alias for GenerateFinancialStatementsController with concrete UseCase
pub type GenerateFinancialStatementsControllerType = GenerateFinancialStatementsController<
    javelin_application::interactor::GenerateFinancialStatementsInteractor<
        javelin_infrastructure::read::ledger::LedgerQueryServiceImpl,
    >,
>;

/// Type alias for EvaluateMaterialityController with concrete UseCase
pub type EvaluateMaterialityControllerType = EvaluateMaterialityController<
    javelin_application::interactor::closing::EvaluateMaterialityInteractor,
>;

/// Type alias for VerifyLedgerConsistencyController with concrete UseCase
pub type VerifyLedgerConsistencyControllerType = VerifyLedgerConsistencyController<
    javelin_application::interactor::closing::VerifyLedgerConsistencyInteractor,
>;

/// Type alias for GenerateComprehensiveFinancialStatementsController with concrete UseCase
pub type GenerateComprehensiveFinancialStatementsControllerType =
    GenerateComprehensiveFinancialStatementsController<
        javelin_application::interactor::closing::GenerateComprehensiveFinancialStatementsInteractor,
    >;

/// Type alias for LedgerController (no generics needed after refactoring)
pub type LedgerControllerType = LedgerController;

/// Container for all controllers
///
/// Bundles all controllers into a single struct for easy passing to pages.
/// This simplifies the PageState::run() signature and makes it easier to
/// add new controllers without changing existing page implementations.
pub struct Controllers {
    pub account_master: Arc<AccountMasterControllerType>,
    pub application_settings: Arc<ApplicationSettingsControllerType>,
    pub company_master: Arc<CompanyMasterControllerType>,
    pub subsidiary_account_master: Arc<SubsidiaryAccountMasterControllerType>,
    pub journal_entry: Arc<JournalEntryControllerType>,
    pub journal_detail: Arc<JournalDetailControllerType>,
    pub consolidate_ledger: Arc<ConsolidateLedgerControllerType>,
    pub prepare_closing: Arc<PrepareClosingControllerType>,
    pub lock_closing_period: Arc<LockClosingPeriodControllerType>,
    pub generate_trial_balance: Arc<GenerateTrialBalanceControllerType>,
    pub generate_note_draft: Arc<GenerateNoteDraftControllerType>,
    pub adjust_accounts: Arc<AdjustAccountsControllerType>,
    pub apply_ifrs_valuation: Arc<ApplyIfrsValuationControllerType>,
    pub generate_financial_statements: Arc<GenerateFinancialStatementsControllerType>,
    pub evaluate_materiality: Arc<EvaluateMaterialityControllerType>,
    pub verify_ledger_consistency: Arc<VerifyLedgerConsistencyControllerType>,
    pub generate_comprehensive_financial_statements:
        Arc<GenerateComprehensiveFinancialStatementsControllerType>,
    pub ledger: Arc<LedgerControllerType>,
    pub search: Arc<SearchControllerType>,
    pub batch_history: Arc<BatchHistoryControllerType>,
    pub materiality_evaluation_presenter: Arc<MaterialityEvaluationPresenter>,
    pub ledger_consistency_verification_presenter: Arc<LedgerConsistencyVerificationPresenter>,
    pub comprehensive_financial_statements_presenter:
        Arc<ComprehensiveFinancialStatementsPresenter>,
}

impl Controllers {
    /// Create a new Controllers container
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_master: Arc<AccountMasterControllerType>,
        application_settings: Arc<ApplicationSettingsControllerType>,
        company_master: Arc<CompanyMasterControllerType>,
        subsidiary_account_master: Arc<SubsidiaryAccountMasterControllerType>,
        journal_entry: Arc<JournalEntryControllerType>,
        journal_detail: Arc<JournalDetailControllerType>,
        consolidate_ledger: Arc<ConsolidateLedgerControllerType>,
        prepare_closing: Arc<PrepareClosingControllerType>,
        lock_closing_period: Arc<LockClosingPeriodControllerType>,
        generate_trial_balance: Arc<GenerateTrialBalanceControllerType>,
        generate_note_draft: Arc<GenerateNoteDraftControllerType>,
        adjust_accounts: Arc<AdjustAccountsControllerType>,
        apply_ifrs_valuation: Arc<ApplyIfrsValuationControllerType>,
        generate_financial_statements: Arc<GenerateFinancialStatementsControllerType>,
        evaluate_materiality: Arc<EvaluateMaterialityControllerType>,
        verify_ledger_consistency: Arc<VerifyLedgerConsistencyControllerType>,
        generate_comprehensive_financial_statements: Arc<
            GenerateComprehensiveFinancialStatementsControllerType,
        >,
        ledger: Arc<LedgerControllerType>,
        search: Arc<SearchControllerType>,
        batch_history: Arc<BatchHistoryControllerType>,
        materiality_evaluation_presenter: Arc<MaterialityEvaluationPresenter>,
        ledger_consistency_verification_presenter: Arc<LedgerConsistencyVerificationPresenter>,
        comprehensive_financial_statements_presenter: Arc<
            ComprehensiveFinancialStatementsPresenter,
        >,
    ) -> Self {
        Self {
            account_master,
            application_settings,
            company_master,
            subsidiary_account_master,
            journal_entry,
            journal_detail,
            consolidate_ledger,
            prepare_closing,
            lock_closing_period,
            generate_trial_balance,
            generate_note_draft,
            adjust_accounts,
            apply_ifrs_valuation,
            generate_financial_statements,
            evaluate_materiality,
            verify_ledger_consistency,
            generate_comprehensive_financial_statements,
            ledger,
            search,
            batch_history,
            materiality_evaluation_presenter,
            ledger_consistency_verification_presenter,
            comprehensive_financial_statements_presenter,
        }
    }
}
