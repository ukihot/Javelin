// Masters - マスタドメイン
// 責務: 各種マスタデータの定義
//
// 注意: このモジュールは後方互換性のために残されています。
// 新しいコードでは chart_of_accounts および company モジュールを直接使用してください。

pub mod account_master;
pub mod application_settings;
pub mod company_master;
pub mod events;
pub mod subsidiary_account_master;

// 公開インターフェース - 新しい集約モジュールから再エクスポート
pub use application_settings::{
    ApplicationSettings, BackupRetentionDays, ClosingDay, DateFormat, DecimalPlaces,
    FiscalYearStartMonth, Language,
};
pub use events::AccountMasterEvent;

pub use crate::{
    chart_of_accounts::{
        AccountCode, AccountMaster, AccountName, AccountType, SubsidiaryAccountCode,
        SubsidiaryAccountMaster, SubsidiaryAccountName,
    },
    company::{CompanyCode, CompanyMaster, CompanyName},
};
