// Email - メールアドレス値オブジェクト

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// メールアドレス
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> DomainResult<Self> {
        let email = email.into();
        if email.is_empty() {
            return Err(DomainError::ValidationError(
                "メールアドレスは空にできません".to_string(),
            ));
        }
        if !email.contains('@') || !email.contains('.') {
            return Err(DomainError::ValidationError(
                "メールアドレスの形式が不正です".to_string(),
            ));
        }
        Ok(Self(email))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for Email {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() || !self.0.contains('@') {
            return Err(DomainError::ValidationError(
                "メールアドレスの形式が不正です".to_string(),
            ));
        }
        Ok(())
    }
}

impl FromStr for Email {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
