// 判断ログのエンティティ

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{
    JudgmentLogId, JudgmentType, ParameterValue, Scenario, SensitivityAnalysis,
    calculate_retention_expiry,
};
use crate::{
    common::Amount,
    entity::Entity,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 判断ログエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgmentLog {
    /// 判断ログID
    id: JudgmentLogId,
    /// 判断タイプ
    judgment_type: JudgmentType,
    /// 判断日
    judgment_date: DateTime<Utc>,
    /// 判断根拠
    judgment_basis: String,
    /// パラメータ
    parameters: HashMap<String, ParameterValue>,
    /// シナリオ
    scenarios: Vec<Scenario>,
    /// 感度分析
    sensitivity_analysis: Option<SensitivityAnalysis>,
    /// 承認者ID
    approver_id: String,
    /// 承認日
    approval_date: DateTime<Utc>,
    /// 保存期限
    retention_expiry: DateTime<Utc>,
    /// 関連エンティティID
    related_entity_id: Option<String>,
    /// 関連エンティティタイプ
    related_entity_type: Option<String>,
    /// 前提条件変更履歴
    assumption_changes: Vec<AssumptionChange>,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// 更新日時
    updated_at: DateTime<Utc>,
}

impl JudgmentLog {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: JudgmentLogId,
        judgment_type: JudgmentType,
        judgment_basis: String,
        parameters: HashMap<String, ParameterValue>,
        scenarios: Vec<Scenario>,
        approver_id: String,
    ) -> DomainResult<Self> {
        if judgment_basis.is_empty() {
            return Err(DomainError::InvalidJudgmentLog);
        }

        if approver_id.is_empty() {
            return Err(DomainError::InvalidJudgmentLog);
        }

        // シナリオの確率合計が1.0であることを確認
        if !scenarios.is_empty() {
            let total_probability: f64 = scenarios.iter().map(|s| s.probability()).sum();
            if (total_probability - 1.0).abs() > 0.01 {
                return Err(DomainError::InvalidJudgmentLog);
            }
        }

        let now = Utc::now();
        let retention_expiry = calculate_retention_expiry(now);

        Ok(Self {
            id,
            judgment_type,
            judgment_date: now,
            judgment_basis,
            parameters,
            scenarios,
            sensitivity_analysis: None,
            approver_id,
            approval_date: now,
            retention_expiry,
            related_entity_id: None,
            related_entity_type: None,
            assumption_changes: Vec::new(),
            created_at: now,
            updated_at: now,
        })
    }

    /// 感度分析を設定
    pub fn set_sensitivity_analysis(&mut self, analysis: SensitivityAnalysis) -> DomainResult<()> {
        // 基本的な検証
        if analysis.parameter_name().is_empty() {
            return Err(DomainError::InvalidJudgmentLog);
        }
        if analysis.variation_rate() <= 0.0 {
            return Err(DomainError::InvalidJudgmentLog);
        }

        self.sensitivity_analysis = Some(analysis);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 関連エンティティを設定
    pub fn set_related_entity(&mut self, entity_id: String, entity_type: String) {
        self.related_entity_id = Some(entity_id);
        self.related_entity_type = Some(entity_type);
        self.updated_at = Utc::now();
    }

    /// 前提条件変更を記録
    pub fn record_assumption_change(&mut self, change: AssumptionChange) -> DomainResult<()> {
        change.validate()?;
        self.assumption_changes.push(change);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 保存期限が切れているか確認
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.retention_expiry
    }

    /// 期待値を計算（シナリオベース）
    pub fn calculate_expected_value(&self) -> Option<i64> {
        if self.scenarios.is_empty() {
            return None;
        }

        let expected_value: f64 = self
            .scenarios
            .iter()
            .filter_map(|s| s.result_value().map(|v| v as f64 * s.probability()))
            .sum();

        Some(expected_value as i64)
    }

    /// 判断の信頼性を評価
    pub fn assess_reliability(&self) -> ReliabilityAssessment {
        let mut score = 100.0;

        // シナリオ数が少ない場合は減点
        if self.scenarios.len() < 3 {
            score -= 20.0;
        }

        // 感度分析がない場合は減点
        if self.sensitivity_analysis.is_none() {
            score -= 15.0;
        }

        // 前提条件変更が多い場合は減点
        if self.assumption_changes.len() > 5 {
            score -= 10.0 * (self.assumption_changes.len() - 5) as f64;
        }

        // パラメータが少ない場合は減点
        if self.parameters.len() < 3 {
            score -= 10.0;
        }

        score = score.clamp(0.0, 100.0);

        if score >= 80.0 {
            ReliabilityAssessment::High
        } else if score >= 60.0 {
            ReliabilityAssessment::Medium
        } else {
            ReliabilityAssessment::Low
        }
    }

    // Getters
    pub fn id(&self) -> &JudgmentLogId {
        &self.id
    }

    pub fn judgment_type(&self) -> &JudgmentType {
        &self.judgment_type
    }

    pub fn judgment_date(&self) -> DateTime<Utc> {
        self.judgment_date
    }

    pub fn judgment_basis(&self) -> &str {
        &self.judgment_basis
    }

    pub fn parameters(&self) -> &HashMap<String, ParameterValue> {
        &self.parameters
    }

    pub fn scenarios(&self) -> &[Scenario] {
        &self.scenarios
    }

    pub fn sensitivity_analysis(&self) -> Option<&SensitivityAnalysis> {
        self.sensitivity_analysis.as_ref()
    }

    pub fn approver_id(&self) -> &str {
        &self.approver_id
    }

    pub fn approval_date(&self) -> DateTime<Utc> {
        self.approval_date
    }

    pub fn retention_expiry(&self) -> DateTime<Utc> {
        self.retention_expiry
    }

    pub fn related_entity_id(&self) -> Option<&str> {
        self.related_entity_id.as_deref()
    }

    pub fn related_entity_type(&self) -> Option<&str> {
        self.related_entity_type.as_deref()
    }

    pub fn assumption_changes(&self) -> &[AssumptionChange] {
        &self.assumption_changes
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Entity for JudgmentLog {
    type Id = JudgmentLogId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

/// 前提条件変更
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssumptionChange {
    /// 変更日
    change_date: DateTime<Utc>,
    /// 変更対象
    target: String,
    /// 変更前値
    old_value: String,
    /// 変更後値
    new_value: String,
    /// 変更理由
    reason: String,
    /// 影響額
    impact_amount: Option<Amount>,
    /// 重要性判定
    is_material: bool,
}

impl AssumptionChange {
    pub fn new(
        target: String,
        old_value: String,
        new_value: String,
        reason: String,
        impact_amount: Option<Amount>,
        is_material: bool,
    ) -> DomainResult<Self> {
        if target.is_empty() || reason.is_empty() {
            return Err(DomainError::InvalidJudgmentLog);
        }

        Ok(Self {
            change_date: Utc::now(),
            target,
            old_value,
            new_value,
            reason,
            impact_amount,
            is_material,
        })
    }

    pub fn change_date(&self) -> DateTime<Utc> {
        self.change_date
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn old_value(&self) -> &str {
        &self.old_value
    }

    pub fn new_value(&self) -> &str {
        &self.new_value
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn impact_amount(&self) -> Option<&Amount> {
        self.impact_amount.as_ref()
    }

    pub fn is_material(&self) -> bool {
        self.is_material
    }
}

impl ValueObject for AssumptionChange {
    fn validate(&self) -> DomainResult<()> {
        if self.target.is_empty() || self.reason.is_empty() {
            return Err(DomainError::InvalidJudgmentLog);
        }
        Ok(())
    }
}

/// 信頼性評価
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReliabilityAssessment {
    High,
    Medium,
    Low,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_judgment_log() -> JudgmentLog {
        let id = JudgmentLogId::new();
        let mut params = HashMap::new();
        params.insert("discount_rate".to_string(), ParameterValue::Float(0.05));
        params.insert("growth_rate".to_string(), ParameterValue::Float(0.03));
        params.insert("terminal_value".to_string(), ParameterValue::Integer(10_000_000));

        let mut scenario_params = HashMap::new();
        scenario_params.insert("outcome".to_string(), ParameterValue::String("base".to_string()));

        let scenario = Scenario::new(
            "Base Case".to_string(),
            "Base scenario".to_string(),
            1.0,
            scenario_params,
        )
        .unwrap()
        .with_result(1_000_000);

        JudgmentLog::new(
            id,
            JudgmentType::Impairment,
            "DCF calculation for impairment test".to_string(),
            params,
            vec![scenario],
            "USER001".to_string(),
        )
        .unwrap()
    }

    #[test]
    fn test_judgment_log_creation() {
        let log = create_test_judgment_log();
        assert_eq!(log.judgment_type(), &JudgmentType::Impairment);
        assert_eq!(log.approver_id(), "USER001");
        assert_eq!(log.parameters().len(), 3);
        assert_eq!(log.scenarios().len(), 1);
    }

    #[test]
    fn test_judgment_log_invalid_basis() {
        let id = JudgmentLogId::new();
        let result = JudgmentLog::new(
            id,
            JudgmentType::Impairment,
            "".to_string(), // Invalid
            HashMap::new(),
            Vec::new(),
            "USER001".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_set_sensitivity_analysis() {
        let mut log = create_test_judgment_log();

        let analysis = SensitivityAnalysis::new(
            "discount_rate".to_string(),
            0.05,
            1_000_000,
            0.1,
            1_100_000,
            900_000,
        )
        .unwrap();

        assert!(log.set_sensitivity_analysis(analysis).is_ok());
        assert!(log.sensitivity_analysis().is_some());
    }

    #[test]
    fn test_set_related_entity() {
        let mut log = create_test_judgment_log();
        log.set_related_entity("ASSET001".to_string(), "FixedAsset".to_string());

        assert_eq!(log.related_entity_id(), Some("ASSET001"));
        assert_eq!(log.related_entity_type(), Some("FixedAsset"));
    }

    #[test]
    fn test_record_assumption_change() {
        let mut log = create_test_judgment_log();

        let change = AssumptionChange::new(
            "discount_rate".to_string(),
            "0.05".to_string(),
            "0.06".to_string(),
            "Market rate increased".to_string(),
            Some(Amount::from_i64(-50_000)),
            true,
        )
        .unwrap();

        assert!(log.record_assumption_change(change).is_ok());
        assert_eq!(log.assumption_changes().len(), 1);
    }

    #[test]
    fn test_is_expired() {
        let log = create_test_judgment_log();
        assert!(!log.is_expired()); // Should not be expired immediately
    }

    #[test]
    fn test_calculate_expected_value() {
        let id = JudgmentLogId::new();
        let params = HashMap::new();

        let scenario1 = Scenario::new(
            "Optimistic".to_string(),
            "Optimistic scenario".to_string(),
            0.3,
            HashMap::new(),
        )
        .unwrap()
        .with_result(1_500_000);

        let scenario2 =
            Scenario::new("Base".to_string(), "Base scenario".to_string(), 0.5, HashMap::new())
                .unwrap()
                .with_result(1_000_000);

        let scenario3 = Scenario::new(
            "Pessimistic".to_string(),
            "Pessimistic scenario".to_string(),
            0.2,
            HashMap::new(),
        )
        .unwrap()
        .with_result(500_000);

        let log = JudgmentLog::new(
            id,
            JudgmentType::Impairment,
            "Multi-scenario analysis".to_string(),
            params,
            vec![scenario1, scenario2, scenario3],
            "USER001".to_string(),
        )
        .unwrap();

        // Expected value = 0.3 * 1,500,000 + 0.5 * 1,000,000 + 0.2 * 500,000
        //                = 450,000 + 500,000 + 100,000 = 1,050,000
        assert_eq!(log.calculate_expected_value(), Some(1_050_000));
    }

    #[test]
    fn test_assess_reliability_high() {
        let id = JudgmentLogId::new();
        let mut params = HashMap::new();
        params.insert("param1".to_string(), ParameterValue::Integer(1));
        params.insert("param2".to_string(), ParameterValue::Integer(2));
        params.insert("param3".to_string(), ParameterValue::Integer(3));

        let scenarios = vec![
            Scenario::new("S1".to_string(), "Scenario 1".to_string(), 0.3, HashMap::new())
                .unwrap()
                .with_result(1_000_000),
            Scenario::new("S2".to_string(), "Scenario 2".to_string(), 0.4, HashMap::new())
                .unwrap()
                .with_result(1_100_000),
            Scenario::new("S3".to_string(), "Scenario 3".to_string(), 0.3, HashMap::new())
                .unwrap()
                .with_result(900_000),
        ];

        let mut log = JudgmentLog::new(
            id,
            JudgmentType::Impairment,
            "Comprehensive analysis".to_string(),
            params,
            scenarios,
            "USER001".to_string(),
        )
        .unwrap();

        let analysis = SensitivityAnalysis::new(
            "discount_rate".to_string(),
            0.05,
            1_000_000,
            0.1,
            1_100_000,
            900_000,
        )
        .unwrap();
        log.set_sensitivity_analysis(analysis).unwrap();

        assert_eq!(log.assess_reliability(), ReliabilityAssessment::High);
    }

    #[test]
    fn test_assess_reliability_low() {
        let id = JudgmentLogId::new();
        let params = HashMap::new(); // No parameters

        let log = JudgmentLog::new(
            id,
            JudgmentType::Impairment,
            "Minimal analysis".to_string(),
            params,
            Vec::new(), // No scenarios
            "USER001".to_string(),
        )
        .unwrap();

        assert_eq!(log.assess_reliability(), ReliabilityAssessment::Low);
    }
}
