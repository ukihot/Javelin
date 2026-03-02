// Page States module - PageState implementations for each screen
// Organized by route hierarchy for better maintainability

// Generic/Shared page states
pub mod stub_page_state;
pub use stub_page_state::StubPageState;

// Top-level pages
pub mod home_page_state;
pub use home_page_state::HomePageState;

// Route hierarchy modules
pub mod closing;
pub mod closing_preparation;
pub mod financial_statements;
pub mod fixed_assets;
pub mod ledger;
pub mod maintenance;
pub mod management_accounting;
pub mod master_management;
pub mod primary_records;

// Legacy individual page state files (to be migrated)
pub mod note_draft_page_state;
pub mod notes_menu_page_state;

// Re-exports for backward compatibility
pub use closing::{
    AccountAdjustmentExecutionPageState, AdjustmentJournalListPageState,
    ClosingLockExecutionPageState, ClosingMenuPageState, ClosingPreparationExecutionPageState,
    ComprehensiveFinancialStatementsPageState, LedgerConsistencyVerificationPageState,
    MaterialityEvaluationPageState, NotesDraftGenerationExecutionPageState,
    PreparationResultPageState, TrialBalanceGenerationExecutionPageState, TrialBalancePageState,
    ValuationResultPageState,
};
pub use financial_statements::{
    FinancialStatementExecutionPageState, FinancialStatementsMenuPageState, NoteDraftPageState,
    NotesMenuPageState,
};
pub use fixed_assets::{
    AssetDetailPageState, AssetRegistrationPageState, DepreciationExecutionPageState,
    DepreciationResultPageState, FixedAssetListPageState, FixedAssetsMenuPageState,
    IfrsValuationExecutionPageState, LeaseContractDetailPageState, LeaseContractListPageState,
    LeaseSchedulePageState, RouAssetListPageState,
};
pub use ledger::{
    AccountDetailPageState, ApDetailPageState, ApLedgerPageState, ArDetailPageState,
    ArLedgerPageState, GeneralLedgerPageState, LedgerAggregationExecutionPageState,
    LedgerMenuPageState,
};
pub use maintenance::{
    CleanEventStorePageState, MaintenanceHomePageState, MaintenanceMenuPageState,
    RebuildProjectionsPageState,
};
pub use management_accounting::ManagementAccountingMenuPageState;
pub use master_management::{
    AccountMasterPageState, ApplicationSettingsPageState, MasterManagementMenuPageState,
    SubsidiaryAccountMasterPageState,
};
pub use primary_records::{
    CashLogInputPageState, CashLogListPageState, DocumentManagementPageState,
    JournalDetailPageState, JournalEntryPageState, JournalListPageState,
    PrimaryRecordsMenuPageState,
};
