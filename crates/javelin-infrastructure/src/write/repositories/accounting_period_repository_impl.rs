// AccountingPeriodRepository実装
// Event Sourcingパターンに基づく会計期間の永続化

use std::sync::Arc;

use javelin_domain::{error::DomainResult, repositories::RepositoryBase};
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    event_store::EventStore,
    types::{ExpectedVersion, Sequence},
};

/// 会計期間ドメインイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum AccountingPeriodEvent {
    /// 期間作成
    PeriodCreated {
        period_id: String,
        fiscal_year: u32,
        period: u32,
        start_date: String,
        end_date: String,
        created_by: String,
        created_at: String,
    },

    /// 期間締め
    PeriodClosed { period_id: String, closed_by: String, closed_at: String },

    /// 期間ロック
    PeriodLocked { period_id: String, locked_by: String, locked_at: String },

    /// 期間再オープン
    PeriodReopened { period_id: String, reason: String, reopened_by: String, reopened_at: String },
}

impl AccountingPeriodEvent {
    /// イベントタイプを取得
    pub fn event_type(&self) -> &str {
        match self {
            AccountingPeriodEvent::PeriodCreated { .. } => "PeriodCreated",
            AccountingPeriodEvent::PeriodClosed { .. } => "PeriodClosed",
            AccountingPeriodEvent::PeriodLocked { .. } => "PeriodLocked",
            AccountingPeriodEvent::PeriodReopened { .. } => "PeriodReopened",
        }
    }

    /// 集約IDを取得
    pub fn aggregate_id(&self) -> &str {
        match self {
            AccountingPeriodEvent::PeriodCreated { period_id, .. }
            | AccountingPeriodEvent::PeriodClosed { period_id, .. }
            | AccountingPeriodEvent::PeriodLocked { period_id, .. }
            | AccountingPeriodEvent::PeriodReopened { period_id, .. } => period_id,
        }
    }
}

/// AccountingPeriodRepository実装
///
/// Event Sourcingパターンに基づき、会計期間のイベントを
/// EventStoreに永続化する。
pub struct AccountingPeriodRepositoryImpl {
    event_store: Arc<EventStore>,
}

impl AccountingPeriodRepositoryImpl {
    /// 新しいリポジトリインスタンスを作成
    pub fn new(event_store: Arc<EventStore>) -> Self {
        Self { event_store }
    }

    /// イベントストリームから会計期間を復元
    pub async fn load_events(
        &self,
        period_id: &str,
    ) -> InfrastructureResult<Vec<AccountingPeriodEvent>> {
        let agg_id = crate::types::AggregateId::parse(period_id)
            .map_err(InfrastructureError::DeserializationFailed)?;
        let stream = self.event_store.stream_aggregate_events(agg_id, Sequence::new(0));

        let mut events = Vec::new();
        for event_result in stream.iter() {
            let stored_event = event_result?;
            let event: AccountingPeriodEvent = serde_json::from_slice(&stored_event.payload)
                .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;
            events.push(event);
        }

        Ok(events)
    }

    /// 最新バージョンを取得
    pub async fn get_latest_version(&self, period_id: &str) -> InfrastructureResult<u64> {
        let events = self.load_events(period_id).await?;
        Ok(events.len() as u64)
    }
}

impl RepositoryBase for AccountingPeriodRepositoryImpl {
    type Event = AccountingPeriodEvent;

    async fn append(&self, event: Self::Event) -> DomainResult<()> {
        let event_type = event.event_type();
        let aggregate_id = event.aggregate_id();
        let payload = serde_json::to_vec(&event)
            .map_err(|e| javelin_domain::error::DomainError::SerializationFailed(e.to_string()))?;

        let version = self.get_latest_version(aggregate_id).await.map_err(|e| {
            javelin_domain::error::DomainError::RepositoryError(format!(
                "Failed to get version: {}",
                e
            ))
        })? + 1;

        self.event_store
            .append_event(event_type, aggregate_id, version, ExpectedVersion::any(), &payload)
            .await
            .map_err(|e| {
                javelin_domain::error::DomainError::RepositoryError(format!(
                    "Failed to append event: {}",
                    e
                ))
            })?;

        Ok(())
    }

    async fn append_events<T>(&self, aggregate_id: &str, events: Vec<T>) -> DomainResult<u64>
    where
        T: serde::Serialize + Send + 'static,
    {
        self.event_store
            .append(aggregate_id, events)
            .await
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))
    }

    async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>> {
        let events = self
            .event_store
            .get_events(aggregate_id)
            .await
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        events
            .into_iter()
            .map(|stored_event| {
                serde_json::to_value(&stored_event.payload)
                    .or_else(|_| serde_json::from_slice(&stored_event.payload))
                    .map_err(|e| {
                        javelin_domain::error::DomainError::RepositoryError(format!(
                            "Failed to convert event: {}",
                            e
                        ))
                    })
            })
            .collect()
    }

    async fn get_all_events(&self, from_sequence: u64) -> DomainResult<Vec<serde_json::Value>> {
        let events = self
            .event_store
            .get_all_events(from_sequence)
            .await
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        events
            .into_iter()
            .map(|stored_event| {
                serde_json::to_value(&stored_event.payload)
                    .or_else(|_| serde_json::from_slice(&stored_event.payload))
                    .map_err(|e| {
                        javelin_domain::error::DomainError::RepositoryError(format!(
                            "Failed to convert event: {}",
                            e
                        ))
                    })
            })
            .collect()
    }

    async fn get_latest_sequence(&self) -> DomainResult<u64> {
        self.event_store
            .get_latest_sequence()
            .await
            .map(|seq| seq.as_u64())
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))
    }
}

// DomainEventトレイト実装
impl javelin_domain::event::DomainEvent for AccountingPeriodEvent {
    fn event_type(&self) -> &str {
        self.event_type()
    }

    fn aggregate_id(&self) -> &str {
        self.aggregate_id()
    }

    fn version(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_append_and_load_period_events() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let repo = AccountingPeriodRepositoryImpl::new(event_store);

        let event = AccountingPeriodEvent::PeriodCreated {
            period_id: "P202401".to_string(),
            fiscal_year: 2024,
            period: 1,
            start_date: "2024-01-01".to_string(),
            end_date: "2024-01-31".to_string(),
            created_by: "admin".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };

        repo.append(event.clone()).await.unwrap();

        let events = repo.load_events("P202401").await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], event);
    }

    #[tokio::test]
    async fn test_period_lifecycle() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let repo = AccountingPeriodRepositoryImpl::new(event_store);

        let period_id = "P202402";

        let event1 = AccountingPeriodEvent::PeriodCreated {
            period_id: period_id.to_string(),
            fiscal_year: 2024,
            period: 2,
            start_date: "2024-02-01".to_string(),
            end_date: "2024-02-29".to_string(),
            created_by: "admin".to_string(),
            created_at: "2024-02-01T00:00:00Z".to_string(),
        };

        let event2 = AccountingPeriodEvent::PeriodClosed {
            period_id: period_id.to_string(),
            closed_by: "admin".to_string(),
            closed_at: "2024-03-01T00:00:00Z".to_string(),
        };

        let event3 = AccountingPeriodEvent::PeriodLocked {
            period_id: period_id.to_string(),
            locked_by: "admin".to_string(),
            locked_at: "2024-03-05T00:00:00Z".to_string(),
        };

        repo.append(event1).await.unwrap();
        repo.append(event2).await.unwrap();
        repo.append(event3).await.unwrap();

        let events = repo.load_events(period_id).await.unwrap();
        assert_eq!(events.len(), 3);

        let version = repo.get_latest_version(period_id).await.unwrap();
        assert_eq!(version, 3);
    }
}
