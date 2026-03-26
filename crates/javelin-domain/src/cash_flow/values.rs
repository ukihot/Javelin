// 入出金集約の値オブジェクト

use serde::{Deserialize, Serialize};

use crate::value_object::ValueObject;

/// 入出金タイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CashFlowType {
    Deposit,    // 入金
    Withdrawal, // 出金
}

impl ValueObject for CashFlowType {
    fn validate(&self) -> crate::error::DomainResult<()> {
        Ok(())
    }
}
