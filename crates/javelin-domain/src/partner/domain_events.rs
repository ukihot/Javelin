// 取引先集約のドメインイベント

use serde::{Deserialize, Serialize};

use crate::event::DomainEvent;

/// 取引先が作成されたイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCreated {
    pub partner_id: String,
    pub name: String,
    pub partner_type: String,
}

impl DomainEvent for PartnerCreated {
    fn event_type(&self) -> &str {
        "partner_created"
    }

    fn aggregate_id(&self) -> &str {
        &self.partner_id
    }

    fn version(&self) -> u64 {
        1
    }
}
