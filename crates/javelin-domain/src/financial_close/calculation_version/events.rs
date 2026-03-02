// 計算バージョンのイベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{CalculationVersionId, VersionStatus};
use crate::event::DomainEvent;

/// 計算バージョンイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationVersionEvent {
    pub version_id: CalculationVersionId,
    pub event_type: CalculationVersionEventType,
    pub occurred_at: DateTime<Utc>,
    pub version: u64,
}

impl CalculationVersionEvent {
    pub fn new(version_id: CalculationVersionId, event_type: CalculationVersionEventType) -> Self {
        Self { version_id, event_type, occurred_at: Utc::now(), version: 1 }
    }

    pub fn with_version(
        version_id: CalculationVersionId,
        event_type: CalculationVersionEventType,
        version: u64,
    ) -> Self {
        Self { version_id, event_type, occurred_at: Utc::now(), version }
    }
}

impl DomainEvent for CalculationVersionEvent {
    fn event_type(&self) -> &str {
        match &self.event_type {
            CalculationVersionEventType::VersionCreated { .. } => "VersionCreated",
            CalculationVersionEventType::StatusChanged { .. } => "StatusChanged",
            CalculationVersionEventType::ApprovalRecorded { .. } => "ApprovalRecorded",
            CalculationVersionEventType::EffectivePeriodSet { .. } => "EffectivePeriodSet",
        }
    }

    fn aggregate_id(&self) -> &str {
        Box::leak(self.version_id.to_string().into_boxed_str())
    }

    fn version(&self) -> u64 {
        self.version
    }
}

/// 計算バージョンイベントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalculationVersionEventType {
    /// バージョン作成
    VersionCreated { logic_name: String, version_number: String },
    /// ステータス変更
    StatusChanged { old_status: VersionStatus, new_status: VersionStatus },
    /// 承認記録
    ApprovalRecorded { approver_id: String, approved: bool },
    /// 有効期間設定
    EffectivePeriodSet { from: DateTime<Utc>, to: Option<DateTime<Utc>> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_created_event() {
        let id = CalculationVersionId::new();
        let event = CalculationVersionEvent::new(
            id.clone(),
            CalculationVersionEventType::VersionCreated {
                logic_name: "ECL Calculation".to_string(),
                version_number: "1.0.0".to_string(),
            },
        );

        assert_eq!(event.event_type(), "VersionCreated");
        assert_eq!(event.version_id, id);
        assert_eq!(event.version(), 1);
    }

    #[test]
    fn test_status_changed_event() {
        let id = CalculationVersionId::new();
        let event = CalculationVersionEvent::new(
            id,
            CalculationVersionEventType::StatusChanged {
                old_status: VersionStatus::Draft,
                new_status: VersionStatus::PendingApproval,
            },
        );

        assert_eq!(event.event_type(), "StatusChanged");
    }
}
