// EntryNumber - 伝票番号値オブジェクト

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 伝票番号
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryNumber(String);

impl ValueObject for EntryNumber {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount("Entry number cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl EntryNumber {
    pub fn new(number: String) -> DomainResult<Self> {
        let entry_number = Self(number);
        entry_number.validate()?;
        Ok(entry_number)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
