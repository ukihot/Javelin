// 固定資産台帳の値オブジェクト

use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 固定資産ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FixedAssetId(Uuid);

impl FixedAssetId {
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

impl Default for FixedAssetId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FixedAssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for FixedAssetId {
    fn value(&self) -> &str {
        // UUIDを文字列として返すため、一時的な文字列を作成
        // 注: これは非効率的だが、EntityIdトレイトの制約上必要
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// コンポーネントID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(Uuid);

impl ComponentId {
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

impl Default for ComponentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for ComponentId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// 資産区分（IAS 16, IAS 38, IFRS 16）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetCategory {
    /// 有形固定資産（IAS 16）
    TangibleAsset,
    /// 無形資産（IAS 38）
    IntangibleAsset,
    /// 使用権資産（IFRS 16）
    RightOfUseAsset,
    /// 建設仮勘定
    ConstructionInProgress,
}

impl AssetCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TangibleAsset => "TangibleAsset",
            Self::IntangibleAsset => "IntangibleAsset",
            Self::RightOfUseAsset => "RightOfUseAsset",
            Self::ConstructionInProgress => "ConstructionInProgress",
        }
    }

    pub fn is_depreciable(&self) -> bool {
        !matches!(self, Self::ConstructionInProgress)
    }
}

impl fmt::Display for AssetCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for AssetCategory {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TangibleAsset" => Ok(Self::TangibleAsset),
            "IntangibleAsset" => Ok(Self::IntangibleAsset),
            "RightOfUseAsset" => Ok(Self::RightOfUseAsset),
            "ConstructionInProgress" => Ok(Self::ConstructionInProgress),
            _ => Err(DomainError::InvalidAssetCategory),
        }
    }
}

/// 測定モデル（IAS 16, IAS 38）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeasurementModel {
    /// 原価モデル
    CostModel,
    /// 再評価モデル
    RevaluationModel,
}

impl MeasurementModel {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CostModel => "CostModel",
            Self::RevaluationModel => "RevaluationModel",
        }
    }
}

impl fmt::Display for MeasurementModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MeasurementModel {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CostModel" => Ok(Self::CostModel),
            "RevaluationModel" => Ok(Self::RevaluationModel),
            _ => Err(DomainError::InvalidMeasurementModel),
        }
    }
}

/// 償却方法
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepreciationMethod {
    /// 定額法
    StraightLine,
    /// 定率法
    DecliningBalance,
    /// 生産高比例法
    UnitsOfProduction,
}

impl DepreciationMethod {
    pub fn as_str(&self) -> &str {
        match self {
            Self::StraightLine => "StraightLine",
            Self::DecliningBalance => "DecliningBalance",
            Self::UnitsOfProduction => "UnitsOfProduction",
        }
    }
}

impl fmt::Display for DepreciationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for DepreciationMethod {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "StraightLine" => Ok(Self::StraightLine),
            "DecliningBalance" => Ok(Self::DecliningBalance),
            "UnitsOfProduction" => Ok(Self::UnitsOfProduction),
            _ => Err(DomainError::InvalidDepreciationMethod),
        }
    }
}

/// 耐用年数
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsefulLife {
    years: u32,
    months: u8,
}

impl UsefulLife {
    pub fn new(years: u32, months: u8) -> DomainResult<Self> {
        if months >= 12 {
            return Err(DomainError::InvalidUsefulLife);
        }
        if years == 0 && months == 0 {
            return Err(DomainError::InvalidUsefulLife);
        }
        Ok(Self { years, months })
    }

    pub fn years(&self) -> u32 {
        self.years
    }

    pub fn months(&self) -> u8 {
        self.months
    }

    pub fn total_months(&self) -> u32 {
        self.years * 12 + u32::from(self.months)
    }
}

impl ValueObject for UsefulLife {
    fn validate(&self) -> DomainResult<()> {
        if self.months >= 12 {
            return Err(DomainError::InvalidUsefulLife);
        }
        if self.years == 0 && self.months == 0 {
            return Err(DomainError::InvalidUsefulLife);
        }
        Ok(())
    }
}

/// 資産ステータス
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetStatus {
    /// 使用中
    InUse,
    /// 遊休
    Idle,
    /// 処分予定
    HeldForDisposal,
    /// 除却済
    Disposed,
}

impl AssetStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::InUse => "InUse",
            Self::Idle => "Idle",
            Self::HeldForDisposal => "HeldForDisposal",
            Self::Disposed => "Disposed",
        }
    }

    pub fn can_depreciate(&self) -> bool {
        matches!(self, Self::InUse)
    }
}

impl fmt::Display for AssetStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for AssetStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "InUse" => Ok(Self::InUse),
            "Idle" => Ok(Self::Idle),
            "HeldForDisposal" => Ok(Self::HeldForDisposal),
            "Disposed" => Ok(Self::Disposed),
            _ => Err(DomainError::InvalidAssetStatus),
        }
    }
}

/// 取得日
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcquisitionDate(DateTime<Utc>);

impl AcquisitionDate {
    pub fn new(date: DateTime<Utc>) -> Self {
        Self(date)
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl ValueObject for AcquisitionDate {
    fn validate(&self) -> DomainResult<()> {
        // 未来日付のチェック
        if self.0 > Utc::now() {
            return Err(DomainError::InvalidAcquisitionDate);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_asset_id_creation() {
        let id1 = FixedAssetId::new();
        let id2 = FixedAssetId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_component_id_creation() {
        let id1 = ComponentId::new();
        let id2 = ComponentId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_asset_category_from_str() {
        assert_eq!("TangibleAsset".parse::<AssetCategory>().unwrap(), AssetCategory::TangibleAsset);
        assert_eq!(
            "IntangibleAsset".parse::<AssetCategory>().unwrap(),
            AssetCategory::IntangibleAsset
        );
        assert!("Invalid".parse::<AssetCategory>().is_err());
    }

    #[test]
    fn test_asset_category_is_depreciable() {
        assert!(AssetCategory::TangibleAsset.is_depreciable());
        assert!(AssetCategory::IntangibleAsset.is_depreciable());
        assert!(AssetCategory::RightOfUseAsset.is_depreciable());
        assert!(!AssetCategory::ConstructionInProgress.is_depreciable());
    }

    #[test]
    fn test_measurement_model_from_str() {
        assert_eq!("CostModel".parse::<MeasurementModel>().unwrap(), MeasurementModel::CostModel);
        assert_eq!(
            "RevaluationModel".parse::<MeasurementModel>().unwrap(),
            MeasurementModel::RevaluationModel
        );
    }

    #[test]
    fn test_depreciation_method_from_str() {
        assert_eq!(
            "StraightLine".parse::<DepreciationMethod>().unwrap(),
            DepreciationMethod::StraightLine
        );
        assert_eq!(
            "DecliningBalance".parse::<DepreciationMethod>().unwrap(),
            DepreciationMethod::DecliningBalance
        );
    }

    #[test]
    fn test_useful_life_valid() {
        let life = UsefulLife::new(5, 6).unwrap();
        assert_eq!(life.years(), 5);
        assert_eq!(life.months(), 6);
        assert_eq!(life.total_months(), 66);
    }

    #[test]
    fn test_useful_life_invalid_months() {
        assert!(UsefulLife::new(5, 12).is_err());
        assert!(UsefulLife::new(5, 13).is_err());
    }

    #[test]
    fn test_useful_life_zero() {
        assert!(UsefulLife::new(0, 0).is_err());
    }

    #[test]
    fn test_asset_status_can_depreciate() {
        assert!(AssetStatus::InUse.can_depreciate());
        assert!(!AssetStatus::Idle.can_depreciate());
        assert!(!AssetStatus::HeldForDisposal.can_depreciate());
        assert!(!AssetStatus::Disposed.can_depreciate());
    }

    #[test]
    fn test_acquisition_date_validation() {
        let past_date = Utc::now() - chrono::Duration::days(1);
        let acquisition_date = AcquisitionDate::new(past_date);
        assert!(acquisition_date.validate().is_ok());

        let future_date = Utc::now() + chrono::Duration::days(1);
        let acquisition_date = AcquisitionDate::new(future_date);
        assert!(acquisition_date.validate().is_err());
    }
}
