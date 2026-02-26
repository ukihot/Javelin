// 説明的な値オブジェクト

use std::{fmt, str::FromStr};

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 摘要
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl ValueObject for Description {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount("Description cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl Description {
    pub fn new(text: String) -> DomainResult<Self> {
        let description = Self(text);
        description.validate()?;
        Ok(description)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 取引日付
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionDate(chrono::NaiveDate);

impl ValueObject for TransactionDate {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

impl FromStr for TransactionDate {
    type Err = DomainError;

    fn from_str(date_str: &str) -> Result<Self, Self::Err> {
        let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
            DomainError::InvalidAmount(format!("Invalid date format: {}", date_str))
        })?;
        Self::new(date)
    }
}

impl fmt::Display for TransactionDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl TransactionDate {
    pub fn new(date: chrono::NaiveDate) -> DomainResult<Self> {
        let transaction_date = Self(date);
        transaction_date.validate()?;
        Ok(transaction_date)
    }

    pub fn value(&self) -> chrono::NaiveDate {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description() {
        let desc = Description::new("売上計上".to_string());
        assert!(desc.is_ok());
        assert_eq!(desc.unwrap().value(), "売上計上");

        assert!(Description::new("".to_string()).is_err());
    }

    #[test]
    fn test_transaction_date() {
        let date = chrono::NaiveDate::from_ymd_opt(2024, 3, 31).unwrap();
        let trans_date = TransactionDate::new(date);
        assert!(trans_date.is_ok());
        assert_eq!(trans_date.unwrap().value(), date);
    }

    #[test]
    fn test_transaction_date_from_str() {
        let trans_date = TransactionDate::from_str("2024-03-31");
        assert!(trans_date.is_ok());
        assert_eq!(
            trans_date.unwrap().value(),
            chrono::NaiveDate::from_ymd_opt(2024, 3, 31).unwrap()
        );

        assert!(TransactionDate::from_str("invalid").is_err());
    }

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            // プロパティ1: 非空文字列の摘要は常に作成可能
            #[test]
            fn prop_valid_description(text in "[\\p{Hiragana}\\p{Katakana}\\p{Han}a-zA-Z0-9 ]{1,200}") {
                let description = Description::new(text.clone());
                prop_assert!(description.is_ok());
                let description = description.unwrap();
                prop_assert_eq!(description.value(), text.as_str());
            }

            // プロパティ2: 空文字列の摘要は常にエラー
            #[test]
            fn prop_empty_description_fails(_unit in Just(())) {
                let description = Description::new("".to_string());
                prop_assert!(description.is_err());
            }

            // プロパティ3: 有効な日付は常に作成可能
            #[test]
            fn prop_valid_transaction_date(
                year in 2000i32..=2100i32,
                month in 1u32..=12u32,
                day in 1u32..=28u32  // 28日までは全ての月で有効
            ) {
                if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
                    let trans_date = TransactionDate::new(date);
                    prop_assert!(trans_date.is_ok());
                    prop_assert_eq!(trans_date.unwrap().value(), date);
                }
            }

            // プロパティ4: TransactionDateの順序性
            #[test]
            fn prop_transaction_date_ordering(
                year1 in 2000i32..=2100i32,
                month1 in 1u32..=12u32,
                day1 in 1u32..=28u32,
                year2 in 2000i32..=2100i32,
                month2 in 1u32..=12u32,
                day2 in 1u32..=28u32
            ) {
                if let (Some(date1), Some(date2)) = (
                    chrono::NaiveDate::from_ymd_opt(year1, month1, day1),
                    chrono::NaiveDate::from_ymd_opt(year2, month2, day2)
                ) {
                    let td1 = TransactionDate::new(date1).unwrap();
                    let td2 = TransactionDate::new(date2).unwrap();

                    if date1 < date2 {
                        prop_assert!(td1 < td2);
                    } else if date1 > date2 {
                        prop_assert!(td1 > td2);
                    } else {
                        prop_assert_eq!(td1, td2);
                    }
                }
            }

            // プロパティ5: FromStr往復変換
            #[test]
            fn prop_transaction_date_from_str_roundtrip(
                year in 2000i32..=2100i32,
                month in 1u32..=12u32,
                day in 1u32..=28u32
            ) {
                if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
                    let trans_date = TransactionDate::new(date).unwrap();
                    let date_str = trans_date.to_string();
                    let parsed = TransactionDate::from_str(&date_str);

                    prop_assert!(parsed.is_ok());
                    prop_assert_eq!(parsed.unwrap().value(), date);
                }
            }
        }
    }
}
