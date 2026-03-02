// Controllers container - Bundles all controllers for easy passing to pages
// Simplifies PageState::run() signature by grouping controllers

use std::sync::Arc;

use crate::{
    controller::{
        AccountMasterController, ApplicationSettingsController, BatchHistoryController,
        ClosingController, CompanyMasterController, JournalDetailController,
        JournalEntryController, LedgerController, SearchController,
        SubsidiaryAccountMasterController,
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

/// Type alias for JournalEntryController with concrete UseCase
pub type JournalEntryControllerType = JournalEntryController<
    javelin_application::interactor::RegisterJournalEntryInteractor<
        javelin_infrastructure::write::event_store::EventStore,
        crate::presenter::JournalEntryPresenter,
        javelin_infrastructure::read::journal_entry::JournalEntrySearchQueryServiceImpl,
    >,
>;

/// Type alias for JournalDetailController with concrete UseCase
pub type JournalDetailControllerType = JournalDetailController<
    javelin_application::interactor::GetJournalEntryDetailInteractor<
        javelin_infrastructure::read::journal_entry::JournalEntrySearchQueryServiceImpl,
        crate::presenter::JournalEntryPresenter,
    >,
>;

/// Type alias for SearchController (no generics needed)
pub type SearchControllerType = SearchController;

/// Type alias for BatchHistoryController (no generics needed)
pub type BatchHistoryControllerType = BatchHistoryController;

/// Type alias for ClosingController (no generics needed after refactoring)
pub type ClosingControllerType = ClosingController;

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
    pub closing: Arc<ClosingControllerType>,
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
        closing: Arc<ClosingControllerType>,
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
            closing,
            ledger,
            search,
            batch_history,
            materiality_evaluation_presenter,
            ledger_consistency_verification_presenter,
            comprehensive_financial_statements_presenter,
        }
    }
}
