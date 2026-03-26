// 消込集約の値オブジェクト

use serde::{Deserialize, Serialize};

use crate::value_object::ValueObject;

/// 消込ステータス
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReconciliationStatus {
    Partial, // 部分消込
    Full,    // 全額消込
}

impl ValueObject for ReconciliationStatus {
    fn validate(&self) -> crate::error::DomainResult<()> {
        Ok(())
    }
}
