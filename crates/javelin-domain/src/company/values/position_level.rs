// PositionLevel - 役職レベル値オブジェクト

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 役職レベル（数値が大きいほど上位）
///
/// 権限判定や承認フローにおける上下関係を表現する。
/// 例: 1=一般社員, 2=主任, 3=係長, 4=課長, 5=部長, 6=取締役, 7=代表取締役
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PositionLevel(u8);

impl PositionLevel {
    pub fn new(level: u8) -> DomainResult<Self> {
        if level == 0 {
            return Err(DomainError::ValidationError(
                "役職レベルは1以上でなければなりません".to_string(),
            ));
        }
        if level > 10 {
            return Err(DomainError::ValidationError(
                "役職レベルは10以下でなければなりません".to_string(),
            ));
        }
        Ok(Self(level))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    /// このレベルが他のレベル以上かどうか
    pub fn is_at_least(&self, other: &Self) -> bool {
        self.0 >= other.0
    }
}

impl ValueObject for PositionLevel {
    fn validate(&self) -> DomainResult<()> {
        if self.0 == 0 || self.0 > 10 {
            return Err(DomainError::ValidationError(
                "役職レベルは1〜10の範囲でなければなりません".to_string(),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for PositionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
