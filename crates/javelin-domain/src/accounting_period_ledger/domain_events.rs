// 会計期間・帳簿集約のドメインイベント

use serde::{Deserialize, Serialize};

use crate::event::DomainEvent;

/// 会計期間が閉じられたイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingPeriodClosed {
    pub period_id: String,
    pub year: u32,
    pub month: u8,
}

impl DomainEvent for AccountingPeriodClosed {
    fn event_type(&self) -> &str {
        "accounting_period_closed"
    }

    fn aggregate_id(&self) -> &str {
        &self.period_id
    }

    fn version(&self) -> u64 {
        1
    }
}
