// VoucherNumber - 証憑番号値オブジェクト

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 証憑番号
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoucherNumber(String);

impl ValueObject for VoucherNumber {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount("Voucher number cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl VoucherNumber {
    pub fn new(number: String) -> DomainResult<Self> {
        let voucher_number = Self(number);
        voucher_number.validate()?;
        Ok(voucher_number)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
