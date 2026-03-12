// SubsidiaryAccountCode - 補助科目コード値オブジェクト

use crate::{error::DomainResult, value_object::ValueObject};

/// 補助科目コード
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubsidiaryAccountCode(String);

impl SubsidiaryAccountCode {
    pub fn new(code: impl Into<String>) -> DomainResult<Self> {
        let code = code.into();
        if code.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "補助科目コードは空にできません".to_string(),
            ));
        }
        Ok(Self(code))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for SubsidiaryAccountCode {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "補助科目コードは空にできません".to_string(),
            ));
        }
        Ok(())
    }
}
