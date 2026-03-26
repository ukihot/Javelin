// BalanceTracking集約の値オブジェクト

use serde::{Deserialize, Serialize};

use crate::value_object::ValueObject;

/// 取引タイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Receivable, // 債権（売掛金）
    Payable,    // 債務（買掛金）
}

impl ValueObject for TransactionType {
    fn validate(&self) -> crate::error::DomainResult<()> {
        Ok(())
    }
}

/// BalanceTrackingステータス
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceTrackingStatus {
    Pending, // 未決済
    Settled, // 決済済み
    Overdue, // 期限超過
}

impl ValueObject for BalanceTrackingStatus {
    fn validate(&self) -> crate::error::DomainResult<()> {
        Ok(())
    }
}
