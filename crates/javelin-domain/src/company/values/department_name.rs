// DepartmentName - 部署名値オブジェクト

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 部署名
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepartmentName(String);

impl DepartmentName {
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        if name.is_empty() {
            return Err(DomainError::ValidationError(
                "部署名は空にできません".to_string(),
            ));
        }
        if name.len() > 200 {
            return Err(DomainError::ValidationError(
                "部署名は200文字以内でなければなりません".to_string(),
            ));
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for DepartmentName {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::ValidationError(
                "部署名は空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

impl FromStr for DepartmentName {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for DepartmentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
