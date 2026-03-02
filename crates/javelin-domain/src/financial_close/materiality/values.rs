// 重要性基準の値オブジェクト

use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::Amount,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 重要性判定ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MaterialityJudgmentId(Uuid);

impl MaterialityJudgmentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl Default for MaterialityJudgmentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MaterialityJudgmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for MaterialityJudgmentId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// 重要性区分
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialityType {
    /// 金額的重要性
    Quantitative,
    /// 質的重要性
    Qualitative,
    /// 見積重要性
    Estimate,
}

impl MaterialityType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Quantitative => "Quantitative",
            Self::Qualitative => "Qualitative",
            Self::Estimate => "Estimate",
        }
    }
}

impl fmt::Display for MaterialityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MaterialityType {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Quantitative" => Ok(Self::Quantitative),
            "Qualitative" => Ok(Self::Qualitative),
            "Estimate" => Ok(Self::Estimate),
            _ => Err(DomainError::InvalidMateriality),
        }
    }
}

/// 金額的重要性基準
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuantitativeThreshold {
    /// 税引前利益の5%
    pretax_income_threshold: Amount,
    /// 総資産の0.5%
    total_assets_threshold: Amount,
    /// 売上高の0.5%
    revenue_threshold: Amount,
    /// 純資産の1%
    equity_threshold: Amount,
}

impl QuantitativeThreshold {
    pub fn new(
        pretax_income: &Amount,
        total_assets: &Amount,
        revenue: &Amount,
        equity: &Amount,
    ) -> Self {
        // 各指標の閾値を計算
        let pretax_income_threshold = pretax_income * &Amount::from_i64(5) / &Amount::from_i64(100);
        let total_assets_threshold = total_assets * &Amount::from_i64(5) / &Amount::from_i64(1000);
        let revenue_threshold = revenue * &Amount::from_i64(5) / &Amount::from_i64(1000);
        let equity_threshold = equity * &Amount::from_i64(1) / &Amount::from_i64(100);

        Self {
            pretax_income_threshold,
            total_assets_threshold,
            revenue_threshold,
            equity_threshold,
        }
    }

    /// 最も低い閾値を返す（保守的な判定）
    pub fn lowest_threshold(&self) -> &Amount {
        let thresholds = vec![
            &self.pretax_income_threshold,
            &self.total_assets_threshold,
            &self.revenue_threshold,
            &self.equity_threshold,
        ];

        thresholds.into_iter().min().unwrap()
    }

    /// 金額が重要性基準を超えているか判定
    pub fn is_material(&self, amount: &Amount) -> bool {
        let threshold = self.lowest_threshold();
        amount.abs() >= *threshold
    }

    pub fn pretax_income_threshold(&self) -> &Amount {
        &self.pretax_income_threshold
    }

    pub fn total_assets_threshold(&self) -> &Amount {
        &self.total_assets_threshold
    }

    pub fn revenue_threshold(&self) -> &Amount {
        &self.revenue_threshold
    }

    pub fn equity_threshold(&self) -> &Amount {
        &self.equity_threshold
    }
}

impl ValueObject for QuantitativeThreshold {
    fn validate(&self) -> DomainResult<()> {
        if self.pretax_income_threshold.is_negative()
            || self.total_assets_threshold.is_negative()
            || self.revenue_threshold.is_negative()
            || self.equity_threshold.is_negative()
        {
            return Err(DomainError::InvalidMateriality);
        }
        Ok(())
    }
}

/// 質的重要性要因
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualitativeFactor {
    /// 会計方針変更
    AccountingPolicyChange,
    /// 関連当事者取引
    RelatedPartyTransaction,
    /// 法令違反
    LegalViolation,
    /// 訴訟・係争
    Litigation,
    /// 経営者の不正
    ManagementFraud,
    /// 継続企業の前提に関する重要な疑義
    GoingConcernUncertainty,
    /// 重要な後発事象
    SubsequentEvent,
    /// その他
    Other(String),
}

impl QualitativeFactor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AccountingPolicyChange => "AccountingPolicyChange",
            Self::RelatedPartyTransaction => "RelatedPartyTransaction",
            Self::LegalViolation => "LegalViolation",
            Self::Litigation => "Litigation",
            Self::ManagementFraud => "ManagementFraud",
            Self::GoingConcernUncertainty => "GoingConcernUncertainty",
            Self::SubsequentEvent => "SubsequentEvent",
            Self::Other(_) => "Other",
        }
    }

    /// 質的要因が常に重要とみなされるか
    pub fn is_always_material(&self) -> bool {
        matches!(
            self,
            Self::ManagementFraud | Self::LegalViolation | Self::GoingConcernUncertainty
        )
    }
}

impl fmt::Display for QualitativeFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(desc) => write!(f, "Other: {}", desc),
            _ => write!(f, "{}", self.as_str()),
        }
    }
}

/// 見積重要性パラメータ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EstimateParameter {
    /// パラメータ名
    name: String,
    /// 基準値
    base_value: Amount,
    /// 変動率（±10%）
    variation_rate: u32,
}

impl EstimateParameter {
    pub fn new(name: String, base_value: Amount) -> DomainResult<Self> {
        Ok(Self {
            name,
            base_value,
            variation_rate: 10, // デフォルト±10%
        })
    }

    pub fn with_variation_rate(
        name: String,
        base_value: Amount,
        variation_rate: u32,
    ) -> DomainResult<Self> {
        if variation_rate > 100 {
            return Err(DomainError::InvalidMateriality);
        }

        Ok(Self {
            name,
            base_value,
            variation_rate,
        })
    }

    /// 上限値を計算
    pub fn upper_bound(&self) -> Amount {
        let multiplier = Amount::from_i64(100 + self.variation_rate as i64);
        &self.base_value * &multiplier / &Amount::from_i64(100)
    }

    /// 下限値を計算
    pub fn lower_bound(&self) -> Amount {
        let multiplier = Amount::from_i64(100 - self.variation_rate as i64);
        &self.base_value * &multiplier / &Amount::from_i64(100)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn base_value(&self) -> &Amount {
        &self.base_value
    }

    pub fn variation_rate(&self) -> u32 {
        self.variation_rate
    }
}

impl ValueObject for EstimateParameter {
    fn validate(&self) -> DomainResult<()> {
        if self.name.is_empty() {
            return Err(DomainError::InvalidMateriality);
        }
        if self.variation_rate > 100 {
            return Err(DomainError::InvalidMateriality);
        }
        Ok(())
    }
}

/// 感度分析結果
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SensitivityAnalysisResult {
    /// パラメータ名
    parameter_name: String,
    /// 基準値での結果
    base_result: Amount,
    /// 上限値での結果
    upper_result: Amount,
    /// 下限値での結果
    lower_result: Amount,
    /// 最大影響額
    max_impact: Amount,
}

impl SensitivityAnalysisResult {
    pub fn new(
        parameter_name: String,
        base_result: Amount,
        upper_result: Amount,
        lower_result: Amount,
    ) -> Self {
        let upper_impact = (&upper_result - &base_result).abs();
        let lower_impact = (&lower_result - &base_result).abs();
        let max_impact = if upper_impact > lower_impact {
            upper_impact
        } else {
            lower_impact
        };

        Self {
            parameter_name,
            base_result,
            upper_result,
            lower_result,
            max_impact,
        }
    }

    pub fn parameter_name(&self) -> &str {
        &self.parameter_name
    }

    pub fn base_result(&self) -> &Amount {
        &self.base_result
    }

    pub fn upper_result(&self) -> &Amount {
        &self.upper_result
    }

    pub fn lower_result(&self) -> &Amount {
        &self.lower_result
    }

    pub fn max_impact(&self) -> &Amount {
        &self.max_impact
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_materiality_judgment_id_creation() {
        let id1 = MaterialityJudgmentId::new();
        let id2 = MaterialityJudgmentId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_materiality_type_from_str() {
        assert_eq!(
            "Quantitative".parse::<MaterialityType>().unwrap(),
            MaterialityType::Quantitative
        );
        assert_eq!(
            "Qualitative".parse::<MaterialityType>().unwrap(),
            MaterialityType::Qualitative
        );
        assert_eq!("Estimate".parse::<MaterialityType>().unwrap(), MaterialityType::Estimate);
        assert!("Invalid".parse::<MaterialityType>().is_err());
    }

    #[test]
    fn test_quantitative_threshold_calculation() {
        let pretax_income = Amount::from_i64(1_000_000_000); // 10億円
        let total_assets = Amount::from_i64(10_000_000_000); // 100億円
        let revenue = Amount::from_i64(5_000_000_000); // 50億円
        let equity = Amount::from_i64(3_000_000_000); // 30億円

        let threshold = QuantitativeThreshold::new(&pretax_income, &total_assets, &revenue, &equity);

        // 税引前利益の5% = 5,000万円
        assert_eq!(threshold.pretax_income_threshold().to_i64(), Some(50_000_000));
        // 総資産の0.5% = 5,000万円
        assert_eq!(threshold.total_assets_threshold().to_i64(), Some(50_000_000));
        // 売上高の0.5% = 2,500万円
        assert_eq!(threshold.revenue_threshold().to_i64(), Some(25_000_000));
        // 純資産の1% = 3,000万円
        assert_eq!(threshold.equity_threshold().to_i64(), Some(30_000_000));
    }

    #[test]
    fn test_quantitative_threshold_lowest() {
        let pretax_income = Amount::from_i64(1_000_000_000);
        let total_assets = Amount::from_i64(10_000_000_000);
        let revenue = Amount::from_i64(5_000_000_000);
        let equity = Amount::from_i64(3_000_000_000);

        let threshold = QuantitativeThreshold::new(&pretax_income, &total_assets, &revenue, &equity);

        // 最も低い閾値は売上高の0.5% = 2,500万円
        assert_eq!(threshold.lowest_threshold().to_i64(), Some(25_000_000));
    }

    #[test]
    fn test_quantitative_threshold_is_material() {
        let pretax_income = Amount::from_i64(1_000_000_000);
        let total_assets = Amount::from_i64(10_000_000_000);
        let revenue = Amount::from_i64(5_000_000_000);
        let equity = Amount::from_i64(3_000_000_000);

        let threshold = QuantitativeThreshold::new(&pretax_income, &total_assets, &revenue, &equity);

        // 3,000万円は重要（最低閾値2,500万円を超える）
        assert!(threshold.is_material(&Amount::from_i64(30_000_000)));

        // 2,000万円は重要でない（最低閾値2,500万円未満）
        assert!(!threshold.is_material(&Amount::from_i64(20_000_000)));

        // 負の金額も絶対値で判定
        assert!(threshold.is_material(&Amount::from_i64(-30_000_000)));
    }

    #[test]
    fn test_qualitative_factor_always_material() {
        assert!(QualitativeFactor::ManagementFraud.is_always_material());
        assert!(QualitativeFactor::LegalViolation.is_always_material());
        assert!(QualitativeFactor::GoingConcernUncertainty.is_always_material());
        assert!(!QualitativeFactor::AccountingPolicyChange.is_always_material());
        assert!(!QualitativeFactor::RelatedPartyTransaction.is_always_material());
    }

    #[test]
    fn test_estimate_parameter_bounds() {
        let param = EstimateParameter::new("割引率".to_string(), Amount::from_i64(1000)).unwrap();

        // ±10%の変動
        assert_eq!(param.upper_bound().to_i64(), Some(1100));
        assert_eq!(param.lower_bound().to_i64(), Some(900));
    }

    #[test]
    fn test_estimate_parameter_custom_variation() {
        let param = EstimateParameter::with_variation_rate(
            "為替レート".to_string(),
            Amount::from_i64(15000),
            20, // ±20%
        )
        .unwrap();

        assert_eq!(param.upper_bound().to_i64(), Some(18000));
        assert_eq!(param.lower_bound().to_i64(), Some(12000));
    }

    #[test]
    fn test_sensitivity_analysis_result() {
        let result = SensitivityAnalysisResult::new(
            "割引率".to_string(),
            Amount::from_i64(1_000_000),
            Amount::from_i64(1_100_000),
            Amount::from_i64(950_000),
        );

        assert_eq!(result.base_result().to_i64(), Some(1_000_000));
        assert_eq!(result.upper_result().to_i64(), Some(1_100_000));
        assert_eq!(result.lower_result().to_i64(), Some(950_000));
        // 最大影響額は上限との差（10万円）
        assert_eq!(result.max_impact().to_i64(), Some(100_000));
    }

    #[test]
    fn test_sensitivity_analysis_result_negative_impact() {
        let result = SensitivityAnalysisResult::new(
            "成長率".to_string(),
            Amount::from_i64(1_000_000),
            Amount::from_i64(1_050_000),
            Amount::from_i64(800_000),
        );

        // 最大影響額は下限との差（20万円）
        assert_eq!(result.max_impact().to_i64(), Some(200_000));
    }
}
