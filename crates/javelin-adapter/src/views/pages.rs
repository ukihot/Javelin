// Pages - ページ単位のビュー
// Organized by route hierarchy for better maintainability

// Generic/Shared components
pub mod home_page;
pub mod menu_page;

// Route hierarchy modules
pub mod closing;
pub mod closing_preparation;
pub mod financial_statements;
pub mod fixed_assets;
pub mod master_management;
pub mod primary_records;

// Re-exports for backward compatibility (explicit to avoid ambiguous glob re-exports)
// Closing
pub use closing::{
    ClosingPage, ComprehensiveFinancialStatementsPage, LedgerConsistencyVerificationPage,
    MaterialityEvaluationPage,
};
// Closing Preparation
pub use closing_preparation::ClosingPreparationExecutionPage;
// Financial Statements
pub use financial_statements::{FinancialStatementExecutionPage, NoteDraftPage};
// Fixed Assets
pub use fixed_assets::{AccountAdjustmentExecutionPage, IfrsValuationExecutionPage};
pub use home_page::*;
// Master Management
pub use master_management::{
    AccountMasterPage, ApplicationSettingsPage, SubsidiaryAccountMasterPage,
};
pub use menu_page::MenuPage;
// Primary Records
pub use primary_records::{JournalDetailPage, JournalEntryFormPage, JournalListPage};
