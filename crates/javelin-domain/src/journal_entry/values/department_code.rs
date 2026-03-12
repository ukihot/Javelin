// DepartmentCode - 部門コード値オブジェクト

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 部門コード
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepartmentCode(String);

impl ValueObject for DepartmentCode {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount("Department code cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl DepartmentCode {
    pub fn new(code: String) -> DomainResult<Self> {
        let department_code = Self(code);
        department_code.validate()?;
        Ok(department_code)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
