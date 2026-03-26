// 取引先集約のエンティティ

use serde::{Deserialize, Serialize};

use super::values::{ContactInfo, PartnerType};
use crate::{
    entity::{Entity, EntityId},
    error::{DomainError, DomainResult},
};

/// 取引先ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PartnerId(String);

impl PartnerId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl EntityId for PartnerId {
    fn value(&self) -> &str {
        &self.0
    }
}

/// 取引先エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partner {
    id: PartnerId,
    name: String,
    partner_type: PartnerType,
    contact_info: Option<ContactInfo>,
    tax_id: Option<String>, // 税務ID
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Partner {
    pub fn new(
        name: String,
        partner_type: PartnerType,
        contact_info: Option<ContactInfo>,
        tax_id: Option<String>,
    ) -> DomainResult<Self> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError("取引先名は空にできません".to_string()));
        }

        let now = chrono::Utc::now();
        Ok(Self {
            id: PartnerId::new(uuid::Uuid::new_v4().to_string()),
            name,
            partner_type,
            contact_info,
            tax_id,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    // Getters
    pub fn id(&self) -> &PartnerId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn partner_type(&self) -> &PartnerType {
        &self.partner_type
    }

    pub fn contact_info(&self) -> Option<&ContactInfo> {
        self.contact_info.as_ref()
    }

    pub fn tax_id(&self) -> Option<&str> {
        self.tax_id.as_deref()
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// 取引先を無効化
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = chrono::Utc::now();
    }
}

impl Entity for Partner {
    type Id = PartnerId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
