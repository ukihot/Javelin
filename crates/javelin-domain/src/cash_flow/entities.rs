// 入出金集約のエンティティ

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    common::Amount,
    entity::{Entity, EntityId},
    error::{DomainError, DomainResult},
};

/// 入出金ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CashFlowId(String);

impl CashFlowId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl EntityId for CashFlowId {
    fn value(&self) -> &str {
        &self.0
    }
}

/// 入出金エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlow {
    id: CashFlowId,
    date: NaiveDate,
    amount: Amount,
    account_id: String, // 現預金勘定科目ID
    description: String,
    reference: Option<String>, // 関連する証憑や仕訳の参照
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl CashFlow {
    pub fn new(
        date: NaiveDate,
        amount: Amount,
        account_id: String,
        description: String,
        reference: Option<String>,
    ) -> DomainResult<Self> {
        if description.trim().is_empty() {
            return Err(DomainError::ValidationError("摘要は空にできません".to_string()));
        }

        let now = chrono::Utc::now();
        Ok(Self {
            id: CashFlowId::new(uuid::Uuid::new_v4().to_string()),
            date,
            amount,
            account_id,
            description,
            reference,
            created_at: now,
            updated_at: now,
        })
    }

    // Getters
    pub fn id(&self) -> &CashFlowId {
        &self.id
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }
}

impl Entity for CashFlow {
    type Id = CashFlowId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
