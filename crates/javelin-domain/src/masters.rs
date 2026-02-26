// Masters - マスタドメイン
// 責務: 各種マスタデータの定義

pub mod account_master;
pub mod application_settings;
pub mod company_master;
pub mod subsidiary_account_master;

// 公開インターフェース
pub use account_master::{AccountCode, AccountMaster, AccountName, AccountType};
pub use application_settings::{
    ApplicationSettings, BackupRetentionDays, ClosingDay, DateFormat, DecimalPlaces,
    FiscalYearStartMonth, Language,
};
pub use company_master::{CompanyCode, CompanyMaster, CompanyName};
pub use subsidiary_account_master::{
    SubsidiaryAccountCode, SubsidiaryAccountMaster, SubsidiaryAccountName,
};
