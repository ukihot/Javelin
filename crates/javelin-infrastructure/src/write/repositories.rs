// Write-side: ドメイン層のリポジトリトレイト実装を一括配置 (pure CQRS)

pub mod account_master_repository_impl;
pub mod accounting_period_repository_impl;
pub mod application_settings_repository_impl; // Disabled: ApplicationSettings aggregate removed
pub mod company_master_repository_impl;
pub mod journal_entry_repository_impl;
pub mod subsidiary_account;

pub use account_master_repository_impl::AccountMasterRepositoryImpl;
pub use accounting_period_repository_impl::{
    AccountingPeriodEvent, AccountingPeriodRepositoryImpl,
};
// pub use application_settings_repository_impl::ApplicationSettingsRepositoryImpl; // Disabled
pub use company_master_repository_impl::CompanyMasterRepositoryImpl;
pub use journal_entry_repository_impl::JournalEntryRepositoryImpl;
pub use subsidiary_account::SubsidiaryAccountMasterRepositoryImpl;
