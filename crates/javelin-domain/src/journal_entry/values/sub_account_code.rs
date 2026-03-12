// SubAccountCode - 補助科目コード値オブジェクト

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 補助科目コード
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubAccountCode(String);

impl ValueObject for SubAccountCode {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }
        Ok(())
    }
}

impl SubAccountCode {
    pub fn new(code: String) -> DomainResult<Self> {
        let sub_account_code = Self(code);
        sub_account_code.validate()?;
        Ok(sub_account_code)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
