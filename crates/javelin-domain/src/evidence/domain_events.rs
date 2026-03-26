// 証憑集約のドメインイベント

use serde::{Deserialize, Serialize};

use crate::event::DomainEvent;

/// 証憑が添付されたイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceAttached {
    pub evidence_id: String,
    pub evidence_type: String,
    pub description: String,
}

impl DomainEvent for EvidenceAttached {
    fn event_type(&self) -> &str {
        "evidence_attached"
    }

    fn aggregate_id(&self) -> &str {
        &self.evidence_id
    }

    fn version(&self) -> u64 {
        1
    }
}
