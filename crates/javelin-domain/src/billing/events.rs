// Billing Events - 請求集約のドメインイベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{InvoiceId, SettlementStatus};
use crate::event::DomainEvent;

/// 請求書イベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceEvent {
    invoice_id: InvoiceId,
    event_type: InvoiceEventType,
    occurred_at: DateTime<Utc>,
}

impl InvoiceEvent {
    pub fn new(invoice_id: InvoiceId, event_type: InvoiceEventType) -> Self {
        Self { invoice_id, event_type, occurred_at: Utc::now() }
    }

    pub fn invoice_id(&self) -> &InvoiceId {
        &self.invoice_id
    }

    pub fn event_type(&self) -> &InvoiceEventType {
        &self.event_type
    }
}

impl DomainEvent for InvoiceEvent {
    fn event_type(&self) -> &str {
        match &self.event_type {
            InvoiceEventType::InvoiceCreated { .. } => "InvoiceCreated",
            InvoiceEventType::InvoiceRevised { .. } => "InvoiceRevised",
            InvoiceEventType::PaymentReceived { .. } => "PaymentReceived",
            InvoiceEventType::StatusChanged { .. } => "StatusChanged",
        }
    }

    fn aggregate_id(&self) -> &str {
        self.invoice_id.value()
    }

    fn version(&self) -> u64 {
        1 // デフォルトバージョン
    }
}

/// 請求書イベント種別
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceEventType {
    /// 請求書が作成された
    InvoiceCreated { invoice_number: String, recipient_name: String, total_amount: String },
    /// 請求書が訂正された
    InvoiceRevised { original_invoice_id: InvoiceId, new_invoice_number: String },
    /// 支払いを受領した
    PaymentReceived { payment_date: DateTime<Utc>, amount: String },
    /// ステータスが変更された
    StatusChanged { old_status: SettlementStatus, new_status: SettlementStatus },
}
