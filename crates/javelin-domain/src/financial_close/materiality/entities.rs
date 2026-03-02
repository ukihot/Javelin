// 重要性基準のエンティティ

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{
    EstimateParameter, MaterialityJudgmentId, MaterialityType, QualitativeFactor,
    QuantitativeThreshold, SensitivityAnalysisResult,
};
use crate::{
    common::Amount,
    entity::Entity,
    error::{DomainError, DomainResult},
};

/// 重要性判定エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialityJudgment {
    id: MaterialityJudgmentId,
    materiality_type: MaterialityType,
    judgment_date: DateTime<Utc>,
    requires_adjustment: bool,
    requires_approval: bool,
    approval_level: ApprovalLevel,
    control_method: String,
    quantitative_threshold: Option<QuantitativeThreshold>,
    qualitative_factors: Vec<QualitativeFactor>,
    estimate_parameters: Vec<EstimateParameter>,
    sensitivity_results: Vec<SensitivityAnalysisResult>,
    judgment_basis: String,
    judged_by: String,
    approved_by: Option<String>,
    approval_date: Option<DateTime<Utc>>,
}

/// 承認レベル
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalLevel {
    /// 担当者レベル（調整不要）
    Staff,
    /// 課長レベル（軽微な調整）
    Manager,
    /// 部長レベル（重要な調整）
    Director,
    /// CFOレベル（極めて重要）
    CFO,
    /// 取締役会レベル（会計方針変更等）
    Board,
}

impl ApprovalLevel {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Staff => "Staff",
            Self::Manager => "Manager",
            Self::Director => "Director",
            Self::CFO => "CFO",
            Self::Board => "Board",
        }
    }
}

impl MaterialityJudgment {
    /// 金額的重要性判定を作成
    pub fn new_quantitative(
        threshold: QuantitativeThreshold,
        amount: &Amount,
        control_method: String,
        judged_by: String,
    ) -> DomainResult<Self> {
        let is_material = threshold.is_material(amount);
        let (requires_adjustment, approval_level) = if is_material {
            (true, Self::determine_approval_level_by_amount(amount, threshold.lowest_threshold()))
        } else {
            (false, ApprovalLevel::Staff)
        };

        Ok(Self {
            id: MaterialityJudgmentId::new(),
            materiality_type: MaterialityType::Quantitative,
            judgment_date: Utc::now(),
            requires_adjustment,
            requires_approval: is_material,
            approval_level,
            control_method,
            quantitative_threshold: Some(threshold),
            qualitative_factors: Vec::new(),
            estimate_parameters: Vec::new(),
            sensitivity_results: Vec::new(),
            judgment_basis: format!("金額的重要性判定: {}", if is_material { "重要" } else { "非重要" }),
            judged_by,
            approved_by: None,
            approval_date: None,
        })
    }

    /// 質的重要性判定を作成
    pub fn new_qualitative(
        factors: Vec<QualitativeFactor>,
        control_method: String,
        judged_by: String,
    ) -> DomainResult<Self> {
        if factors.is_empty() {
            return Err(DomainError::InvalidMateriality);
        }

        let has_always_material = factors.iter().any(|f| f.is_always_material());
        let approval_level = if has_always_material {
            ApprovalLevel::Board
        } else {
            ApprovalLevel::CFO
        };

        Ok(Self {
            id: MaterialityJudgmentId::new(),
            materiality_type: MaterialityType::Qualitative,
            judgment_date: Utc::now(),
            requires_adjustment: true,
            requires_approval: true,
            approval_level,
            control_method,
            quantitative_threshold: None,
            qualitative_factors: factors,
            estimate_parameters: Vec::new(),
            sensitivity_results: Vec::new(),
            judgment_basis: "質的重要性判定: 重要".to_string(),
            judged_by,
            approved_by: None,
            approval_date: None,
        })
    }

    /// 見積重要性判定を作成
    pub fn new_estimate(
        parameters: Vec<EstimateParameter>,
        control_method: String,
        judged_by: String,
    ) -> DomainResult<Self> {
        if parameters.is_empty() {
            return Err(DomainError::InvalidMateriality);
        }

        Ok(Self {
            id: MaterialityJudgmentId::new(),
            materiality_type: MaterialityType::Estimate,
            judgment_date: Utc::now(),
            requires_adjustment: false, // 感度分析後に決定
            requires_approval: false,
            approval_level: ApprovalLevel::Manager,
            control_method,
            quantitative_threshold: None,
            qualitative_factors: Vec::new(),
            estimate_parameters: parameters,
            sensitivity_results: Vec::new(),
            judgment_basis: "見積重要性判定: 感度分析実施中".to_string(),
            judged_by,
            approved_by: None,
            approval_date: None,
        })
    }

    /// 感度分析結果を追加
    pub fn add_sensitivity_result(&mut self, result: SensitivityAnalysisResult) -> DomainResult<()> {
        if self.materiality_type != MaterialityType::Estimate {
            return Err(DomainError::InvalidMateriality);
        }

        self.sensitivity_results.push(result);
        Ok(())
    }

    /// 感度分析に基づいて重要性を判定
    pub fn evaluate_sensitivity(
        &mut self,
        quantitative_threshold: &QuantitativeThreshold,
    ) -> DomainResult<()> {
        if self.materiality_type != MaterialityType::Estimate {
            return Err(DomainError::InvalidMateriality);
        }

        if self.sensitivity_results.is_empty() {
            return Err(DomainError::InvalidMateriality);
        }

        // 最大影響額を確認
        let max_impact = self
            .sensitivity_results
            .iter()
            .map(|r| r.max_impact())
            .max()
            .unwrap();

        let is_material = quantitative_threshold.is_material(max_impact);
        self.requires_adjustment = is_material;
        self.requires_approval = is_material;

        if is_material {
            self.approval_level =
                Self::determine_approval_level_by_amount(max_impact, quantitative_threshold.lowest_threshold());
            self.judgment_basis = format!("見積重要性判定: 重要（最大影響額: {}）", max_impact);
        } else {
            self.approval_level = ApprovalLevel::Manager;
            self.judgment_basis = format!("見積重要性判定: 非重要（最大影響額: {}）", max_impact);
        }

        Ok(())
    }

    /// 承認を記録
    pub fn approve(&mut self, approver: String) -> DomainResult<()> {
        if !self.requires_approval {
            return Err(DomainError::InvalidMateriality);
        }

        if self.approved_by.is_some() {
            return Err(DomainError::InvalidMateriality);
        }

        self.approved_by = Some(approver);
        self.approval_date = Some(Utc::now());
        Ok(())
    }

    /// 金額に基づいて承認レベルを決定
    fn determine_approval_level_by_amount(amount: &Amount, threshold: &Amount) -> ApprovalLevel {
        let ratio = if let (Some(amt), Some(thr)) = (amount.abs().to_f64(), threshold.to_f64()) {
            amt / thr
        } else {
            1.0
        };

        if ratio >= 10.0 {
            ApprovalLevel::Board
        } else if ratio >= 5.0 {
            ApprovalLevel::CFO
        } else if ratio >= 2.0 {
            ApprovalLevel::Director
        } else {
            ApprovalLevel::Manager
        }
    }

    // Getters
    pub fn id(&self) -> &MaterialityJudgmentId {
        &self.id
    }

    pub fn materiality_type(&self) -> &MaterialityType {
        &self.materiality_type
    }

    pub fn judgment_date(&self) -> &DateTime<Utc> {
        &self.judgment_date
    }

    pub fn requires_adjustment(&self) -> bool {
        self.requires_adjustment
    }

    pub fn requires_approval(&self) -> bool {
        self.requires_approval
    }

    pub fn approval_level(&self) -> &ApprovalLevel {
        &self.approval_level
    }

    pub fn control_method(&self) -> &str {
        &self.control_method
    }

    pub fn quantitative_threshold(&self) -> Option<&QuantitativeThreshold> {
        self.quantitative_threshold.as_ref()
    }

    pub fn qualitative_factors(&self) -> &[QualitativeFactor] {
        &self.qualitative_factors
    }

    pub fn estimate_parameters(&self) -> &[EstimateParameter] {
        &self.estimate_parameters
    }

    pub fn sensitivity_results(&self) -> &[SensitivityAnalysisResult] {
        &self.sensitivity_results
    }

    pub fn judgment_basis(&self) -> &str {
        &self.judgment_basis
    }

    pub fn judged_by(&self) -> &str {
        &self.judged_by
    }

    pub fn approved_by(&self) -> Option<&str> {
        self.approved_by.as_deref()
    }

    pub fn approval_date(&self) -> Option<&DateTime<Utc>> {
        self.approval_date.as_ref()
    }

    pub fn is_approved(&self) -> bool {
        self.approved_by.is_some()
    }
}

impl Entity for MaterialityJudgment {
    type Id = MaterialityJudgmentId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_threshold() -> QuantitativeThreshold {
        QuantitativeThreshold::new(
            &Amount::from_i64(1_000_000_000), // 税引前利益10億円
            &Amount::from_i64(10_000_000_000), // 総資産100億円
            &Amount::from_i64(5_000_000_000), // 売上高50億円
            &Amount::from_i64(3_000_000_000), // 純資産30億円
        )
    }

    #[test]
    fn test_quantitative_judgment_material() {
        let threshold = create_test_threshold();
        let amount = Amount::from_i64(30_000_000); // 3,000万円（重要）

        let judgment = MaterialityJudgment::new_quantitative(
            threshold,
            &amount,
            "補正仕訳起票".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        assert_eq!(judgment.materiality_type(), &MaterialityType::Quantitative);
        assert!(judgment.requires_adjustment());
        assert!(judgment.requires_approval());
        assert_eq!(judgment.approval_level(), &ApprovalLevel::Manager);
    }

    #[test]
    fn test_quantitative_judgment_not_material() {
        let threshold = create_test_threshold();
        let amount = Amount::from_i64(20_000_000); // 2,000万円（非重要）

        let judgment = MaterialityJudgment::new_quantitative(
            threshold,
            &amount,
            "調整不要".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        assert!(!judgment.requires_adjustment());
        assert!(!judgment.requires_approval());
        assert_eq!(judgment.approval_level(), &ApprovalLevel::Staff);
    }

    #[test]
    fn test_quantitative_judgment_high_amount() {
        let threshold = create_test_threshold();
        let amount = Amount::from_i64(300_000_000); // 3億円（閾値の10倍以上）

        let judgment = MaterialityJudgment::new_quantitative(
            threshold,
            &amount,
            "重要な補正".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        assert!(judgment.requires_adjustment());
        assert!(judgment.requires_approval());
        assert_eq!(judgment.approval_level(), &ApprovalLevel::Board);
    }

    #[test]
    fn test_qualitative_judgment_always_material() {
        let factors = vec![QualitativeFactor::ManagementFraud];

        let judgment = MaterialityJudgment::new_qualitative(
            factors,
            "特別調査実施".to_string(),
            "内部監査部".to_string(),
        )
        .unwrap();

        assert_eq!(judgment.materiality_type(), &MaterialityType::Qualitative);
        assert!(judgment.requires_adjustment());
        assert!(judgment.requires_approval());
        assert_eq!(judgment.approval_level(), &ApprovalLevel::Board);
    }

    #[test]
    fn test_qualitative_judgment_normal() {
        let factors = vec![QualitativeFactor::AccountingPolicyChange];

        let judgment = MaterialityJudgment::new_qualitative(
            factors,
            "会計方針変更の開示".to_string(),
            "経理部長".to_string(),
        )
        .unwrap();

        assert!(judgment.requires_adjustment());
        assert!(judgment.requires_approval());
        assert_eq!(judgment.approval_level(), &ApprovalLevel::CFO);
    }

    #[test]
    fn test_estimate_judgment_with_sensitivity() {
        let params = vec![EstimateParameter::new("割引率".to_string(), Amount::from_i64(1000)).unwrap()];

        let mut judgment = MaterialityJudgment::new_estimate(
            params,
            "感度分析実施".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        assert_eq!(judgment.materiality_type(), &MaterialityType::Estimate);
        assert!(!judgment.requires_adjustment()); // 初期状態

        // 感度分析結果を追加
        let sensitivity = SensitivityAnalysisResult::new(
            "割引率".to_string(),
            Amount::from_i64(1_000_000),
            Amount::from_i64(1_100_000),
            Amount::from_i64(950_000),
        );

        judgment.add_sensitivity_result(sensitivity).unwrap();

        // 重要性を評価
        let threshold = create_test_threshold();
        judgment.evaluate_sensitivity(&threshold).unwrap();

        // 影響額10万円は非重要
        assert!(!judgment.requires_adjustment());
        assert!(!judgment.requires_approval());
    }

    #[test]
    fn test_estimate_judgment_material_impact() {
        let params = vec![EstimateParameter::new("為替レート".to_string(), Amount::from_i64(15000)).unwrap()];

        let mut judgment = MaterialityJudgment::new_estimate(
            params,
            "感度分析実施".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        // 大きな影響額の感度分析結果
        let sensitivity = SensitivityAnalysisResult::new(
            "為替レート".to_string(),
            Amount::from_i64(1_000_000_000),
            Amount::from_i64(1_050_000_000),
            Amount::from_i64(950_000_000),
        );

        judgment.add_sensitivity_result(sensitivity).unwrap();

        let threshold = create_test_threshold();
        judgment.evaluate_sensitivity(&threshold).unwrap();

        // 影響額5,000万円は重要
        assert!(judgment.requires_adjustment());
        assert!(judgment.requires_approval());
    }

    #[test]
    fn test_approval_process() {
        let threshold = create_test_threshold();
        let amount = Amount::from_i64(30_000_000);

        let mut judgment = MaterialityJudgment::new_quantitative(
            threshold,
            &amount,
            "補正仕訳起票".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        assert!(!judgment.is_approved());
        assert!(judgment.approved_by().is_none());

        // 承認
        judgment.approve("課長B".to_string()).unwrap();

        assert!(judgment.is_approved());
        assert_eq!(judgment.approved_by(), Some("課長B"));
        assert!(judgment.approval_date().is_some());

        // 二重承認はエラー
        assert!(judgment.approve("部長C".to_string()).is_err());
    }

    #[test]
    fn test_approval_not_required() {
        let threshold = create_test_threshold();
        let amount = Amount::from_i64(20_000_000); // 非重要

        let mut judgment = MaterialityJudgment::new_quantitative(
            threshold,
            &amount,
            "調整不要".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        // 承認不要な判定に承認しようとするとエラー
        assert!(judgment.approve("課長B".to_string()).is_err());
    }

    #[test]
    fn test_approval_level_determination() {
        let threshold = create_test_threshold();

        // 閾値の2倍 → Director
        let amount1 = Amount::from_i64(50_000_000);
        let judgment1 = MaterialityJudgment::new_quantitative(
            threshold.clone(),
            &amount1,
            "補正".to_string(),
            "担当者".to_string(),
        )
        .unwrap();
        assert_eq!(judgment1.approval_level(), &ApprovalLevel::Director);

        // 閾値の5倍 → CFO
        let amount2 = Amount::from_i64(125_000_000);
        let judgment2 = MaterialityJudgment::new_quantitative(
            threshold.clone(),
            &amount2,
            "補正".to_string(),
            "担当者".to_string(),
        )
        .unwrap();
        assert_eq!(judgment2.approval_level(), &ApprovalLevel::CFO);

        // 閾値の10倍 → Board
        let amount3 = Amount::from_i64(250_000_000);
        let judgment3 = MaterialityJudgment::new_quantitative(
            threshold,
            &amount3,
            "補正".to_string(),
            "担当者".to_string(),
        )
        .unwrap();
        assert_eq!(judgment3.approval_level(), &ApprovalLevel::Board);
    }
}
