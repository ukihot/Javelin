// MasterDataLoader - マスタデータとユーザ設定のロード
// 責務: アプリケーション起動時の初期データロード
// 禁止: Repository利用（Projectionのみ）

use javelin_domain::masters::{
    AccountCode, AccountMaster as DomainAccountMaster, AccountName,
    AccountType as DomainAccountType, ApplicationSettings as DomainApplicationSettings,
    BackupRetentionDays, ClosingDay, CompanyCode, CompanyMaster as DomainCompanyMaster,
    CompanyName, DateFormat, DecimalPlaces, FiscalYearStartMonth, Language,
};
use serde::{Deserialize, Serialize};

use crate::error::ApplicationResult;

/// マスタデータとユーザ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterData {
    /// 勘定科目マスタ
    pub accounts: Vec<AccountMaster>,
    /// 会社マスタ
    pub companies: Vec<CompanyMaster>,
    /// ユーザ設定
    pub user_options: UserOptions,
    /// システム設定
    pub system_settings: SystemSettings,
}

/// 勘定科目マスタ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountMaster {
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub is_active: bool,
}

/// 勘定科目タイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

/// 会社マスタ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyMaster {
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

/// ユーザ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOptions {
    /// デフォルト会社コード
    pub default_company_code: Option<String>,
    /// 表示言語
    pub language: String,
    /// 小数点以下桁数
    pub decimal_places: u8,
    /// 日付フォーマット
    pub date_format: String,
}

/// システム設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettings {
    /// 会計年度開始月
    pub fiscal_year_start_month: u8,
    /// 締日
    pub closing_day: u8,
    /// 自動バックアップ有効
    pub auto_backup_enabled: bool,
    /// バックアップ保持日数
    pub backup_retention_days: u32,
}

impl Default for UserOptions {
    fn default() -> Self {
        Self {
            default_company_code: None,
            language: "ja".to_string(),
            decimal_places: 2,
            date_format: "YYYY-MM-DD".to_string(),
        }
    }
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            fiscal_year_start_month: 4, // 4月開始
            closing_day: 31,            // 月末締め
            auto_backup_enabled: true,
            backup_retention_days: 90,
        }
    }
}

/// マスタデータローダークエリ
#[derive(Debug, Clone)]
pub struct LoadMasterDataQuery;

/// マスタデータローダーサービス
#[allow(async_fn_in_trait)]
pub trait MasterDataLoaderService: Send + Sync {
    /// マスタデータをロード
    async fn load_master_data(&self) -> ApplicationResult<MasterData>;
}

// ドメインオブジェクトからアプリケーションDTOへの変換
impl From<&DomainAccountMaster> for AccountMaster {
    fn from(domain: &DomainAccountMaster) -> Self {
        Self {
            code: domain.code().value().to_string(),
            name: domain.name().value().to_string(),
            account_type: match domain.account_type() {
                DomainAccountType::Asset => AccountType::Asset,
                DomainAccountType::Liability => AccountType::Liability,
                DomainAccountType::Equity => AccountType::Equity,
                DomainAccountType::Revenue => AccountType::Revenue,
                DomainAccountType::Expense => AccountType::Expense,
            },
            is_active: domain.is_active(),
        }
    }
}

impl From<&DomainCompanyMaster> for CompanyMaster {
    fn from(domain: &DomainCompanyMaster) -> Self {
        Self {
            code: domain.code().value().to_string(),
            name: domain.name().value().to_string(),
            is_active: domain.is_active(),
        }
    }
}

impl From<&DomainApplicationSettings> for UserOptions {
    fn from(domain: &DomainApplicationSettings) -> Self {
        Self {
            default_company_code: domain.default_company_code().map(|c| c.value().to_string()),
            language: domain.language().value().to_string(),
            decimal_places: domain.decimal_places().value(),
            date_format: domain.date_format().value().to_string(),
        }
    }
}

impl From<&DomainApplicationSettings> for SystemSettings {
    fn from(domain: &DomainApplicationSettings) -> Self {
        Self {
            fiscal_year_start_month: domain.fiscal_year_start_month().value(),
            closing_day: domain.closing_day().value(),
            auto_backup_enabled: domain.auto_backup_enabled(),
            backup_retention_days: domain.backup_retention_days().value(),
        }
    }
}

// アプリケーションDTOからドメインオブジェクトへの変換
impl TryFrom<&AccountMaster> for DomainAccountMaster {
    type Error = crate::error::ApplicationError;

    fn try_from(dto: &AccountMaster) -> Result<Self, Self::Error> {
        let code = AccountCode::new(&dto.code)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
        let name = AccountName::new(&dto.name)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        let account_type = match dto.account_type {
            AccountType::Asset => DomainAccountType::Asset,
            AccountType::Liability => DomainAccountType::Liability,
            AccountType::Equity => DomainAccountType::Equity,
            AccountType::Revenue => DomainAccountType::Revenue,
            AccountType::Expense => DomainAccountType::Expense,
        };

        Ok(Self::new(code, name, account_type, dto.is_active))
    }
}

impl TryFrom<&CompanyMaster> for DomainCompanyMaster {
    type Error = crate::error::ApplicationError;

    fn try_from(dto: &CompanyMaster) -> Result<Self, Self::Error> {
        let code = CompanyCode::new(&dto.code)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
        let name = CompanyName::new(&dto.name)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        Ok(Self::new(code, name, dto.is_active))
    }
}

// Helper function to convert DTOs to domain object
pub fn to_application_settings(
    user_opts: &UserOptions,
    sys_settings: &SystemSettings,
) -> Result<DomainApplicationSettings, crate::error::ApplicationError> {
    let default_company_code = user_opts
        .default_company_code
        .as_ref()
        .map(|code| {
            CompanyCode::new(code)
                .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))
        })
        .transpose()?;

    let language = Language::new(&user_opts.language)
        .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
    let decimal_places = DecimalPlaces::new(user_opts.decimal_places)
        .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
    let date_format = DateFormat::new(&user_opts.date_format)
        .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

    let fiscal_year_start_month =
        FiscalYearStartMonth::new(sys_settings.fiscal_year_start_month)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
    let closing_day = ClosingDay::new(sys_settings.closing_day)
        .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
    let backup_retention_days = BackupRetentionDays::new(sys_settings.backup_retention_days)
        .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

    Ok(DomainApplicationSettings::new(
        default_company_code,
        language,
        decimal_places,
        date_format,
        fiscal_year_start_month,
        closing_day,
        sys_settings.auto_backup_enabled,
        backup_retention_days,
    ))
}
