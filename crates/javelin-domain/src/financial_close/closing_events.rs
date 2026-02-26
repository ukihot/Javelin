// 決算処理ドメインイベント
// 勘定補正・IFRS評価などの決算処理をイベントとして記録

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 決算処理ドメインイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ClosingEvent {
    /// 勘定補正実施
    AccountAdjusted {
        adjustment_id: String,
        fiscal_year: i32,
        period: u8,
        account_code: String,
        adjustment_type: String, // "Reclassification", "Temporary", etc.
        amount: f64,
        currency: String,
        reason: String,
        adjusted_by: String,
        adjusted_at: DateTime<Utc>,
    },

    /// IFRS評価実施
    IfrsValuationApplied {
        valuation_id: String,
        fiscal_year: i32,
        period: u8,
        valuation_type: String, // "ExpectedCreditLoss", "FairValue", etc.
        account_code: String,
        amount: f64,
        currency: String,
        applied_by: String,
        applied_at: DateTime<Utc>,
    },
}

impl ClosingEvent {
    pub fn event_type(&self) -> &str {
        match self {
            ClosingEvent::AccountAdjusted { .. } => "AccountAdjusted",
            ClosingEvent::IfrsValuationApplied { .. } => "IfrsValuationApplied",
        }
    }

    pub fn aggregate_id(&self) -> &str {
        match self {
            ClosingEvent::AccountAdjusted { adjustment_id, .. } => adjustment_id,
            ClosingEvent::IfrsValuationApplied { valuation_id, .. } => valuation_id,
        }
    }

    pub fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            ClosingEvent::AccountAdjusted { adjusted_at, .. } => *adjusted_at,
            ClosingEvent::IfrsValuationApplied { applied_at, .. } => *applied_at,
        }
    }

    pub fn actor(&self) -> &str {
        match self {
            ClosingEvent::AccountAdjusted { adjusted_by, .. } => adjusted_by,
            ClosingEvent::IfrsValuationApplied { applied_by, .. } => applied_by,
        }
    }
}

impl crate::event::DomainEvent for ClosingEvent {
    fn event_type(&self) -> &str {
        self.event_type()
    }

    fn aggregate_id(&self) -> &str {
        self.aggregate_id()
    }

    fn version(&self) -> u64 {
        0
    }
}
