// Controller - 外部入力受付
// 責務: DTO変換、InputPort呼び出し
// 禁止: 業務判断

pub mod account_master_controller;
pub mod application_settings_controller;
pub mod batch_history_controller;
pub mod closing_controller;
pub mod company_master_controller;
pub mod journal_detail_controller;
pub mod journal_entry_controller;
pub mod ledger_controller;
pub mod search_controller;
pub mod subsidiary_account_master_controller;

pub use account_master_controller::AccountMasterController;
pub use application_settings_controller::ApplicationSettingsController;
pub use batch_history_controller::BatchHistoryController;
pub use closing_controller::ClosingController;
pub use company_master_controller::CompanyMasterController;
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
pub use search_controller::SearchController;
pub use subsidiary_account_master_controller::SubsidiaryAccountMasterController;
