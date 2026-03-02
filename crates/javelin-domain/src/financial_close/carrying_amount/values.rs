// 帳簿価額の値オブジェクト

use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::Amount,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 帳簿価額ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CarryingAmountId(Uuid);

impl CarryingAmountId {
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

impl Default for CarryingAmountId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CarryingAmountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for CarryingAmountId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// 測定基礎
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeasurementBasis {
    /// 歴史的原価
    HistoricalCost,
    /// 現在原価
    CurrentCost,
    /// 実現可能価額
    RealizableValue,
    /// 現在価値
    PresentValue,
    /// 公正価値
    FairValue,
    /// 使用価値
    ValueInUse,
}

impl MeasurementBasis {
    pub fn as_str(&self) -> &str {
        match self {
            Self::HistoricalCost => "HistoricalCost",
            Self::CurrentCost => "CurrentCost",
            Self::RealizableValue => "RealizableValue",
            Self::PresentValue => "PresentValue",
            Self::FairValue => "FairValue",
            Self::ValueInUse => "ValueInUse",
        }
    }
}

impl fmt::Display for MeasurementBasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MeasurementBasis {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HistoricalCost" => Ok(Self::HistoricalCost),
            "CurrentCost" => Ok(Self::CurrentCost),
            "RealizableValue" => Ok(Self::RealizableValue),
            "PresentValue" => Ok(Self::PresentValue),
            "FairValue" => Ok(Self::FairValue),
            "ValueInUse" => Ok(Self::ValueInUse),
            _ => Err(DomainError::InvalidMeasurementBasis),
        }
    }
}

/// 構成要素タイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    /// 取得原価
    AcquisitionCost,
    /// 償却累計額
    AccumulatedDepreciation,
    /// 減損損失累計額
    AccumulatedImpairmentLoss,
    /// 減損戻入累計額
    AccumulatedImpairmentReversal,
    /// 再評価差額
    RevaluationSurplus,
    /// 公正価値測定差額
    FairValueAdjustment,
    /// その他の調整
    OtherAdjustment,
}

impl ComponentType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AcquisitionCost => "AcquisitionCost",
            Self::AccumulatedDepreciation => "AccumulatedDepreciation",
            Self::AccumulatedImpairmentLoss => "AccumulatedImpairmentLoss",
            Self::AccumulatedImpairmentReversal => "AccumulatedImpairmentReversal",
            Self::RevaluationSurplus => "RevaluationSurplus",
            Self::FairValueAdjustment => "FairValueAdjustment",
            Self::OtherAdjustment => "OtherAdjustment",
        }
    }

    /// 帳簿価額への影響（加算=true, 減算=false）
    pub fn is_additive(&self) -> bool {
        matches!(
            self,
            Self::AcquisitionCost
                | Self::AccumulatedImpairmentReversal
                | Self::RevaluationSurplus
                | Self::FairValueAdjustment
        )
    }
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ComponentType {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AcquisitionCost" => Ok(Self::AcquisitionCost),
            "AccumulatedDepreciation" => Ok(Self::AccumulatedDepreciation),
            "AccumulatedImpairmentLoss" => Ok(Self::AccumulatedImpairmentLoss),
            "AccumulatedImpairmentReversal" => Ok(Self::AccumulatedImpairmentReversal),
            "RevaluationSurplus" => Ok(Self::RevaluationSurplus),
            "FairValueAdjustment" => Ok(Self::FairValueAdjustment),
            "OtherAdjustment" => Ok(Self::OtherAdjustment),
            _ => Err(DomainError::InvalidComponentType),
        }
    }
}

/// 測定変更
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeasurementChange {
    /// 変更日
    change_date: DateTime<Utc>,
    /// 変更前測定基礎
    old_basis: MeasurementBasis,
    /// 変更後測定基礎
    new_basis: MeasurementBasis,
    /// 変更理由
    reason: String,
    /// 会計方針変更か
    is_policy_change: bool,
    /// 遡及適用するか
    retrospective_application: bool,
}

impl MeasurementChange {
    pub fn new(
        old_basis: MeasurementBasis,
        new_basis: MeasurementBasis,
        reason: String,
        is_policy_change: bool,
        retrospective_application: bool,
    ) -> DomainResult<Self> {
        if reason.is_empty() {
            return Err(DomainError::InvalidMeasurementChange);
        }

        Ok(Self {
            change_date: Utc::now(),
            old_basis,
            new_basis,
            reason,
            is_policy_change,
            retrospective_application,
        })
    }

    pub fn change_date(&self) -> DateTime<Utc> {
        self.change_date
    }

    pub fn old_basis(&self) -> &MeasurementBasis {
        &self.old_basis
    }

    pub fn new_basis(&self) -> &MeasurementBasis {
        &self.new_basis
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn is_policy_change(&self) -> bool {
        self.is_policy_change
    }

    pub fn retrospective_application(&self) -> bool {
        self.retrospective_application
    }
}

impl ValueObject for MeasurementChange {
    fn validate(&self) -> DomainResult<()> {
        if self.reason.is_empty() {
            return Err(DomainError::InvalidMeasurementChange);
        }
        Ok(())
    }
}

/// 見積変更
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EstimateChange {
    /// 変更日
    change_date: DateTime<Utc>,
    /// 変更対象
    target: String,
    /// 変更前見積値
    old_estimate: Amount,
    /// 変更後見積値
    new_estimate: Amount,
    /// 変更理由
    reason: String,
    /// 将来に向かって適用
    prospective_application: bool,
}

impl EstimateChange {
    pub fn new(
        target: String,
        old_estimate: Amount,
        new_estimate: Amount,
        reason: String,
    ) -> DomainResult<Self> {
        if target.is_empty() || reason.is_empty() {
            return Err(DomainError::InvalidEstimateChange);
        }

        Ok(Self {
            change_date: Utc::now(),
            target,
            old_estimate,
            new_estimate,
            reason,
            prospective_application: true, // 見積変更は常に将来に向かって適用
        })
    }

    pub fn change_date(&self) -> DateTime<Utc> {
        self.change_date
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn old_estimate(&self) -> &Amount {
        &self.old_estimate
    }

    pub fn new_estimate(&self) -> &Amount {
        &self.new_estimate
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn prospective_application(&self) -> bool {
        self.prospective_application
    }
}

impl ValueObject for EstimateChange {
    fn validate(&self) -> DomainResult<()> {
        if self.target.is_empty() || self.reason.is_empty() {
            return Err(DomainError::InvalidEstimateChange);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carrying_amount_id_creation() {
        let id1 = CarryingAmountId::new();
        let id2 = CarryingAmountId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_measurement_basis_from_str() {
        assert_eq!(
            "HistoricalCost".parse::<MeasurementBasis>().unwrap(),
            MeasurementBasis::HistoricalCost
        );
        assert_eq!("FairValue".parse::<MeasurementBasis>().unwrap(), MeasurementBasis::FairValue);
    }

    #[test]
    fn test_component_type_is_additive() {
        assert!(ComponentType::AcquisitionCost.is_additive());
        assert!(!ComponentType::AccumulatedDepreciation.is_additive());
        assert!(!ComponentType::AccumulatedImpairmentLoss.is_additive());
        assert!(ComponentType::AccumulatedImpairmentReversal.is_additive());
    }

    #[test]
    fn test_measurement_change_creation() {
        let change = MeasurementChange::new(
            MeasurementBasis::HistoricalCost,
            MeasurementBasis::FairValue,
            "Change to fair value model".to_string(),
            true,
            false,
        )
        .unwrap();

        assert_eq!(change.old_basis(), &MeasurementBasis::HistoricalCost);
        assert_eq!(change.new_basis(), &MeasurementBasis::FairValue);
        assert!(change.is_policy_change());
        assert!(!change.retrospective_application());
    }

    #[test]
    fn test_measurement_change_invalid_reason() {
        let result = MeasurementChange::new(
            MeasurementBasis::HistoricalCost,
            MeasurementBasis::FairValue,
            "".to_string(),
            true,
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_estimate_change_creation() {
        let change = EstimateChange::new(
            "Useful life".to_string(),
            Amount::from_i64(10),
            Amount::from_i64(8),
            "Revised based on actual usage".to_string(),
        )
        .unwrap();

        assert_eq!(change.target(), "Useful life");
        assert_eq!(change.old_estimate().to_i64(), Some(10));
        assert_eq!(change.new_estimate().to_i64(), Some(8));
        assert!(change.prospective_application());
    }

    #[test]
    fn test_estimate_change_validation() {
        let change = EstimateChange::new(
            "Useful life".to_string(),
            Amount::from_i64(10),
            Amount::from_i64(8),
            "Revised based on actual usage".to_string(),
        )
        .unwrap();

        assert!(change.validate().is_ok());
    }
}
