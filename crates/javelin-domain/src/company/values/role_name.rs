// RoleName - ロール名値オブジェクト

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// ロール名
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleName(String);

impl RoleName {
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        if name.is_empty() {
            return Err(DomainError::ValidationError("ロール名は空にできません".to_string()));
        }
        if name.len() > 100 {
            return Err(DomainError::ValidationError(
                "ロール名は100文字以内でなければなりません".to_string(),
            ));
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for RoleName {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::ValidationError("ロール名は空にできません".to_string()));
        }
        Ok(())
    }
}

impl FromStr for RoleName {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for RoleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
