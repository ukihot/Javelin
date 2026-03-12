// TransactionDate - 取引日付値オブジェクト

use std::{fmt, str::FromStr};

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

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
