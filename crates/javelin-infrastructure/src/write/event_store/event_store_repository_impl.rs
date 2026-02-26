// EventStore の RepositoryBase トレイト実装
// Infrastructure層がDomain層のトレイトを実装

use javelin_domain::{
    error::DomainResult,
    repositories::{ClosingRepository, JournalEntryRepository, RepositoryBase},
};

use super::store::EventStore;

/// EventStoreのRepositoryBaseトレイト実装
///
/// Domain層が定義するRepositoryBaseトレイトを、
/// Infrastructure層のEventStoreが実装する。
/// これによりクリーンアーキテクチャの依存関係原則を実現。
impl RepositoryBase for EventStore {
    type Event = javelin_domain::financial_close::journal_entry::events::JournalEntryEvent;

    async fn append(&self, event: Self::Event) -> DomainResult<()> {
        // 単一イベントの追記は、ベクタに変換して append_events を呼び出す
        let aggregate_id = event.aggregate_id().to_string();
        self.append_events(&aggregate_id, vec![event]).await.map(|_| ())
    }

    async fn append_events<T>(&self, aggregate_id: &str, events: Vec<T>) -> DomainResult<u64>
    where
        T: serde::Serialize + Send + 'static,
    {
        self.append(aggregate_id, events)
            .await
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))
    }

    async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>> {
        let events = self
            .get_events(aggregate_id)
            .await
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        // StoredEventをserde_json::Valueに変換
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
            .get_all_events(from_sequence)
            .await
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        // StoredEventをserde_json::Valueに変換
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
        self.get_latest_sequence()
            .await
            .map(|seq| seq.as_u64())
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))
    }
}

/// EventStoreのJournalEntryRepositoryトレイト実装
impl JournalEntryRepository for EventStore {}

/// ClosingEvent用のEventStore型エイリアス
///
/// ClosingEventを扱うためのEventStore実装
pub struct ClosingEventStore(pub std::sync::Arc<EventStore>);

impl RepositoryBase for ClosingEventStore {
    type Event = javelin_domain::financial_close::closing_events::ClosingEvent;

    async fn append(&self, event: Self::Event) -> DomainResult<()> {
        let aggregate_id = format!("closing-{}", event.aggregate_id());
        self.0.append_events(&aggregate_id, vec![event]).await.map(|_| ())
    }

    async fn append_events<T>(&self, aggregate_id: &str, events: Vec<T>) -> DomainResult<u64>
    where
        T: serde::Serialize + Send + 'static,
    {
        self.0
            .append(aggregate_id, events)
            .await
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))
    }

    async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>> {
        let events = self
            .0
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
            .0
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
        self.0
            .get_latest_sequence()
            .await
            .map(|seq| seq.as_u64())
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))
    }
}

/// ClosingEventStoreのClosingRepositoryトレイト実装
impl ClosingRepository for ClosingEventStore {}
