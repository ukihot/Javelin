// Permission - 権限値オブジェクト

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 権限（リソース:アクション形式）
///
/// 権限はリソースとアクションの組み合わせで表現する。
/// 例: "journal_entry:create", "journal_entry:approve", "closing:execute", "master:edit"
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission(String);

impl Permission {
    pub fn new(permission: impl Into<String>) -> DomainResult<Self> {
        let permission = permission.into();
        if permission.is_empty() {
            return Err(DomainError::ValidationError("権限は空にできません".to_string()));
        }
        if !permission.contains(':') {
            return Err(DomainError::ValidationError(
                "権限は 'リソース:アクション' 形式でなければなりません".to_string(),
            ));
        }
        Ok(Self(permission))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    /// リソース部分を取得
    pub fn resource(&self) -> &str {
        self.0.split(':').next().unwrap_or("")
    }

    /// アクション部分を取得
    pub fn action(&self) -> &str {
        self.0.split(':').nth(1).unwrap_or("")
    }

    /// ワイルドカード権限かどうか（例: "journal_entry:*"）
    pub fn is_wildcard(&self) -> bool {
        self.action() == "*"
    }

    /// 指定された権限を包含するかどうか
    pub fn covers(&self, other: &Permission) -> bool {
        if self.resource() == "*" {
            return true;
        }
        if self.resource() != other.resource() {
            return false;
        }
        self.is_wildcard() || self.action() == other.action()
    }
}

impl ValueObject for Permission {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() || !self.0.contains(':') {
            return Err(DomainError::ValidationError(
                "権限は 'リソース:アクション' 形式でなければなりません".to_string(),
            ));
        }
        Ok(())
    }
}

impl FromStr for Permission {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
