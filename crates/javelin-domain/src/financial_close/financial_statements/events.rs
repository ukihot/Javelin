// 財務諸表生成イベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{FinancialStatementId, FinancialStatementType};
use crate::event::DomainEvent;

/// 財務諸表イベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialStatementEvent {
    pub statement_id: FinancialStatementId,
    pub event_type: FinancialStatementEventType,
    pub occurred_at: DateTime<Utc>,
    pub version: u64,
}

impl FinancialStatementEvent {
    pub fn new(
        statement_id: FinancialStatementId,
        event_type: FinancialStatementEventType,
    ) -> Self {
        Self { statement_id, event_type, occurred_at: Utc::now(), version: 1 }
    }

    pub fn with_version(
        statement_id: FinancialStatementId,
        event_type: FinancialStatementEventType,
        version: u64,
    ) -> Self {
        Self { statement_id, event_type, occurred_at: Utc::now(), version }
    }
}

impl DomainEvent for FinancialStatementEvent {
    fn event_type(&self) -> &str {
        match &self.event_type {
            FinancialStatementEventType::StatementGenerated { .. } => "StatementGenerated",
            FinancialStatementEventType::ItemAdded { .. } => "ItemAdded",
            FinancialStatementEventType::ItemsSorted { .. } => "ItemsSorted",
            FinancialStatementEventType::StatementApproved { .. } => "StatementApproved",
            FinancialStatementEventType::ConsistencyVerified { .. } => "ConsistencyVerified",
            FinancialStatementEventType::CrossCheckCompleted { .. } => "CrossCheckCompleted",
        }
    }

    fn aggregate_id(&self) -> &str {
        Box::leak(self.statement_id.to_string().into_boxed_str())
    }

    fn version(&self) -> u64 {
        self.version
    }
}

/// 財務諸表イベントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinancialStatementEventType {
    /// 財務諸表生成
    StatementGenerated {
        statement_type: FinancialStatementType,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    },
    /// 項目追加
    ItemAdded { item_count: usize },
    /// 項目ソート
    ItemsSorted { sort_order: String },
    /// 財務諸表承認
    StatementApproved { approver: String, approval_date: DateTime<Utc> },
    /// 整合性検証完了
    ConsistencyVerified { is_consistent: bool, discrepancies: Vec<String> },
    /// クロスチェック完了
    CrossCheckCompleted { checks_passed: usize, checks_failed: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statement_generated_event() {
        let statement_id = FinancialStatementId::new();
        let event = FinancialStatementEvent::new(
            statement_id.clone(),
            FinancialStatementEventType::StatementGenerated {
                statement_type: FinancialStatementType::BalanceSheet,
                period_start: Utc::now(),
                period_end: Utc::now(),
            },
        );

        assert_eq!(event.event_type(), "StatementGenerated");
        assert_eq!(event.statement_id, statement_id);
    }

    #[test]
    fn test_statement_approved_event() {
        let statement_id = FinancialStatementId::new();
        let event = FinancialStatementEvent::new(
            statement_id,
            FinancialStatementEventType::StatementApproved {
                approver: "CFO".to_string(),
                approval_date: Utc::now(),
            },
        );

        assert_eq!(event.event_type(), "StatementApproved");
    }

    #[test]
    fn test_consistency_verified_event() {
        let statement_id = FinancialStatementId::new();
        let event = FinancialStatementEvent::new(
            statement_id,
            FinancialStatementEventType::ConsistencyVerified {
                is_consistent: true,
                discrepancies: vec![],
            },
        );

        assert_eq!(event.event_type(), "ConsistencyVerified");
    }
}
