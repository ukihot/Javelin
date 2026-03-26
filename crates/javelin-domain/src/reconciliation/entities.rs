// 消込集約のエンティティ

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    common::Amount,
    entity::{Entity, EntityId},
    error::DomainResult,
};

/// 消込ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReconciliationId(String);

impl ReconciliationId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl EntityId for ReconciliationId {
    fn value(&self) -> &str {
        &self.0
    }
}

/// 消込エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reconciliation {
    id: ReconciliationId,
    receivable_payable_id: String,
    cash_flow_id: String,
    reconciled_amount: Amount,
    reconciliation_date: NaiveDate,
    notes: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Reconciliation {
    pub fn new(
        receivable_payable_id: String,
        cash_flow_id: String,
        reconciled_amount: Amount,
        reconciliation_date: NaiveDate,
        notes: Option<String>,
    ) -> DomainResult<Self> {
        let now = chrono::Utc::now();
        Ok(Self {
            id: ReconciliationId::new(uuid::Uuid::new_v4().to_string()),
            receivable_payable_id,
            cash_flow_id,
            reconciled_amount,
            reconciliation_date,
            notes,
            created_at: now,
            updated_at: now,
        })
    }

    // Getters
    pub fn id(&self) -> &ReconciliationId {
        &self.id
    }

    pub fn receivable_payable_id(&self) -> &str {
        &self.receivable_payable_id
    }

    pub fn cash_flow_id(&self) -> &str {
        &self.cash_flow_id
    }

    pub fn reconciled_amount(&self) -> &Amount {
        &self.reconciled_amount
    }

    pub fn reconciliation_date(&self) -> NaiveDate {
        self.reconciliation_date
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}

impl Entity for Reconciliation {
    type Id = ReconciliationId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
