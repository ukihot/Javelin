// JournalEntryRepository実装
// Event Sourcingパターンに基づく仕訳伝票の永続化

use std::sync::Arc;

use javelin_domain::{
    error::DomainResult, financial_close::journal_entry::events::JournalEntryEvent,
    repositories::RepositoryBase,
};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    event_store::EventStore,
    types::{ExpectedVersion, Sequence},
};

/// JournalEntryRepository実装
///
/// Event Sourcingパターンに基づき、仕訳伝票のイベントを
/// EventStoreに永続化する。
pub struct JournalEntryRepositoryImpl {
    event_store: Arc<EventStore>,
}

impl JournalEntryRepositoryImpl {
    /// 新しいリポジトリインスタンスを作成
    pub fn new(event_store: Arc<EventStore>) -> Self {
        Self { event_store }
    }

    /// イベントストリームから仕訳伝票を復元
    ///
    /// 指定された集約IDのすべてのイベントを読み込み、
    /// 仕訳伝票の現在の状態を復元する。
    pub async fn load_events(
        &self,
        entry_id: &str,
    ) -> InfrastructureResult<Vec<JournalEntryEvent>> {
        let agg_id = crate::types::AggregateId::parse(entry_id)
            .map_err(InfrastructureError::DeserializationFailed)?;
        let stream = self.event_store.stream_aggregate_events(agg_id, Sequence::new(0));

        // モダンプラクティス: 初期キャパシティを確保
        let mut events = Vec::with_capacity(16);
        for event_result in stream.iter() {
            let stored_event = event_result?;
            let event: JournalEntryEvent = serde_json::from_slice(&stored_event.payload)
                .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;
            events.push(event);
        }

        Ok(events)
    }

    /// 最新バージョンを取得
    ///
    /// 指定された集約IDの最新バージョン番号を取得する。
    pub async fn get_latest_version(&self, entry_id: &str) -> InfrastructureResult<u64> {
        let events = self.load_events(entry_id).await?;
        Ok(events.len() as u64)
    }
}

impl RepositoryBase for JournalEntryRepositoryImpl {
    type Event = JournalEntryEvent;

    async fn append(&self, event: Self::Event) -> DomainResult<()> {
        let event_type = event.event_type();
        let aggregate_id = event.aggregate_id();
        let payload = serde_json::to_vec(&event)
            .map_err(|e| javelin_domain::error::DomainError::SerializationFailed(e.to_string()))?;

        // バージョン管理（簡易実装）
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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_append_and_load_events() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let repo = JournalEntryRepositoryImpl::new(event_store);

        // イベント追加
        let event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        repo.append(event.clone()).await.unwrap();

        // イベント読み込み
        let events = repo.load_events("JE001").await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], event);
    }

    #[tokio::test]
    async fn test_multiple_events() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let repo = JournalEntryRepositoryImpl::new(event_store);

        let entry_id = "JE002";

        // 複数イベント追加
        let event1 = JournalEntryEvent::DraftCreated {
            entry_id: entry_id.to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V002".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        let event2 = JournalEntryEvent::ApprovalRequested {
            entry_id: entry_id.to_string(),
            requested_by: "user1".to_string(),
            requested_at: Utc::now(),
        };

        let event3 = JournalEntryEvent::Posted {
            entry_id: entry_id.to_string(),
            entry_number: "EN-2024-001".to_string(),
            posted_by: "approver1".to_string(),
            posted_at: Utc::now(),
        };

        repo.append(event1).await.unwrap();
        repo.append(event2).await.unwrap();
        repo.append(event3).await.unwrap();

        // イベント読み込み
        let events = repo.load_events(entry_id).await.unwrap();
        assert_eq!(events.len(), 3);

        // バージョン確認
        let version = repo.get_latest_version(entry_id).await.unwrap();
        assert_eq!(version, 3);
    }
}
