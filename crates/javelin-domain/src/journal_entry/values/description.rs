// Description - 摘要値オブジェクト

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
