// Controllers container - Bundles all controllers for easy passing to pages
// Simplifies PageState::run() signature by grouping controllers

use std::sync::Arc;

use crate::controller::{
    AccountMasterController, ApplicationSettingsController, BatchHistoryController,
    ClosingController, CompanyMasterController, JournalEntryController, LedgerController,
    SearchController, SubsidiaryAccountMasterController,
};

/// Type alias for AccountMasterController (no generics needed)
pub type AccountMasterControllerType = AccountMasterController;

/// Type alias for ApplicationSettingsController (no generics needed)
pub type ApplicationSettingsControllerType = ApplicationSettingsController;

/// Type alias for CompanyMasterController (no generics needed)
pub type CompanyMasterControllerType = CompanyMasterController;

/// Type alias for SubsidiaryAccountMasterController (no generics needed)
pub type SubsidiaryAccountMasterControllerType = SubsidiaryAccountMasterController;

/// Type alias for JournalEntryController (no generics needed)
pub type JournalEntryControllerType = JournalEntryController;

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
    pub closing: Arc<ClosingControllerType>,
    pub ledger: Arc<LedgerControllerType>,
    pub search: Arc<SearchControllerType>,
    pub batch_history: Arc<BatchHistoryControllerType>,
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
        closing: Arc<ClosingControllerType>,
        ledger: Arc<LedgerControllerType>,
        search: Arc<SearchControllerType>,
        batch_history: Arc<BatchHistoryControllerType>,
    ) -> Self {
        Self {
            account_master,
            application_settings,
            company_master,
            subsidiary_account_master,
            journal_entry,
            closing,
            ledger,
            search,
            batch_history,
        }
    }
}
