// ApplicationSettings - アプリケーション設定マスタ

use super::company_master::CompanyCode;
use crate::{error::DomainResult, value_object::ValueObject};

/// アプリケーション設定マスタ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationSettings {
    // ユーザ設定
    default_company_code: Option<CompanyCode>,
    language: Language,
    decimal_places: DecimalPlaces,
    date_format: DateFormat,

    // システム設定
    fiscal_year_start_month: FiscalYearStartMonth,
    closing_day: ClosingDay,
    auto_backup_enabled: bool,
    backup_retention_days: BackupRetentionDays,
}

impl ApplicationSettings {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        default_company_code: Option<CompanyCode>,
        language: Language,
        decimal_places: DecimalPlaces,
        date_format: DateFormat,
        fiscal_year_start_month: FiscalYearStartMonth,
        closing_day: ClosingDay,
        auto_backup_enabled: bool,
        backup_retention_days: BackupRetentionDays,
    ) -> Self {
        Self {
            default_company_code,
            language,
            decimal_places,
            date_format,
            fiscal_year_start_month,
            closing_day,
            auto_backup_enabled,
            backup_retention_days,
        }
    }

    // ユーザ設定のゲッター
    pub fn default_company_code(&self) -> Option<&CompanyCode> {
        self.default_company_code.as_ref()
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn decimal_places(&self) -> &DecimalPlaces {
        &self.decimal_places
    }

    pub fn date_format(&self) -> &DateFormat {
        &self.date_format
    }

    // システム設定のゲッター
    pub fn fiscal_year_start_month(&self) -> &FiscalYearStartMonth {
        &self.fiscal_year_start_month
    }

    pub fn closing_day(&self) -> &ClosingDay {
        &self.closing_day
    }

    pub fn auto_backup_enabled(&self) -> bool {
        self.auto_backup_enabled
    }

    pub fn backup_retention_days(&self) -> &BackupRetentionDays {
        &self.backup_retention_days
    }

    // セッター
    pub fn update_default_company_code(&mut self, company_code: Option<CompanyCode>) {
        self.default_company_code = company_code;
    }

    pub fn update_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn update_decimal_places(&mut self, decimal_places: DecimalPlaces) {
        self.decimal_places = decimal_places;
    }

    pub fn update_date_format(&mut self, date_format: DateFormat) {
        self.date_format = date_format;
    }

    pub fn update_fiscal_year_start_month(&mut self, month: FiscalYearStartMonth) {
        self.fiscal_year_start_month = month;
    }

    pub fn update_closing_day(&mut self, day: ClosingDay) {
        self.closing_day = day;
    }

    pub fn update_auto_backup_enabled(&mut self, enabled: bool) {
        self.auto_backup_enabled = enabled;
    }

    pub fn update_backup_retention_days(&mut self, days: BackupRetentionDays) {
        self.backup_retention_days = days;
    }

    pub fn validate(&self) -> DomainResult<()> {
        if let Some(company_code) = &self.default_company_code {
            company_code.validate()?;
        }
        self.language.validate()?;
        self.decimal_places.validate()?;
        self.date_format.validate()?;
        self.fiscal_year_start_month.validate()?;
        self.closing_day.validate()?;
        self.backup_retention_days.validate()?;
        Ok(())
    }
}

/// 言語設定
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language(String);

impl Language {
    pub fn new(language: impl Into<String>) -> DomainResult<Self> {
        let language = language.into();
        if language.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "言語設定は空にできません".to_string(),
            ));
        }
        Ok(Self(language))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for Language {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "言語設定は空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

/// 小数点以下桁数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecimalPlaces(u8);

impl DecimalPlaces {
    pub fn new(decimal_places: u8) -> DomainResult<Self> {
        if decimal_places > 6 {
            return Err(crate::error::DomainError::ValidationError(
                "小数点以下桁数は0〜6の範囲で指定してください".to_string(),
            ));
        }
        Ok(Self(decimal_places))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl ValueObject for DecimalPlaces {
    fn validate(&self) -> DomainResult<()> {
        if self.0 > 6 {
            return Err(crate::error::DomainError::ValidationError(
                "小数点以下桁数は0〜6の範囲で指定してください".to_string(),
            ));
        }
        Ok(())
    }
}

/// 日付フォーマット
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateFormat(String);

impl DateFormat {
    pub fn new(format: impl Into<String>) -> DomainResult<Self> {
        let format = format.into();
        if format.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "日付フォーマットは空にできません".to_string(),
            ));
        }
        Ok(Self(format))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for DateFormat {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "日付フォーマットは空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

/// 会計年度開始月
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FiscalYearStartMonth(u8);

impl FiscalYearStartMonth {
    pub fn new(month: u8) -> DomainResult<Self> {
        if !(1..=12).contains(&month) {
            return Err(crate::error::DomainError::ValidationError(
                "会計年度開始月は1〜12の範囲で指定してください".to_string(),
            ));
        }
        Ok(Self(month))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl ValueObject for FiscalYearStartMonth {
    fn validate(&self) -> DomainResult<()> {
        if self.0 < 1 || self.0 > 12 {
            return Err(crate::error::DomainError::ValidationError(
                "会計年度開始月は1〜12の範囲で指定してください".to_string(),
            ));
        }
        Ok(())
    }
}

/// 締日
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosingDay(u8);

impl ClosingDay {
    pub fn new(day: u8) -> DomainResult<Self> {
        if !(1..=31).contains(&day) {
            return Err(crate::error::DomainError::ValidationError(
                "締日は1〜31の範囲で指定してください".to_string(),
            ));
        }
        Ok(Self(day))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl ValueObject for ClosingDay {
    fn validate(&self) -> DomainResult<()> {
        if self.0 < 1 || self.0 > 31 {
            return Err(crate::error::DomainError::ValidationError(
                "締日は1〜31の範囲で指定してください".to_string(),
            ));
        }
        Ok(())
    }
}

/// バックアップ保持日数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupRetentionDays(u32);

impl BackupRetentionDays {
    pub fn new(days: u32) -> DomainResult<Self> {
        if days == 0 {
            return Err(crate::error::DomainError::ValidationError(
                "バックアップ保持日数は1日以上を指定してください".to_string(),
            ));
        }
        if days > 3650 {
            return Err(crate::error::DomainError::ValidationError(
                "バックアップ保持日数は10年(3650日)以内を指定してください".to_string(),
            ));
        }
        Ok(Self(days))
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl ValueObject for BackupRetentionDays {
    fn validate(&self) -> DomainResult<()> {
        if self.0 == 0 {
            return Err(crate::error::DomainError::ValidationError(
                "バックアップ保持日数は1日以上を指定してください".to_string(),
            ));
        }
        if self.0 > 3650 {
            return Err(crate::error::DomainError::ValidationError(
                "バックアップ保持日数は10年(3650日)以内を指定してください".to_string(),
            ));
        }
        Ok(())
    }
}
