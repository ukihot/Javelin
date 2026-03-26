// BalanceTracking集約のドメインイベント

use serde::{Deserialize, Serialize};

use crate::event::DomainEvent;

/// BalanceTrackingが決済されたイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceTrackingSettled {
    pub balance_tracking_id: String,
    pub partner_id: String,
    pub amount: String,
}

impl DomainEvent for BalanceTrackingSettled {
    fn event_type(&self) -> &str {
        "balance_tracking_settled"
    }

    fn aggregate_id(&self) -> &str {
        &self.balance_tracking_id
    }

    fn version(&self) -> u64 {
        1
    }
}
