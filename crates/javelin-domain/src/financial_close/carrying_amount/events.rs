// 帳簿価額のイベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{CarryingAmountId, ComponentType, EstimateChange, MeasurementChange};
use crate::{common::Amount, event::DomainEvent};

/// 帳簿価額イベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarryingAmountEvent {
    pub carrying_amount_id: CarryingAmountId,
    pub event_type: CarryingAmountEventType,
    pub occurred_at: DateTime<Utc>,
    pub version: u64,
}

impl CarryingAmountEvent {
    pub fn new(carrying_amount_id: CarryingAmountId, event_type: CarryingAmountEventType) -> Self {
        Self { carrying_amount_id, event_type, occurred_at: Utc::now(), version: 1 }
    }

    pub fn with_version(
        carrying_amount_id: CarryingAmountId,
        event_type: CarryingAmountEventType,
        version: u64,
    ) -> Self {
        Self { carrying_amount_id, event_type, occurred_at: Utc::now(), version }
    }
}

impl DomainEvent for CarryingAmountEvent {
    fn event_type(&self) -> &str {
        match &self.event_type {
            CarryingAmountEventType::CarryingAmountCreated { .. } => "CarryingAmountCreated",
            CarryingAmountEventType::ComponentAdded { .. } => "ComponentAdded",
            CarryingAmountEventType::MeasurementChanged { .. } => "MeasurementChanged",
            CarryingAmountEventType::EstimateChanged { .. } => "EstimateChanged",
            CarryingAmountEventType::PresentationAmountSet { .. } => "PresentationAmountSet",
        }
    }

    fn aggregate_id(&self) -> &str {
        Box::leak(self.carrying_amount_id.to_string().into_boxed_str())
    }

    fn version(&self) -> u64 {
        self.version
    }
}

/// 帳簿価額イベントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CarryingAmountEventType {
    /// 帳簿価額作成
    CarryingAmountCreated { asset_liability_id: String, account_code: String },
    /// 構成要素追加
    ComponentAdded { component_type: ComponentType, amount: Amount },
    /// 測定変更
    MeasurementChanged { change: MeasurementChange },
    /// 見積変更
    EstimateChanged { change: EstimateChange },
    /// 表示額設定
    PresentationAmountSet { amount: Amount, reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::financial_close::carrying_amount::values::MeasurementBasis;

    #[test]
    fn test_carrying_amount_created_event() {
        let id = CarryingAmountId::new();
        let event = CarryingAmountEvent::new(
            id.clone(),
            CarryingAmountEventType::CarryingAmountCreated {
                asset_liability_id: "ASSET001".to_string(),
                account_code: "1500".to_string(),
            },
        );

        assert_eq!(event.event_type(), "CarryingAmountCreated");
        assert_eq!(event.carrying_amount_id, id);
        assert_eq!(event.version(), 1);
    }

    #[test]
    fn test_component_added_event() {
        let id = CarryingAmountId::new();
        let event = CarryingAmountEvent::new(
            id,
            CarryingAmountEventType::ComponentAdded {
                component_type: ComponentType::AcquisitionCost,
                amount: Amount::from_i64(1_000_000),
            },
        );

        assert_eq!(event.event_type(), "ComponentAdded");
    }

    #[test]
    fn test_measurement_changed_event() {
        let id = CarryingAmountId::new();
        let change = MeasurementChange::new(
            MeasurementBasis::HistoricalCost,
            MeasurementBasis::FairValue,
            "Change to fair value model".to_string(),
            true,
            false,
        )
        .unwrap();

        let event =
            CarryingAmountEvent::new(id, CarryingAmountEventType::MeasurementChanged { change });

        assert_eq!(event.event_type(), "MeasurementChanged");
    }
}
