// 債権債務集約のドメインイベント

use serde::{Deserialize, Serialize};

use crate::event::DomainEvent;

/// 債権債務が決済されたイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivablePayableSettled {
    pub receivable_payable_id: String,
    pub partner_id: String,
    pub amount: String,
}

impl DomainEvent for ReceivablePayableSettled {
    fn event_type(&self) -> &str {
        "receivable_payable_settled"
    }

    fn aggregate_id(&self) -> &str {
        &self.receivable_payable_id
    }

    fn version(&self) -> u64 {
        1
    }
}