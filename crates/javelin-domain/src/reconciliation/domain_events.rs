// 消込集約のドメインイベント

use serde::{Deserialize, Serialize};

use crate::event::DomainEvent;

/// 消込が実行されたイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationExecuted {
    pub reconciliation_id: String,
    pub receivable_payable_id: String,
    pub cash_flow_id: String,
    pub amount: String,
}

impl DomainEvent for ReconciliationExecuted {
    fn event_type(&self) -> &str {
        "reconciliation_executed"
    }

    fn aggregate_id(&self) -> &str {
        &self.reconciliation_id
    }

    fn version(&self) -> u64 {
        1
    }
}
