// 月次決算確報ドメインモデル
// financialCloseFinalReport.md 第2章 財務情報基盤に基づく

pub mod accounting_period;
pub mod closing_events;
pub mod company;
pub mod journal_entry;
pub mod ledger;
pub mod valuation_service;
pub mod values;

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 決算期間
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountingPeriod {
    year: u32,
    month: u8,
}

impl ValueObject for AccountingPeriod {
    fn validate(&self) -> DomainResult<()> {
        if self.month < 1 || self.month > 12 {
            return Err(DomainError::InvalidAccountingPeriod);
        }
        Ok(())
    }
}

impl AccountingPeriod {
    pub fn new(year: u32, month: u8) -> DomainResult<Self> {
        let period = Self { year, month };
        period.validate()?;
        Ok(period)
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }
}

/// 金額（円）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Amount {
    value: i64, // 円単位
}

impl ValueObject for Amount {
    fn validate(&self) -> DomainResult<()> {
        // 金額の妥当性検証
        Ok(())
    }
}

impl Amount {
    pub fn new(value: i64) -> DomainResult<Self> {
        let amount = Self { value };
        amount.validate()?;
        Ok(amount)
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn add(&self, other: &Amount) -> Amount {
        Amount { value: self.value + other.value }
    }

    pub fn subtract(&self, other: &Amount) -> Amount {
        Amount { value: self.value - other.value }
    }
}

/// 勘定科目コード
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountCode {
    code: String,
}

impl ValueObject for AccountCode {
    fn validate(&self) -> DomainResult<()> {
        if self.code.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }
        Ok(())
    }
}

impl AccountCode {
    pub fn new(code: String) -> DomainResult<Self> {
        let account_code = Self { code };
        account_code.validate()?;
        Ok(account_code)
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

#[cfg(test)]
pub mod accounting_period_tests;
#[cfg(test)]
pub mod journal_entry_line_tests;
#[cfg(test)]
pub mod values_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounting_period_valid() {
        let period = AccountingPeriod::new(2024, 1);
        assert!(period.is_ok());
        let period = period.unwrap();
        assert_eq!(period.year(), 2024);
        assert_eq!(period.month(), 1);
    }

    #[test]
    fn test_accounting_period_invalid_month() {
        let period = AccountingPeriod::new(2024, 0);
        assert!(period.is_err());

        let period = AccountingPeriod::new(2024, 13);
        assert!(period.is_err());
    }

    #[test]
    fn test_amount_operations() {
        let amount1 = Amount::new(1000).unwrap();
        let amount2 = Amount::new(500).unwrap();

        let sum = amount1.add(&amount2);
        assert_eq!(sum.value(), 1500);

        let diff = amount1.subtract(&amount2);
        assert_eq!(diff.value(), 500);
    }

    #[test]
    fn test_account_code_valid() {
        let code = AccountCode::new("1000".to_string());
        assert!(code.is_ok());
        assert_eq!(code.unwrap().code(), "1000");
    }

    #[test]
    fn test_account_code_empty() {
        let code = AccountCode::new("".to_string());
        assert!(code.is_err());
    }
}
