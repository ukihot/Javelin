// 会計期間関連の値オブジェクト

use chrono::{NaiveDate, NaiveDateTime};

use crate::{
    entity::EntityId,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 会計年度
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FiscalYear(u32);

impl ValueObject for FiscalYear {
    fn validate(&self) -> DomainResult<()> {
        if self.0 < 1900 || self.0 > 2100 {
            return Err(DomainError::InvalidAmount(
                "Fiscal year must be between 1900 and 2100".to_string(),
            ));
        }
        Ok(())
    }
}

impl FiscalYear {
    pub fn new(year: u32) -> DomainResult<Self> {
        let fiscal_year = Self(year);
        fiscal_year.validate()?;
        Ok(fiscal_year)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

/// 会計期間（月）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Period(u8);

impl ValueObject for Period {
    fn validate(&self) -> DomainResult<()> {
        if self.0 < 1 || self.0 > 12 {
            return Err(DomainError::InvalidAccountingPeriod);
        }
        Ok(())
    }
}

impl Period {
    pub fn new(month: u8) -> DomainResult<Self> {
        let period = Self(month);
        period.validate()?;
        Ok(period)
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

/// 期間ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeriodId(String);

impl EntityId for PeriodId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl PeriodId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// 会計年度と期間から期間IDを生成
    pub fn from_year_period(year: FiscalYear, period: Period) -> Self {
        Self(format!("{}-{:02}", year.value(), period.value()))
    }
}

/// 日付
pub type Date = NaiveDate;

/// 日時
pub type DateTime = NaiveDateTime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fiscal_year_valid() {
        let year = FiscalYear::new(2024);
        assert!(year.is_ok());
        assert_eq!(year.unwrap().value(), 2024);
    }

    #[test]
    fn test_fiscal_year_invalid() {
        assert!(FiscalYear::new(1899).is_err());
        assert!(FiscalYear::new(2101).is_err());
    }

    #[test]
    fn test_period_valid() {
        let period = Period::new(1);
        assert!(period.is_ok());
        assert_eq!(period.unwrap().value(), 1);

        let period = Period::new(12);
        assert!(period.is_ok());
        assert_eq!(period.unwrap().value(), 12);
    }

    #[test]
    fn test_period_invalid() {
        assert!(Period::new(0).is_err());
        assert!(Period::new(13).is_err());
    }

    #[test]
    fn test_period_id_from_year_period() {
        let year = FiscalYear::new(2024).unwrap();
        let period = Period::new(3).unwrap();
        let id = PeriodId::from_year_period(year, period);
        assert_eq!(id.value(), "2024-03");
    }

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            // プロパティ1: 有効範囲内の会計年度は常に作成可能
            #[test]
            fn prop_valid_fiscal_year_creation(year in 1900u32..=2100u32) {
                let fiscal_year = FiscalYear::new(year);
                prop_assert!(fiscal_year.is_ok());
                prop_assert_eq!(fiscal_year.unwrap().value(), year);
            }

            // プロパティ2: 範囲外の会計年度は常にエラー
            #[test]
            fn prop_invalid_fiscal_year_fails(year in prop_oneof![
                0u32..1900u32,
                2101u32..10000u32
            ]) {
                let fiscal_year = FiscalYear::new(year);
                prop_assert!(fiscal_year.is_err());
            }

            // プロパティ3: 有効範囲内の期間は常に作成可能
            #[test]
            fn prop_valid_period_creation(month in 1u8..=12u8) {
                let period = Period::new(month);
                prop_assert!(period.is_ok());
                prop_assert_eq!(period.unwrap().value(), month);
            }

            // プロパティ4: 範囲外の期間は常にエラー
            #[test]
            fn prop_invalid_period_fails(month in prop_oneof![
                Just(0u8),
                13u8..=255u8
            ]) {
                let period = Period::new(month);
                prop_assert!(period.is_err());
            }

            // プロパティ5: PeriodIDのフォーマット検証
            #[test]
            fn prop_period_id_format(
                year in 1900u32..=2100u32,
                month in 1u8..=12u8
            ) {
                let fiscal_year = FiscalYear::new(year).unwrap();
                let period = Period::new(month).unwrap();
                let id = PeriodId::from_year_period(fiscal_year, period);

                let expected = format!("{}-{:02}", year, month);
                prop_assert_eq!(id.value(), expected.as_str());
            }

            // プロパティ6: FiscalYearの順序性
            #[test]
            fn prop_fiscal_year_ordering(
                year1 in 1900u32..=2100u32,
                year2 in 1900u32..=2100u32
            ) {
                let fy1 = FiscalYear::new(year1).unwrap();
                let fy2 = FiscalYear::new(year2).unwrap();

                if year1 < year2 {
                    prop_assert!(fy1 < fy2);
                } else if year1 > year2 {
                    prop_assert!(fy1 > fy2);
                } else {
                    prop_assert_eq!(fy1, fy2);
                }
            }

            // プロパティ7: Periodの順序性
            #[test]
            fn prop_period_ordering(
                month1 in 1u8..=12u8,
                month2 in 1u8..=12u8
            ) {
                let p1 = Period::new(month1).unwrap();
                let p2 = Period::new(month2).unwrap();

                if month1 < month2 {
                    prop_assert!(p1 < p2);
                } else if month1 > month2 {
                    prop_assert!(p1 > p2);
                } else {
                    prop_assert_eq!(p1, p2);
                }
            }
        }
    }
}
