// 固定資産台帳のイベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{
    AssetCategory, AssetStatus, ComponentId, DepreciationMethod, FixedAssetId, MeasurementModel,
    UsefulLife,
};
use crate::{common::Amount, event::DomainEvent};

/// 固定資産イベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedAssetEvent {
    pub asset_id: FixedAssetId,
    pub event_type: FixedAssetEventType,
    pub occurred_at: DateTime<Utc>,
    pub version: u64,
}

impl FixedAssetEvent {
    pub fn new(asset_id: FixedAssetId, event_type: FixedAssetEventType) -> Self {
        Self { asset_id, event_type, occurred_at: Utc::now(), version: 1 }
    }
}

impl DomainEvent for FixedAssetEvent {
    fn event_type(&self) -> &str {
        match &self.event_type {
            FixedAssetEventType::AssetRegistered { .. } => "AssetRegistered",
            FixedAssetEventType::ComponentAdded { .. } => "ComponentAdded",
            FixedAssetEventType::DepreciationRecorded { .. } => "DepreciationRecorded",
            FixedAssetEventType::AssetRevaluated { .. } => "AssetRevaluated",
            FixedAssetEventType::ImpairmentRecognized { .. } => "ImpairmentRecognized",
            FixedAssetEventType::ImpairmentReversed { .. } => "ImpairmentReversed",
            FixedAssetEventType::StatusChanged { .. } => "StatusChanged",
            FixedAssetEventType::AssetDisposed { .. } => "AssetDisposed",
            FixedAssetEventType::CguAssigned { .. } => "CguAssigned",
        }
    }

    fn aggregate_id(&self) -> &str {
        Box::leak(self.asset_id.to_string().into_boxed_str())
    }

    fn version(&self) -> u64 {
        self.version
    }
}

/// 固定資産イベントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixedAssetEventType {
    /// 資産登録
    AssetRegistered {
        category: AssetCategory,
        name: String,
        account_code: String,
        acquisition_date: DateTime<Utc>,
        acquisition_cost: Amount,
        measurement_model: MeasurementModel,
    },
    /// コンポーネント追加
    ComponentAdded {
        component_id: ComponentId,
        component_name: String,
        cost: Amount,
        useful_life: UsefulLife,
        depreciation_method: DepreciationMethod,
    },
    /// 償却記録
    DepreciationRecorded {
        component_id: ComponentId,
        depreciation_amount: Amount,
        accumulated_depreciation: Amount,
        period: String,
    },
    /// 再評価実施
    AssetRevaluated { old_amount: Amount, new_amount: Amount, revaluation_surplus: Amount },
    /// 減損損失計上
    ImpairmentRecognized { impairment_loss: Amount, recoverable_amount: Amount, reason: String },
    /// 減損戻入計上
    ImpairmentReversed { reversal_amount: Amount, reason: String },
    /// ステータス変更
    StatusChanged { old_status: AssetStatus, new_status: AssetStatus },
    /// 資産除却
    AssetDisposed { disposal_date: DateTime<Utc>, disposal_amount: Amount, carrying_amount: Amount },
    /// CGU割当
    CguAssigned { cgu: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_registered_event() {
        let asset_id = FixedAssetId::new();
        let event = FixedAssetEvent::new(
            asset_id.clone(),
            FixedAssetEventType::AssetRegistered {
                category: AssetCategory::TangibleAsset,
                name: "Test Asset".to_string(),
                account_code: "1000".to_string(),
                acquisition_date: Utc::now(),
                acquisition_cost: Amount::from_i64(1_000_000),
                measurement_model: MeasurementModel::CostModel,
            },
        );

        assert_eq!(event.event_type(), "AssetRegistered");
        assert_eq!(event.asset_id, asset_id);
    }

    #[test]
    fn test_component_added_event() {
        let asset_id = FixedAssetId::new();
        let component_id = ComponentId::new();
        let useful_life = UsefulLife::new(5, 0).unwrap();

        let event = FixedAssetEvent::new(
            asset_id,
            FixedAssetEventType::ComponentAdded {
                component_id,
                component_name: "Main Component".to_string(),
                cost: Amount::from_i64(800_000),
                useful_life,
                depreciation_method: DepreciationMethod::StraightLine,
            },
        );

        assert_eq!(event.event_type(), "ComponentAdded");
    }

    #[test]
    fn test_depreciation_recorded_event() {
        let asset_id = FixedAssetId::new();
        let component_id = ComponentId::new();

        let event = FixedAssetEvent::new(
            asset_id,
            FixedAssetEventType::DepreciationRecorded {
                component_id,
                depreciation_amount: Amount::from_i64(150_000),
                accumulated_depreciation: Amount::from_i64(150_000),
                period: "2024-01".to_string(),
            },
        );

        assert_eq!(event.event_type(), "DepreciationRecorded");
    }

    #[test]
    fn test_impairment_recognized_event() {
        let asset_id = FixedAssetId::new();

        let event = FixedAssetEvent::new(
            asset_id,
            FixedAssetEventType::ImpairmentRecognized {
                impairment_loss: Amount::from_i64(100_000),
                recoverable_amount: Amount::from_i64(900_000),
                reason: "Market value decline".to_string(),
            },
        );

        assert_eq!(event.event_type(), "ImpairmentRecognized");
    }

    #[test]
    fn test_status_changed_event() {
        let asset_id = FixedAssetId::new();

        let event = FixedAssetEvent::new(
            asset_id,
            FixedAssetEventType::StatusChanged {
                old_status: AssetStatus::InUse,
                new_status: AssetStatus::Idle,
            },
        );

        assert_eq!(event.event_type(), "StatusChanged");
    }
}
