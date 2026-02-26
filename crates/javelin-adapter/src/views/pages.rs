// Pages - ページ単位のビュー
// 責務: 各画面の実装

// Generic components
pub mod menu_page;

// Specific pages
pub mod account_adjustment_execution_page;
pub mod account_adjustment_page;
pub mod account_master_page;
pub mod application_settings_page;
pub mod closing_lock_page;
pub mod closing_page;
pub mod closing_preparation_execution_page;
pub mod closing_preparation_page;
pub mod financial_statement_execution_page;
pub mod financial_statement_page;
pub mod home_page;
pub mod ifrs_valuation_execution_page;
pub mod ifrs_valuation_page;
pub mod journal_entry_form_page;
pub mod ledger_consolidation_execution_page;
pub mod ledger_consolidation_page;
pub mod ledger_detail_page;
pub mod ledger_page;
pub mod note_draft_page;
pub mod search_page;
pub mod subsidiary_account_master_page;

// Generic component exports
// Specific page exports
pub use account_adjustment_execution_page::*;
pub use account_adjustment_page::*;
pub use account_master_page::*;
pub use application_settings_page::*;
pub use closing_lock_page::*;
pub use closing_page::*;
pub use closing_preparation_execution_page::*;
pub use closing_preparation_page::*;
pub use financial_statement_execution_page::*;
pub use financial_statement_page::*;
pub use home_page::*;
pub use ifrs_valuation_execution_page::*;
pub use ifrs_valuation_page::*;
pub use journal_entry_form_page::*;
pub use ledger_consolidation_execution_page::*;
pub use ledger_consolidation_page::*;
pub use ledger_detail_page::*;
pub use ledger_page::*;
pub use menu_page::MenuPage;
pub use note_draft_page::*;
pub use search_page::*;
pub use subsidiary_account_master_page::*;
