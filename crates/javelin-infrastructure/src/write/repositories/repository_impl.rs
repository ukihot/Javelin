// 汎用イベントリポジトリ実装 - RepositoryBase具象化

use std::sync::Arc;

use javelin_domain::{error::DomainResult, event::DomainEvent, repositories::RepositoryBase};

use crate::{event_store::EventStore, types::ExpectedVersion};

pub struct EventRepositoryImpl<E: DomainEvent> {
    store: Arc<EventStore>,
    _phantom: std::marker::PhantomData<E>,
}

impl<E: DomainEvent> EventRepositoryImpl<E> {
    pub fn new(store: Arc<EventStore>) -> Self {
        Self { store, _phantom: std::marker::PhantomData }
    }
}

impl<E: DomainEvent + serde::Serialize> RepositoryBase for EventRepositoryImpl<E> {
    type Event = E;

    async fn append(&self, event: Self::Event) -> DomainResult<()> {
        let payload = serde_json::to_vec(&event)
            .map_err(|e| javelin_domain::error::DomainError::SerializationFailed(e.to_string()))?;

        self.store
            .append_event(
                event.event_type(),
                event.aggregate_id(),
                event.version(),
                ExpectedVersion::any(),
                &payload,
            )
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
        self.store
            .append(aggregate_id, events)
            .await
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))
    }

    async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>> {
        self.store
            .get_events(aggregate_id)
            .await
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?
            .into_iter()
            .map(|stored| {
                serde_json::to_value(&stored.payload)
                    .or_else(|_| serde_json::from_slice(&stored.payload))
                    .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))
            })
            .collect()
    }

    async fn get_all_events(&self, from_sequence: u64) -> DomainResult<Vec<serde_json::Value>> {
        self.store
            .get_all_events(from_sequence)
            .await
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?
            .into_iter()
            .map(|stored| {
                serde_json::to_value(&stored.payload)
                    .or_else(|_| serde_json::from_slice(&stored.payload))
                    .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))
            })
            .collect()
    }

    async fn get_latest_sequence(&self) -> DomainResult<u64> {
        self.store
            .get_latest_sequence()
            .await
            .map(|s| s.as_u64())
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use tempfile::TempDir;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestEvent {
        event_type: String,
        aggregate_id: String,
        version: u64,
        data: String,
    }

    impl DomainEvent for TestEvent {
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

    #[tokio::test]
    async fn test_event_repository_append() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let event_store_path = temp_dir.path().join("events");

        let store = Arc::new(EventStore::new(&event_store_path).await.unwrap());
        let repo = EventRepositoryImpl::<TestEvent>::new(store);

        let event = TestEvent {
            event_type: "TestEvent".to_string(),
            aggregate_id: "test-001".to_string(),
            version: 1,
            data: "test data".to_string(),
        };

        let result = repo.append(event).await;
        assert!(result.is_ok(), "Event append should succeed");
    }
}
