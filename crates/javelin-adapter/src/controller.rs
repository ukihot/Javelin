// Controller - 外部入力受付
// 責務: DTO変換、InputPort呼び出し
// 禁止: 業務判断

pub mod account_master_controller;
pub mod adjust_accounts_controller;
pub mod application_settings_controller;
pub mod apply_ifrs_valuation_controller;
pub mod batch_history_controller;
pub mod company_master_controller;
pub mod consolidate_ledger_controller;
pub mod evaluate_materiality_controller;
pub mod generate_comprehensive_financial_statements_controller;
pub mod generate_financial_statements_controller;
pub mod generate_note_draft_controller;
pub mod generate_trial_balance_controller;
pub mod invoice_print_controller;
pub mod journal_detail_controller;
pub mod journal_entry_controller;
pub mod ledger_controller;
pub mod lock_closing_period_controller;
pub mod prepare_closing_controller;
pub mod search_controller;
pub mod subsidiary_account_master_controller;
pub mod verify_ledger_consistency_controller;

pub use account_master_controller::AccountMasterController;
pub use adjust_accounts_controller::AdjustAccountsController;
pub use application_settings_controller::ApplicationSettingsController;
pub use apply_ifrs_valuation_controller::ApplyIfrsValuationController;
pub use batch_history_controller::BatchHistoryController;
pub use company_master_controller::CompanyMasterController;
pub use consolidate_ledger_controller::ConsolidateLedgerController;
pub use evaluate_materiality_controller::EvaluateMaterialityController;
pub use generate_comprehensive_financial_statements_controller::GenerateComprehensiveFinancialStatementsController;
pub use generate_financial_statements_controller::GenerateFinancialStatementsController;
pub use generate_note_draft_controller::GenerateNoteDraftController;
pub use generate_trial_balance_controller::GenerateTrialBalanceController;
pub use invoice_print_controller::InvoicePrintController;
// Re-export application layer DTOs for convenience
pub use javelin_application::dtos::{
    request::{
        LoadAccountMasterRequest, LoadApplicationSettingsRequest, LoadCompanyMasterRequest,
        LoadSubsidiaryAccountMasterRequest,
    },
    response::{
        AccountMasterItem, CompanyMasterItem, LoadAccountMasterResponse,
        LoadApplicationSettingsResponse, LoadCompanyMasterResponse,
        LoadSubsidiaryAccountMasterResponse, SubsidiaryAccountMasterItem, SystemSettingsDto,
        UserOptionsDto,
    },
};
pub use journal_detail_controller::JournalDetailController;
pub use journal_entry_controller::JournalEntryController;
pub use ledger_controller::LedgerController;
pub use lock_closing_period_controller::LockClosingPeriodController;
pub use prepare_closing_controller::PrepareClosingController;
pub use search_controller::SearchController;
pub use subsidiary_account_master_controller::SubsidiaryAccountMasterController;
pub use verify_ledger_consistency_controller::VerifyLedgerConsistencyController;
