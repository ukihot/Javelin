// 会計期間・帳簿集約の値オブジェクト

use serde::{Deserialize, Serialize};

use crate::value_object::ValueObject;

/// 会計期間ステータス
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountingPeriodStatus {
    Open,
    Closed,
}

impl ValueObject for AccountingPeriodStatus {
    fn validate(&self) -> crate::error::DomainResult<()> {
        Ok(())
    }
}
