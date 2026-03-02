// 重要性基準のイベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    entities::ApprovalLevel,
    values::{MaterialityJudgmentId, QualitativeFactor, SensitivityAnalysisResult},
};
use crate::{common::Amount, event::DomainEvent};

/// 重要性判定イベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialityJudgmentEvent {
    pub judgment_id: MaterialityJudgmentId,
    pub event_type: MaterialityJudgmentEventType,
    pub occurred_at: DateTime<Utc>,
    pub version: u64,
}

impl MaterialityJudgmentEvent {
    pub fn new(
        judgment_id: MaterialityJudgmentId,
        event_type: MaterialityJudgmentEventType,
    ) -> Self {
        Self { judgment_id, event_type, occurred_at: Utc::now(), version: 1 }
    }

    pub fn with_version(
        judgment_id: MaterialityJudgmentId,
        event_type: MaterialityJudgmentEventType,
        version: u64,
    ) -> Self {
        Self { judgment_id, event_type, occurred_at: Utc::now(), version }
    }
}

impl DomainEvent for MaterialityJudgmentEvent {
    fn event_type(&self) -> &str {
        match &self.event_type {
            MaterialityJudgmentEventType::QuantitativeJudgmentMade { .. } => {
                "QuantitativeJudgmentMade"
            }
            MaterialityJudgmentEventType::QualitativeJudgmentMade { .. } => {
                "QualitativeJudgmentMade"
            }
            MaterialityJudgmentEventType::EstimateJudgmentMade { .. } => "EstimateJudgmentMade",
            MaterialityJudgmentEventType::SensitivityAnalysisCompleted { .. } => {
                "SensitivityAnalysisCompleted"
            }
            MaterialityJudgmentEventType::MaterialityEvaluated { .. } => "MaterialityEvaluated",
            MaterialityJudgmentEventType::JudgmentApproved { .. } => "JudgmentApproved",
            MaterialityJudgmentEventType::ThresholdExceeded { .. } => "ThresholdExceeded",
        }
    }

    fn aggregate_id(&self) -> &str {
        Box::leak(self.judgment_id.to_string().into_boxed_str())
    }

    fn version(&self) -> u64 {
        self.version
    }
}

/// 重要性判定イベントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialityJudgmentEventType {
    /// 金額的重要性判定実施
    QuantitativeJudgmentMade {
        amount: Amount,
        threshold: Amount,
        is_material: bool,
        approval_level: ApprovalLevel,
    },
    /// 質的重要性判定実施
    QualitativeJudgmentMade { factors: Vec<QualitativeFactor>, approval_level: ApprovalLevel },
    /// 見積重要性判定実施
    EstimateJudgmentMade { parameter_count: usize },
    /// 感度分析完了
    SensitivityAnalysisCompleted { results: Vec<SensitivityAnalysisResult>, max_impact: Amount },
    /// 重要性評価完了
    MaterialityEvaluated { is_material: bool, requires_approval: bool },
    /// 判定承認
    JudgmentApproved { approver: String, approval_date: DateTime<Utc> },
    /// 閾値超過警告
    ThresholdExceeded { amount: Amount, threshold: Amount, excess_ratio: f64 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantitative_judgment_event() {
        let judgment_id = MaterialityJudgmentId::new();

        let event = MaterialityJudgmentEvent::new(
            judgment_id.clone(),
            MaterialityJudgmentEventType::QuantitativeJudgmentMade {
                amount: Amount::from_i64(30_000_000),
                threshold: Amount::from_i64(25_000_000),
                is_material: true,
                approval_level: ApprovalLevel::Manager,
            },
        );

        assert_eq!(event.event_type(), "QuantitativeJudgmentMade");
        assert_eq!(event.judgment_id, judgment_id);
    }

    #[test]
    fn test_qualitative_judgment_event() {
        let judgment_id = MaterialityJudgmentId::new();

        let event = MaterialityJudgmentEvent::new(
            judgment_id,
            MaterialityJudgmentEventType::QualitativeJudgmentMade {
                factors: vec![QualitativeFactor::AccountingPolicyChange],
                approval_level: ApprovalLevel::CFO,
            },
        );

        assert_eq!(event.event_type(), "QualitativeJudgmentMade");
    }

    #[test]
    fn test_sensitivity_analysis_event() {
        let judgment_id = MaterialityJudgmentId::new();

        let result = SensitivityAnalysisResult::new(
            "割引率".to_string(),
            Amount::from_i64(1_000_000),
            Amount::from_i64(1_100_000),
            Amount::from_i64(950_000),
        );

        let event = MaterialityJudgmentEvent::new(
            judgment_id,
            MaterialityJudgmentEventType::SensitivityAnalysisCompleted {
                results: vec![result],
                max_impact: Amount::from_i64(100_000),
            },
        );

        assert_eq!(event.event_type(), "SensitivityAnalysisCompleted");
    }

    #[test]
    fn test_judgment_approved_event() {
        let judgment_id = MaterialityJudgmentId::new();

        let event = MaterialityJudgmentEvent::new(
            judgment_id,
            MaterialityJudgmentEventType::JudgmentApproved {
                approver: "課長A".to_string(),
                approval_date: Utc::now(),
            },
        );

        assert_eq!(event.event_type(), "JudgmentApproved");
    }

    #[test]
    fn test_threshold_exceeded_event() {
        let judgment_id = MaterialityJudgmentId::new();

        let event = MaterialityJudgmentEvent::new(
            judgment_id,
            MaterialityJudgmentEventType::ThresholdExceeded {
                amount: Amount::from_i64(50_000_000),
                threshold: Amount::from_i64(25_000_000),
                excess_ratio: 2.0,
            },
        );

        assert_eq!(event.event_type(), "ThresholdExceeded");
    }
}
