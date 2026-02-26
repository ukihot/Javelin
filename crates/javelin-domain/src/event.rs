// Event - ドメインイベント
// 正本データ: Event Store
// 更新方式: 追記専用
// 状態復元: イベント再生

use serde::{Deserialize, Serialize};

pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &str;
    fn aggregate_id(&self) -> &str;
    fn version(&self) -> u64;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    event_type: String,
    aggregate_id: String,
    version: u64,
    payload: Vec<u8>,
}

impl Event {
    pub fn new(event_type: String, aggregate_id: String, version: u64, payload: Vec<u8>) -> Self {
        Self { event_type, aggregate_id, version, payload }
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

impl DomainEvent for Event {
    fn event_type(&self) -> &str {
        &self.event_type
    }

    fn aggregate_id(&self) -> &str {
        &self.aggregate_id
    }

    fn version(&self) -> u64 {
        self.version
    }
}
