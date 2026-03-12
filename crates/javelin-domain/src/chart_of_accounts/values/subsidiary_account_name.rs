// SubsidiaryAccountName - 補助科目名値オブジェクト

use crate::{error::DomainResult, value_object::ValueObject};

/// 補助科目名
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubsidiaryAccountName(String);

impl SubsidiaryAccountName {
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        if name.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "補助科目名は空にできません".to_string(),
            ));
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for SubsidiaryAccountName {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "補助科目名は空にできません".to_string(),
            ));
        }
        Ok(())
    }
}
