// 外貨換算のイベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{Currency, ForeignCurrencyTransactionId, MonetaryClassification};
use crate::{common::Amount, event::DomainEvent};

/// 外貨換算イベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignCurrencyEvent {
    pub transaction_id: ForeignCurrencyTransactionId,
    pub event_type: ForeignCurrencyEventType,
    pub occurred_at: DateTime<Utc>,
    pub version: u64,
}

impl ForeignCurrencyEvent {
    pub fn new(
        transaction_id: ForeignCurrencyTransactionId,
        event_type: ForeignCurrencyEventType,
    ) -> Self {
        Self { transaction_id, event_type, occurred_at: Utc::now(), version: 1 }
    }

    pub fn with_version(
        transaction_id: ForeignCurrencyTransactionId,
        event_type: ForeignCurrencyEventType,
        version: u64,
    ) -> Self {
        Self { transaction_id, event_type, occurred_at: Utc::now(), version }
    }
}

impl DomainEvent for ForeignCurrencyEvent {
    fn event_type(&self) -> &str {
        match &self.event_type {
            ForeignCurrencyEventType::TransactionRecorded { .. } => "TransactionRecorded",
            ForeignCurrencyEventType::Remeasured { .. } => "Remeasured",
            ForeignCurrencyEventType::ExchangeGainLossRecognized { .. } => {
                "ExchangeGainLossRecognized"
            }
        }
    }

    fn aggregate_id(&self) -> &str {
        Box::leak(self.transaction_id.to_string().into_boxed_str())
    }

    fn version(&self) -> u64 {
        self.version
    }
}

/// 外貨換算イベントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForeignCurrencyEventType {
    /// 取引記録
    TransactionRecorded {
        foreign_currency: Currency,
        foreign_amount: Amount,
        transaction_rate: f64,
        functional_amount: Amount,
        monetary_classification: MonetaryClassification,
    },
    /// 評価替え実施
    Remeasured {
        closing_rate: f64,
        functional_amount_at_closing: Amount,
        exchange_gain_loss: Amount,
    },
    /// 為替差損益認識
    ExchangeGainLossRecognized { amount: Amount, is_gain: bool },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_recorded_event() {
        let transaction_id = ForeignCurrencyTransactionId::new();

        let event = ForeignCurrencyEvent::new(
            transaction_id.clone(),
            ForeignCurrencyEventType::TransactionRecorded {
                foreign_currency: Currency::USD,
                foreign_amount: Amount::from_i64(1_000),
                transaction_rate: 150.0,
                functional_amount: Amount::from_i64(150_000),
                monetary_classification: MonetaryClassification::Monetary,
            },
        );

        assert_eq!(event.event_type(), "TransactionRecorded");
        assert_eq!(event.transaction_id, transaction_id);
    }

    #[test]
    fn test_remeasured_event() {
        let transaction_id = ForeignCurrencyTransactionId::new();

        let event = ForeignCurrencyEvent::new(
            transaction_id,
            ForeignCurrencyEventType::Remeasured {
                closing_rate: 155.0,
                functional_amount_at_closing: Amount::from_i64(155_000),
                exchange_gain_loss: Amount::from_i64(5_000),
            },
        );

        assert_eq!(event.event_type(), "Remeasured");
    }
}
