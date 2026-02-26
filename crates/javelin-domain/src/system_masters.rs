// System Masters - システムマスタドメイン
// 責務: システム設定、コードマスタ、ユーザ設定などの定義
// 禁止: アプリケーション層のDTO定義

pub mod account_master;
pub mod company_master;
pub mod service;
pub mod system_master;
pub mod system_settings;
pub mod user_settings;

// 公開インターフェース
pub use account_master::{AccountCode, AccountMaster, AccountName, AccountType};
pub use company_master::{CompanyCode, CompanyMaster, CompanyName};
pub use service::SystemMasterService;
pub use system_master::{SystemMaster, SystemMasterId};
pub use system_settings::{BackupRetentionDays, ClosingDay, FiscalYearStartMonth, SystemSettings};
pub use user_settings::{DateFormat, DecimalPlaces, Language, UserSettings};
