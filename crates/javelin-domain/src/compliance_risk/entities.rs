// ComplianceRiskSnapshot - リスク監視スナップショット集約

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{RiskIndicator, RiskIndicatorType};
use crate::{
    entity::{Entity, EntityId},
    error::{DomainError, DomainResult},
};

/// ComplianceRiskSnapshotID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComplianceRiskSnapshotId(String);

impl ComplianceRiskSnapshotId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl EntityId for ComplianceRiskSnapshotId {
    fn value(&self) -> &str {
        &self.0
    }
}

/// 月次コンプライアンス・リスク監視スナップショット集約
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRiskSnapshot {
    id: ComplianceRiskSnapshotId,
    period_year: u32,
    period_month: u8,
    indicators: Vec<RiskIndicator>,
    overall_risk_level: String,
    captured_at: DateTime<Utc>,
    reviewed_by: Option<String>,
    reviewed_at: Option<DateTime<Utc>>,
}

impl Entity for ComplianceRiskSnapshot {
    type Id = ComplianceRiskSnapshotId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ComplianceRiskSnapshot {
    pub fn new(
        period_year: u32,
        period_month: u8,
        indicators: Vec<RiskIndicator>,
    ) -> DomainResult<Self> {
        if !(1..=12).contains(&period_month) {
            return Err(DomainError::ValidationError(
                "月は1-12の範囲である必要があります".to_string(),
            ));
        }

        if indicators.len() != 8 {
            return Err(DomainError::ValidationError("リスク指標は8個必要です".to_string()));
        }

        let overall_risk_level = Self::calculate_overall_level(&indicators);

        Ok(Self {
            id: ComplianceRiskSnapshotId::new(uuid::Uuid::new_v4().to_string()),
            period_year,
            period_month,
            indicators,
            overall_risk_level,
            captured_at: Utc::now(),
            reviewed_by: None,
            reviewed_at: None,
        })
    }

    pub fn id(&self) -> &ComplianceRiskSnapshotId {
        &self.id
    }

    pub fn period_year(&self) -> u32 {
        self.period_year
    }

    pub fn period_month(&self) -> u8 {
        self.period_month
    }

    pub fn indicators(&self) -> &[RiskIndicator] {
        &self.indicators
    }

    pub fn overall_risk_level(&self) -> &str {
        &self.overall_risk_level
    }

    pub fn captured_at(&self) -> DateTime<Utc> {
        self.captured_at
    }

    pub fn reviewed_by(&self) -> Option<&str> {
        self.reviewed_by.as_deref()
    }

    pub fn reviewed_at(&self) -> Option<DateTime<Utc>> {
        self.reviewed_at
    }

    pub fn get_indicator(&self, indicator_type: &RiskIndicatorType) -> Option<&RiskIndicator> {
        self.indicators.iter().find(|i| i.indicator_type() == indicator_type)
    }

    pub fn mark_reviewed(&mut self, reviewer_id: String) -> DomainResult<()> {
        self.reviewed_by = Some(reviewer_id);
        self.reviewed_at = Some(Utc::now());
        Ok(())
    }

    fn calculate_overall_level(indicators: &[RiskIndicator]) -> String {
        // BigDecimalで精密計算
        let sum_score: BigDecimal = indicators.iter().map(|i| i.score().value()).sum();
        let count = BigDecimal::from(indicators.len() as i32);
        let avg_score = if indicators.is_empty() {
            BigDecimal::from(0)
        } else {
            sum_score / count
        };

        let critical = BigDecimal::from(60);
        let high = BigDecimal::from(40);
        let medium = BigDecimal::from(20);

        if avg_score >= critical {
            "Critical".to_string()
        } else if avg_score >= high {
            "High".to_string()
        } else if avg_score >= medium {
            "Medium".to_string()
        } else {
            "Low".to_string()
        }
    }
}
