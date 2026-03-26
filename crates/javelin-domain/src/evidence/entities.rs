// 証憑集約のエンティティ

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::values::EvidenceType;
use crate::{
    entity::{Entity, EntityId},
    error::{DomainError, DomainResult},
};

/// 証憑ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EvidenceId(String);

impl EvidenceId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl EntityId for EvidenceId {
    fn value(&self) -> &str {
        &self.0
    }
}

/// 証憑エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    id: EvidenceId,
    evidence_type: EvidenceType,
    date: NaiveDate,
    description: String,
    amount: Option<crate::common::Amount>,
    file_path: Option<String>, // 添付ファイルのパス
    related_journal_entry_id: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Evidence {
    pub fn new(
        evidence_type: EvidenceType,
        date: NaiveDate,
        description: String,
        amount: Option<crate::common::Amount>,
        file_path: Option<String>,
    ) -> DomainResult<Self> {
        if description.trim().is_empty() {
            return Err(DomainError::ValidationError("説明は空にできません".to_string()));
        }

        let now = chrono::Utc::now();
        Ok(Self {
            id: EvidenceId::new(uuid::Uuid::new_v4().to_string()),
            evidence_type,
            date,
            description,
            amount,
            file_path,
            related_journal_entry_id: None,
            created_at: now,
            updated_at: now,
        })
    }

    // Getters
    pub fn id(&self) -> &EvidenceId {
        &self.id
    }

    pub fn evidence_type(&self) -> &EvidenceType {
        &self.evidence_type
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn amount(&self) -> Option<&crate::common::Amount> {
        self.amount.as_ref()
    }

    pub fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }

    /// 関連する仕訳を設定
    pub fn set_related_journal_entry(&mut self, journal_entry_id: String) {
        self.related_journal_entry_id = Some(journal_entry_id);
        self.updated_at = chrono::Utc::now();
    }
}

impl Entity for Evidence {
    type Id = EvidenceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
