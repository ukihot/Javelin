// SystemSettings - システム設定ドメイン

use crate::{error::DomainResult, value_object::ValueObject};

/// システム設定
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemSettings {
    fiscal_year_start_month: FiscalYearStartMonth,
    closing_day: ClosingDay,
    auto_backup_enabled: bool,
    backup_retention_days: BackupRetentionDays,
}

impl SystemSettings {
    pub fn new(
        fiscal_year_start_month: FiscalYearStartMonth,
        closing_day: ClosingDay,
        auto_backup_enabled: bool,
        backup_retention_days: BackupRetentionDays,
    ) -> Self {
        Self { fiscal_year_start_month, closing_day, auto_backup_enabled, backup_retention_days }
    }

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
        self.fiscal_year_start_month.validate()?;
        self.closing_day.validate()?;
        self.backup_retention_days.validate()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fiscal_year_start_month() {
        let month = FiscalYearStartMonth::new(4);
        assert!(month.is_ok());
        assert_eq!(month.unwrap().value(), 4);

        assert!(FiscalYearStartMonth::new(0).is_err());
        assert!(FiscalYearStartMonth::new(13).is_err());
    }

    #[test]
    fn test_closing_day() {
        let day = ClosingDay::new(31);
        assert!(day.is_ok());
        assert_eq!(day.unwrap().value(), 31);

        assert!(ClosingDay::new(0).is_err());
        assert!(ClosingDay::new(32).is_err());
    }

    #[test]
    fn test_backup_retention_days() {
        let days = BackupRetentionDays::new(30);
        assert!(days.is_ok());
        assert_eq!(days.unwrap().value(), 30);

        assert!(BackupRetentionDays::new(0).is_err());
        assert!(BackupRetentionDays::new(3651).is_err());
    }

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            // プロパティ1: 有効範囲内の会計年度開始月は常に作成可能
            #[test]
            fn prop_valid_fiscal_year_start_month(month in 1u8..=12u8) {
                let fiscal_month = FiscalYearStartMonth::new(month);
                prop_assert!(fiscal_month.is_ok());
                prop_assert_eq!(fiscal_month.unwrap().value(), month);
            }

            // プロパティ2: 範囲外の会計年度開始月は常にエラー
            #[test]
            fn prop_invalid_fiscal_year_start_month(month in prop_oneof![
                Just(0u8),
                13u8..=255u8
            ]) {
                let fiscal_month = FiscalYearStartMonth::new(month);
                prop_assert!(fiscal_month.is_err());
            }

            // プロパティ3: 有効範囲内の締日は常に作成可能
            #[test]
            fn prop_valid_closing_day(day in 1u8..=31u8) {
                let closing_day = ClosingDay::new(day);
                prop_assert!(closing_day.is_ok());
                prop_assert_eq!(closing_day.unwrap().value(), day);
            }

            // プロパティ4: 範囲外の締日は常にエラー
            #[test]
            fn prop_invalid_closing_day(day in prop_oneof![
                Just(0u8),
                32u8..=255u8
            ]) {
                let closing_day = ClosingDay::new(day);
                prop_assert!(closing_day.is_err());
            }

            // プロパティ5: 有効範囲内のバックアップ保持日数は常に作成可能
            #[test]
            fn prop_valid_backup_retention_days(days in 1u32..=3650u32) {
                let retention = BackupRetentionDays::new(days);
                prop_assert!(retention.is_ok());
                prop_assert_eq!(retention.unwrap().value(), days);
            }

            // プロパティ6: ゼロのバックアップ保持日数は常にエラー
            #[test]
            fn prop_zero_backup_retention_days_fails(_unit in Just(())) {
                let retention = BackupRetentionDays::new(0);
                prop_assert!(retention.is_err());
            }

            // プロパティ7: 範囲外のバックアップ保持日数は常にエラー
            #[test]
            fn prop_invalid_backup_retention_days(days in 3651u32..=10000u32) {
                let retention = BackupRetentionDays::new(days);
                prop_assert!(retention.is_err());
            }

            // プロパティ8: システム設定の検証
            #[test]
            fn prop_system_settings_validation(
                month in 1u8..=12u8,
                day in 1u8..=31u8,
                days in 1u32..=3650u32,
                auto_backup in any::<bool>()
            ) {
                let fiscal_month = FiscalYearStartMonth::new(month).unwrap();
                let closing_day = ClosingDay::new(day).unwrap();
                let retention = BackupRetentionDays::new(days).unwrap();

                let settings = SystemSettings::new(
                    fiscal_month,
                    closing_day,
                    auto_backup,
                    retention
                );

                prop_assert!(settings.validate().is_ok());
            }
        }
    }
}
