// LineNumber - 行番号値オブジェクト

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 行番号
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineNumber(u32);

impl ValueObject for LineNumber {
    fn validate(&self) -> DomainResult<()> {
        if self.0 == 0 {
            return Err(DomainError::InvalidAmount(
                "Line number must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

impl LineNumber {
    pub fn new(number: u32) -> DomainResult<Self> {
        let line_number = Self(number);
        line_number.validate()?;
        Ok(line_number)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}
