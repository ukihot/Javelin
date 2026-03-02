// 収益認識のイベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{ContractId, PerformanceObligationId, TransactionPrice};
use crate::{common::Amount, event::DomainEvent};

/// 収益認識イベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueRecognitionEvent {
    pub contract_id: ContractId,
    pub event_type: RevenueRecognitionEventType,
    pub occurred_at: DateTime<Utc>,
    pub version: u64,
}

impl RevenueRecognitionEvent {
    pub fn new(contract_id: ContractId, event_type: RevenueRecognitionEventType) -> Self {
        Self { contract_id, event_type, occurred_at: Utc::now(), version: 1 }
    }

    pub fn with_version(
        contract_id: ContractId,
        event_type: RevenueRecognitionEventType,
        version: u64,
    ) -> Self {
        Self { contract_id, event_type, occurred_at: Utc::now(), version }
    }
}

impl DomainEvent for RevenueRecognitionEvent {
    fn event_type(&self) -> &str {
        match &self.event_type {
            RevenueRecognitionEventType::ContractIdentified { .. } => "ContractIdentified",
            RevenueRecognitionEventType::PerformanceObligationAdded { .. } => {
                "PerformanceObligationAdded"
            }
            RevenueRecognitionEventType::TransactionPriceAllocated { .. } => {
                "TransactionPriceAllocated"
            }
            RevenueRecognitionEventType::RevenueRecognized { .. } => "RevenueRecognized",
            RevenueRecognitionEventType::ContractModified { .. } => "ContractModified",
            RevenueRecognitionEventType::ContractCompleted { .. } => "ContractCompleted",
            RevenueRecognitionEventType::ContractsCombined { .. } => "ContractsCombined",
        }
    }

    fn aggregate_id(&self) -> &str {
        Box::leak(self.contract_id.to_string().into_boxed_str())
    }

    fn version(&self) -> u64 {
        self.version
    }
}

/// 収益認識イベントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RevenueRecognitionEventType {
    /// 契約識別（Step 1）
    ContractIdentified {
        customer_id: String,
        contract_date: DateTime<Utc>,
        transaction_price: TransactionPrice,
    },
    /// 履行義務追加（Step 2）
    PerformanceObligationAdded {
        obligation_id: PerformanceObligationId,
        description: String,
        standalone_selling_price: Amount,
        is_distinct: bool,
    },
    /// 取引価格配分（Step 4）
    TransactionPriceAllocated { allocations: Vec<(PerformanceObligationId, Amount)> },
    /// 収益認識（Step 5）
    RevenueRecognized {
        obligation_id: PerformanceObligationId,
        amount: Amount,
        recognition_date: DateTime<Utc>,
    },
    /// 契約変更（Step 3）
    ContractModified { old_price: Amount, new_price: Amount, modification_date: DateTime<Utc> },
    /// 契約完了
    ContractCompleted { completion_date: DateTime<Utc>, total_revenue: Amount },
    /// 契約結合
    ContractsCombined { combined_from: Vec<ContractId> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_identified_event() {
        let contract_id = ContractId::new();
        let transaction_price = TransactionPrice::new(
            Amount::from_i64(1_000_000),
            Amount::zero(),
            Amount::zero(),
            Amount::zero(),
        )
        .unwrap();

        let event = RevenueRecognitionEvent::new(
            contract_id.clone(),
            RevenueRecognitionEventType::ContractIdentified {
                customer_id: "CUST001".to_string(),
                contract_date: Utc::now(),
                transaction_price,
            },
        );

        assert_eq!(event.event_type(), "ContractIdentified");
        assert_eq!(event.contract_id, contract_id);
    }

    #[test]
    fn test_revenue_recognized_event() {
        let contract_id = ContractId::new();
        let obligation_id = PerformanceObligationId::new();

        let event = RevenueRecognitionEvent::new(
            contract_id,
            RevenueRecognitionEventType::RevenueRecognized {
                obligation_id,
                amount: Amount::from_i64(500_000),
                recognition_date: Utc::now(),
            },
        );

        assert_eq!(event.event_type(), "RevenueRecognized");
    }
}
