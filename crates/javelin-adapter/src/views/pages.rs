// Pages - ページ単位のビュー
// Organized by route hierarchy for better maintainability

// Generic/Shared components
pub mod home_page;
pub mod menu_page;

// Route hierarchy modules
pub mod billing;
pub mod closing;
pub mod closing_preparation;
pub mod financial_statements;
pub mod fixed_assets;
pub mod ledger;
pub mod maintenance;
pub mod management_accounting;
pub mod master_management;
pub mod primary_records;

// Re-exports for backward compatibility (explicit to avoid ambiguous glob re-exports)
// Billing
pub use billing::InvoicePrintPage;
// Closing
pub use closing::{
    AdjustmentJournalListPage, ClosingPage, ComprehensiveFinancialStatementsPage,
    LedgerConsistencyVerificationPage, MaterialityEvaluationPage, ValuationResultPage,
};
// Closing Preparation
pub use closing_preparation::ClosingPreparationExecutionPage;
// Financial Statements
pub use financial_statements::{FinancialStatementExecutionPage, NoteDraftPage};
// Fixed Assets
pub use fixed_assets::{
    AccountAdjustmentExecutionPage, DepreciationResultPage, FixedAssetListPage,
    IfrsValuationExecutionPage, LeaseContractListPage, LeaseSchedulePage, RouAssetListPage,
};
pub use home_page::*;
// Ledger
pub use ledger::{
    AccountDetailPage, ApDetailPage, ApLedgerPage, ArDetailPage, ArLedgerPage, GeneralLedgerPage,
};
// Master Management
pub use master_management::{
    AccountMasterPage, ApplicationSettingsPage, SubsidiaryAccountMasterPage,
};
pub use menu_page::MenuPage;
// Primary Records
pub use primary_records::{
    CashLogListPage, DocumentManagementPage, JournalDetailPage, JournalEntryFormPage,
    JournalListPage,
};
