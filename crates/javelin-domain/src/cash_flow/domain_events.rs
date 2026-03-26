// 入出金集約のドメインイベント

use serde::{Deserialize, Serialize};

use crate::event::DomainEvent;

/// 入出金が記録されたイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlowRecorded {
    pub cash_flow_id: String,
    pub date: String,
    pub amount: String,
    pub account_id: String,
}

impl DomainEvent for CashFlowRecorded {
    fn event_type(&self) -> &str {
        "cash_flow_recorded"
    }

    fn aggregate_id(&self) -> &str {
        &self.cash_flow_id
    }

    fn version(&self) -> u64 {
        1
    }
}
