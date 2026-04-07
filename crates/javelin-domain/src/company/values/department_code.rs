// DepartmentCode - 部署コード値オブジェクト

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 部署コード
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DepartmentCode(String);

impl DepartmentCode {
    pub fn new(code: impl Into<String>) -> DomainResult<Self> {
        let code = code.into();
        if code.is_empty() {
            return Err(DomainError::ValidationError(
                "部署コードは空にできません".to_string(),
            ));
        }
        if code.len() > 20 {
            return Err(DomainError::ValidationError(
                "部署コードは20文字以内でなければなりません".to_string(),
            ));
        }
        Ok(Self(code))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for DepartmentCode {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::ValidationError(
                "部署コードは空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

impl FromStr for DepartmentCode {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for DepartmentCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
