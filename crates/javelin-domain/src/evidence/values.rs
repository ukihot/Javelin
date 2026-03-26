// 証憑集約の値オブジェクト

use serde::{Deserialize, Serialize};

use crate::value_object::ValueObject;

/// 証憑タイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    Receipt,       // 領収書
    Invoice,       // 請求書
    BankStatement, // 銀行明細
    Contract,      // 契約書
    Other,         // その他
}

impl ValueObject for EvidenceType {
    fn validate(&self) -> crate::error::DomainResult<()> {
        Ok(())
    }
}
