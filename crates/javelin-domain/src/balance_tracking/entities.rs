// BalanceTracking集約のエンティティ

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::values::{BalanceTrackingStatus, TransactionType};
use crate::{
    common::Amount,
    entity::{Entity, EntityId},
    error::{DomainError, DomainResult},
};

/// BalanceTrackingID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BalanceTrackingId(String);

impl BalanceTrackingId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl EntityId for BalanceTrackingId {
    fn value(&self) -> &str {
        &self.0
    }
}

/// BalanceTrackingエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceTracking {
    id: BalanceTrackingId,
    partner_id: String,
    amount: Amount,
    due_date: NaiveDate,
    description: String,
    transaction_type: TransactionType,
    status: BalanceTrackingStatus,
    related_journal_entry_id: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl BalanceTracking {
    pub fn new(
        partner_id: String,
        amount: Amount,
        due_date: NaiveDate,
        description: String,
        transaction_type: TransactionType,
    ) -> DomainResult<Self> {
        if description.trim().is_empty() {
            return Err(DomainError::ValidationError("説明は空にできません".to_string()));
        }

        let now = chrono::Utc::now();
        Ok(Self {
            id: BalanceTrackingId::new(uuid::Uuid::new_v4().to_string()),
            partner_id,
            amount,
            due_date,
            description,
            transaction_type,
            status: BalanceTrackingStatus::Pending,
            related_journal_entry_id: None,
            created_at: now,
            updated_at: now,
        })
    }

    // Getters
    pub fn id(&self) -> &BalanceTrackingId {
        &self.id
    }

    pub fn partner_id(&self) -> &str {
        &self.partner_id
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn due_date(&self) -> NaiveDate {
        self.due_date
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn transaction_type(&self) -> &TransactionType {
        &self.transaction_type
    }

    pub fn status(&self) -> &BalanceTrackingStatus {
        &self.status
    }

    /// 決済済みに変更
    pub fn mark_as_settled(&mut self) {
        self.status = BalanceTrackingStatus::Settled;
        self.updated_at = chrono::Utc::now();
    }
}

impl Entity for BalanceTracking {
    type Id = BalanceTrackingId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
