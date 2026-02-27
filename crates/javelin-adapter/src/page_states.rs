// Page States module - PageState implementations for each screen
// Each screen has its own PageState that manages state and channels independently

// Generic page states
pub mod stub_page_state;

// Menu page states
pub mod closing_menu_page_state;
pub mod financial_statements_menu_page_state;
pub mod fixed_assets_menu_page_state;
pub mod ledger_menu_page_state;
pub mod management_accounting_menu_page_state;
pub mod master_management_menu_page_state;
pub mod notes_menu_page_state;
pub mod primary_records_menu_page_state;

// Current page states
pub mod account_adjustment_execution_page_state;
pub mod account_master_page_state;
pub mod application_settings_page_state;
pub mod closing_preparation_execution_page_state;
pub mod financial_statement_execution_page_state;
pub mod home_page_state;
pub mod ifrs_valuation_execution_page_state;
pub mod journal_entry_page_state;
pub mod maintenance_home_page_state;
pub mod maintenance_menu_page_state;
pub mod note_draft_page_state;
pub mod subsidiary_account_master_page_state;
pub mod trial_balance_page_state;

// Generic page state exports
// Legacy page state exports
pub use account_adjustment_execution_page_state::AccountAdjustmentExecutionPageState;
pub use account_master_page_state::AccountMasterPageState;
pub use application_settings_page_state::ApplicationSettingsPageState;
pub use closing_menu_page_state::ClosingMenuPageState;
pub use closing_preparation_execution_page_state::ClosingPreparationExecutionPageState;
pub use financial_statement_execution_page_state::FinancialStatementExecutionPageState;
pub use financial_statements_menu_page_state::FinancialStatementsMenuPageState;
pub use fixed_assets_menu_page_state::FixedAssetsMenuPageState;
pub use home_page_state::HomePageState;
pub use ifrs_valuation_execution_page_state::IfrsValuationExecutionPageState;
pub use journal_entry_page_state::JournalEntryPageState;
pub use ledger_menu_page_state::LedgerMenuPageState;
pub use maintenance_home_page_state::MaintenanceHomePageState;
pub use maintenance_menu_page_state::MaintenanceMenuPageState;
pub use management_accounting_menu_page_state::ManagementAccountingMenuPageState;
pub use master_management_menu_page_state::MasterManagementMenuPageState;
pub use note_draft_page_state::NoteDraftPageState;
pub use notes_menu_page_state::NotesMenuPageState;
// Menu page state exports
pub use primary_records_menu_page_state::PrimaryRecordsMenuPageState;
pub use stub_page_state::StubPageState;
pub use subsidiary_account_master_page_state::SubsidiaryAccountMasterPageState;
pub use trial_balance_page_state::TrialBalancePageState;
