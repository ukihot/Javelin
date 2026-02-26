// 法人関連の値オブジェクト

use chrono::NaiveDate;

use crate::{
    entity::EntityId,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 法人ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanyId(String);

impl EntityId for CompanyId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl CompanyId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

/// 法人名
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanyName(String);

impl ValueObject for CompanyName {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount("Company name cannot be empty".to_string()));
        }
        if self.0.len() > 200 {
            return Err(DomainError::InvalidAmount("Company name is too long".to_string()));
        }
        Ok(())
    }
}

impl CompanyName {
    pub fn new(name: String) -> DomainResult<Self> {
        let company_name = Self(name);
        company_name.validate()?;
        Ok(company_name)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 法人名カナ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanyNameKana(String);

impl ValueObject for CompanyNameKana {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount(
                "Company name kana cannot be empty".to_string(),
            ));
        }
        if self.0.len() > 200 {
            return Err(DomainError::InvalidAmount("Company name kana is too long".to_string()));
        }
        Ok(())
    }
}

impl CompanyNameKana {
    pub fn new(name_kana: String) -> DomainResult<Self> {
        let company_name_kana = Self(name_kana);
        company_name_kana.validate()?;
        Ok(company_name_kana)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 代表者名
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentativeName(String);

impl ValueObject for RepresentativeName {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount(
                "Representative name cannot be empty".to_string(),
            ));
        }
        if self.0.len() > 100 {
            return Err(DomainError::InvalidAmount("Representative name is too long".to_string()));
        }
        Ok(())
    }
}

impl RepresentativeName {
    pub fn new(name: String) -> DomainResult<Self> {
        let representative_name = Self(name);
        representative_name.validate()?;
        Ok(representative_name)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 代表者名カナ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentativeNameKana(String);

impl ValueObject for RepresentativeNameKana {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount(
                "Representative name kana cannot be empty".to_string(),
            ));
        }
        if self.0.len() > 100 {
            return Err(DomainError::InvalidAmount(
                "Representative name kana is too long".to_string(),
            ));
        }
        Ok(())
    }
}

impl RepresentativeNameKana {
    pub fn new(name_kana: String) -> DomainResult<Self> {
        let representative_name_kana = Self(name_kana);
        representative_name_kana.validate()?;
        Ok(representative_name_kana)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 代表者役職
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentativeTitle(String);

impl ValueObject for RepresentativeTitle {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount(
                "Representative title cannot be empty".to_string(),
            ));
        }
        if self.0.len() > 50 {
            return Err(DomainError::InvalidAmount("Representative title is too long".to_string()));
        }
        Ok(())
    }
}

impl RepresentativeTitle {
    pub fn new(title: String) -> DomainResult<Self> {
        let representative_title = Self(title);
        representative_title.validate()?;
        Ok(representative_title)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 決算日（月日）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FiscalYearEnd {
    month: u8,
    day: u8,
}

impl ValueObject for FiscalYearEnd {
    fn validate(&self) -> DomainResult<()> {
        if self.month < 1 || self.month > 12 {
            return Err(DomainError::InvalidAccountingPeriod);
        }
        if self.day < 1 || self.day > 31 {
            return Err(DomainError::InvalidAmount("Day must be between 1 and 31".to_string()));
        }
        // 月ごとの日数チェック（簡易版）
        let max_day = match self.month {
            2 => 29, // 閏年を考慮して29日まで許可
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };
        if self.day > max_day {
            return Err(DomainError::InvalidAmount(format!(
                "Invalid day {} for month {}",
                self.day, self.month
            )));
        }
        Ok(())
    }
}

impl FiscalYearEnd {
    pub fn new(month: u8, day: u8) -> DomainResult<Self> {
        let fiscal_year_end = Self { month, day };
        fiscal_year_end.validate()?;
        Ok(fiscal_year_end)
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    /// 指定された年の決算日を取得
    pub fn to_date(&self, year: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year as i32, self.month as u32, self.day as u32)
            .expect("Valid fiscal year end date")
    }
}

/// 締め周期
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosingCycle {
    /// 月次
    Monthly,
    /// 四半期
    Quarterly,
    /// 半期
    SemiAnnual,
    /// 年次
    Annual,
}

impl ClosingCycle {
    /// 締め周期の月数を取得
    pub fn months(&self) -> u8 {
        match self {
            ClosingCycle::Monthly => 1,
            ClosingCycle::Quarterly => 3,
            ClosingCycle::SemiAnnual => 6,
            ClosingCycle::Annual => 12,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_name() {
        let name = CompanyName::new("株式会社テスト".to_string());
        assert!(name.is_ok());
        assert_eq!(name.unwrap().value(), "株式会社テスト");

        let empty = CompanyName::new("".to_string());
        assert!(empty.is_err());
    }

    #[test]
    fn test_fiscal_year_end() {
        let end = FiscalYearEnd::new(3, 31);
        assert!(end.is_ok());
        let end = end.unwrap();
        assert_eq!(end.month(), 3);
        assert_eq!(end.day(), 31);

        // 無効な月
        assert!(FiscalYearEnd::new(0, 31).is_err());
        assert!(FiscalYearEnd::new(13, 31).is_err());

        // 無効な日
        assert!(FiscalYearEnd::new(2, 30).is_err());
        assert!(FiscalYearEnd::new(4, 31).is_err());
    }

    #[test]
    fn test_fiscal_year_end_to_date() {
        let end = FiscalYearEnd::new(3, 31).unwrap();
        let date = end.to_date(2024);
        assert_eq!(date, NaiveDate::from_ymd_opt(2024, 3, 31).unwrap());
    }

    #[test]
    fn test_closing_cycle() {
        assert_eq!(ClosingCycle::Monthly.months(), 1);
        assert_eq!(ClosingCycle::Quarterly.months(), 3);
        assert_eq!(ClosingCycle::SemiAnnual.months(), 6);
        assert_eq!(ClosingCycle::Annual.months(), 12);
    }

    // Property-based tests
    mod property_tests {
        use chrono::Datelike;
        use proptest::prelude::*;

        use super::*;

        // 文字列生成戦略（非空、指定バイト長以下）
        // 注意: Rustの文字列長チェックはバイト数で行われるため、
        // 日本語文字（3バイト）を考慮した生成が必要
        fn non_empty_string(max_bytes: usize) -> impl Strategy<Value = String> {
            // ASCII文字のみを使用して、バイト長の問題を回避
            prop::string::string_regex(&format!("[a-zA-Z0-9 ]{{1,{}}}", max_bytes)).unwrap()
        }

        proptest! {
            // プロパティ1: 有効な法人名は常に作成可能
            #[test]
            fn prop_valid_company_name(name in non_empty_string(200)) {
                let company_name = CompanyName::new(name.clone());
                prop_assert!(company_name.is_ok());
                let company_name = company_name.unwrap();
                prop_assert_eq!(company_name.value(), name.as_str());
            }

            // プロパティ2: 空文字列の法人名は常にエラー
            #[test]
            fn prop_empty_company_name_fails(_unit in Just(())) {
                let company_name = CompanyName::new("".to_string());
                prop_assert!(company_name.is_err());
            }

            // プロパティ3: 長すぎる法人名は常にエラー
            #[test]
            fn prop_too_long_company_name_fails(name in prop::string::string_regex(".{201,300}").unwrap()) {
                let company_name = CompanyName::new(name);
                prop_assert!(company_name.is_err());
            }

            // プロパティ4: 有効な代表者名は常に作成可能
            #[test]
            fn prop_valid_representative_name(name in non_empty_string(100)) {
                let rep_name = RepresentativeName::new(name.clone());
                prop_assert!(rep_name.is_ok());
                let rep_name = rep_name.unwrap();
                prop_assert_eq!(rep_name.value(), name.as_str());
            }

            // プロパティ5: 有効な代表者役職は常に作成可能
            #[test]
            fn prop_valid_representative_title(title in non_empty_string(50)) {
                let rep_title = RepresentativeTitle::new(title.clone());
                prop_assert!(rep_title.is_ok());
                let rep_title = rep_title.unwrap();
                prop_assert_eq!(rep_title.value(), title.as_str());
            }

            // プロパティ6: 決算日の月の妥当性
            #[test]
            fn prop_fiscal_year_end_valid_month(month in 1u8..=12u8, day in 1u8..=28u8) {
                // 28日までは全ての月で有効
                let end = FiscalYearEnd::new(month, day);
                prop_assert!(end.is_ok());
                let end = end.unwrap();
                prop_assert_eq!(end.month(), month);
                prop_assert_eq!(end.day(), day);
            }

            // プロパティ7: 無効な月は常にエラー
            #[test]
            fn prop_fiscal_year_end_invalid_month(month in prop_oneof![
                Just(0u8),
                13u8..=255u8
            ], day in 1u8..=31u8) {
                let end = FiscalYearEnd::new(month, day);
                prop_assert!(end.is_err());
            }

            // プロパティ8: 31日制の月
            #[test]
            fn prop_fiscal_year_end_31_day_months(
                month in prop_oneof![Just(1u8), Just(3u8), Just(5u8), Just(7u8), Just(8u8), Just(10u8), Just(12u8)],
                day in 1u8..=31u8
            ) {
                let end = FiscalYearEnd::new(month, day);
                prop_assert!(end.is_ok());
            }

            // プロパティ9: 30日制の月
            #[test]
            fn prop_fiscal_year_end_30_day_months(
                month in prop_oneof![Just(4u8), Just(6u8), Just(9u8), Just(11u8)],
                day in 1u8..=30u8
            ) {
                let end = FiscalYearEnd::new(month, day);
                prop_assert!(end.is_ok());
            }

            // プロパティ10: 2月は29日まで
            #[test]
            fn prop_fiscal_year_end_february(day in 1u8..=29u8) {
                let end = FiscalYearEnd::new(2, day);
                prop_assert!(end.is_ok());
            }

            // プロパティ11: 2月30日以降は無効
            #[test]
            fn prop_fiscal_year_end_february_invalid(day in 30u8..=31u8) {
                let end = FiscalYearEnd::new(2, day);
                prop_assert!(end.is_err());
            }

            // プロパティ12: 30日制の月の31日は無効
            #[test]
            fn prop_fiscal_year_end_30_day_months_invalid(
                month in prop_oneof![Just(4u8), Just(6u8), Just(9u8), Just(11u8)]
            ) {
                let end = FiscalYearEnd::new(month, 31);
                prop_assert!(end.is_err());
            }

            // プロパティ13: to_dateの一貫性
            #[test]
            fn prop_fiscal_year_end_to_date_consistency(
                month in 1u8..=12u8,
                day in 1u8..=28u8,
                year in 2000u32..=2100u32
            ) {
                let end = FiscalYearEnd::new(month, day).unwrap();
                let date = end.to_date(year);
                prop_assert_eq!(date.month() as u8, month);
                prop_assert_eq!(date.day() as u8, day);
                prop_assert_eq!(date.year() as u32, year);
            }

            // プロパティ14: ClosingCycleの月数の妥当性
            #[test]
            fn prop_closing_cycle_months(_unit in Just(())) {
                prop_assert_eq!(ClosingCycle::Monthly.months(), 1);
                prop_assert_eq!(ClosingCycle::Quarterly.months(), 3);
                prop_assert_eq!(ClosingCycle::SemiAnnual.months(), 6);
                prop_assert_eq!(ClosingCycle::Annual.months(), 12);
            }
        }
    }
}
