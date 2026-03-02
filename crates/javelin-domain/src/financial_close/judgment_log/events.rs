// 判断ログのイベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{JudgmentLogId, JudgmentType};
use crate::event::DomainEvent;

/// 判断ログイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgmentLogEvent {
    pub judgment_log_id: JudgmentLogId,
    pub event_type: JudgmentLogEventType,
    pub occurred_at: DateTime<Utc>,
    pub version: u64,
}

impl JudgmentLogEvent {
    pub fn new(judgment_log_id: JudgmentLogId, event_type: JudgmentLogEventType) -> Self {
        Self { judgment_log_id, event_type, occurred_at: Utc::now(), version: 1 }
    }

    pub fn with_version(
        judgment_log_id: JudgmentLogId,
        event_type: JudgmentLogEventType,
        version: u64,
    ) -> Self {
        Self { judgment_log_id, event_type, occurred_at: Utc::now(), version }
    }
}

impl DomainEvent for JudgmentLogEvent {
    fn event_type(&self) -> &str {
        match &self.event_type {
            JudgmentLogEventType::JudgmentLogCreated { .. } => "JudgmentLogCreated",
            JudgmentLogEventType::SensitivityAnalysisAdded => "SensitivityAnalysisAdded",
            JudgmentLogEventType::AssumptionChanged { .. } => "AssumptionChanged",
            JudgmentLogEventType::RelatedEntitySet { .. } => "RelatedEntitySet",
        }
    }

    fn aggregate_id(&self) -> &str {
        Box::leak(self.judgment_log_id.to_string().into_boxed_str())
    }

    fn version(&self) -> u64 {
        self.version
    }
}

/// 判断ログイベントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JudgmentLogEventType {
    /// 判断ログ作成
    JudgmentLogCreated { judgment_type: JudgmentType, approver_id: String },
    /// 感度分析追加
    SensitivityAnalysisAdded,
    /// 前提条件変更
    AssumptionChanged { target: String, is_material: bool },
    /// 関連エンティティ設定
    RelatedEntitySet { entity_id: String, entity_type: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judgment_log_created_event() {
        let id = JudgmentLogId::new();
        let event = JudgmentLogEvent::new(
            id.clone(),
            JudgmentLogEventType::JudgmentLogCreated {
                judgment_type: JudgmentType::Impairment,
                approver_id: "USER001".to_string(),
            },
        );

        assert_eq!(event.event_type(), "JudgmentLogCreated");
        assert_eq!(event.judgment_log_id, id);
        assert_eq!(event.version(), 1);
    }

    #[test]
    fn test_assumption_changed_event() {
        let id = JudgmentLogId::new();
        let event = JudgmentLogEvent::new(
            id,
            JudgmentLogEventType::AssumptionChanged {
                target: "discount_rate".to_string(),
                is_material: true,
            },
        );

        assert_eq!(event.event_type(), "AssumptionChanged");
    }
}
