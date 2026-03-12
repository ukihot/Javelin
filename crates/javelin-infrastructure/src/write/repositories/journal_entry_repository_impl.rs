// JournalEntryRepository実装
// Event Sourcingパターンに基づく仕訳伝票の永続化

use std::sync::Arc;

use javelin_domain::{
    common::RepositoryBase,
    error::DomainResult,
    journal_entry::{
        entities::JournalEntry, events::JournalEntryEvent, repositories::JournalEntryRepository,
    },
};

use crate::{types::ExpectedVersion, write::event_store::EventStore};

/// JournalEntryRepository実装
///
/// Event Sourcingパターンに基づき、仕訳伝票のイベントを
/// EventStoreに永続化する。
///
/// # Architecture
/// - Command側: Aggregateのロードと保存
/// - EventStoreを使ってイベント再生と保存
/// - Query側はQueryServiceで実装（このリポジトリには含まない）
pub struct JournalEntryRepositoryImpl {
    event_store: Arc<EventStore>,
}

impl JournalEntryRepositoryImpl {
    /// 新しいリポジトリインスタンスを作成
    pub fn new(event_store: Arc<EventStore>) -> Self {
        Self { event_store }
    }
}

// JournalEntryRepository trait implementation
impl JournalEntryRepository for JournalEntryRepositoryImpl {}

// RepositoryBase trait implementation
impl RepositoryBase<JournalEntry> for JournalEntryRepositoryImpl {
    async fn save(&self, aggregate: &JournalEntry) -> DomainResult<()> {
        // 集約から未コミットイベントを取得
        let uncommitted_events = aggregate.uncommitted_events();

        if uncommitted_events.is_empty() {
            return Ok(()); // 変更がない場合は何もしない
        }

        // イベントをEventStoreに保存
        for event in uncommitted_events {
            let event_type = event.event_type();
            let aggregate_id = event.aggregate_id();
            let payload = serde_json::to_vec(&event).map_err(|e| {
                javelin_domain::error::DomainError::SerializationFailed(e.to_string())
            })?;

            // バージョン管理: EventStoreが自動的にシーケンス番号を管理
            let version = self
                .event_store
                .get_events(aggregate_id)
                .await
                .map(|events| events.len() as u64 + 1)
                .unwrap_or(1);

            self.event_store
                .append_event(event_type, aggregate_id, version, ExpectedVersion::any(), &payload)
                .await
                .map_err(|e| {
                    javelin_domain::error::DomainError::RepositoryError(format!(
                        "Failed to append event: {}",
                        e
                    ))
                })?;
        }

        Ok(())
    }

    async fn load(&self, id: &str) -> DomainResult<Option<JournalEntry>> {
        // EventStoreからイベントを取得
        let stored_events = self.event_store.get_events(id).await.map_err(|e| {
            javelin_domain::error::DomainError::RepositoryError(format!(
                "Failed to get events: {}",
                e
            ))
        })?;

        if stored_events.is_empty() {
            return Ok(None);
        }

        // イベントをデシリアライズ
        let events: Vec<JournalEntryEvent> = stored_events
            .into_iter()
            .map(|stored| {
                serde_json::from_slice(&stored.payload).map_err(|e| {
                    javelin_domain::error::DomainError::RepositoryError(format!(
                        "Failed to deserialize event: {}",
                        e
                    ))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        // TODO: イベントから集約を復元する機能を実装
        // 現在はイベントソーシングの復元機能が未実装
        // 一時的にエラーを返す
        Err(javelin_domain::error::DomainError::RepositoryError(
            "Event sourcing reconstruction not yet implemented".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_save_and_load_aggregate() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let repo = JournalEntryRepositoryImpl::new(event_store);

        // 集約を作成
        let entry = JournalEntry::create(
            "JE001".to_string(),
            "2024-01-01".to_string(),
            "V001".to_string(),
            vec![],
            "user1".to_string(),
        )
        .unwrap();

        // 保存
        repo.save(&entry).await.unwrap();

        // ロード
        let loaded = repo.load("JE001").await.unwrap();
        assert!(loaded.is_some());
        let loaded_entry = loaded.unwrap();
        assert_eq!(loaded_entry.entry_id(), "JE001");
    }

    #[tokio::test]
    async fn test_load_nonexistent_aggregate() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let repo = JournalEntryRepositoryImpl::new(event_store);

        // 存在しない集約をロード
        let loaded = repo.load("NONEXISTENT").await.unwrap();
        assert!(loaded.is_none());
    }

    #[tokio::test]
    async fn test_multiple_saves() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let repo = JournalEntryRepositoryImpl::new(event_store);

        // 集約を作成して保存
        let mut entry = JournalEntry::create(
            "JE002".to_string(),
            "2024-01-01".to_string(),
            "V002".to_string(),
            vec![],
            "user1".to_string(),
        )
        .unwrap();

        repo.save(&entry).await.unwrap();

        // 承認申請
        entry.request_approval("user1".to_string()).unwrap();
        repo.save(&entry).await.unwrap();

        // 記帳
        entry.post("EN-2024-001".to_string(), "approver1".to_string()).unwrap();
        repo.save(&entry).await.unwrap();

        // ロードして状態を確認
        let loaded = repo.load("JE002").await.unwrap().unwrap();
        assert_eq!(loaded.status(), javelin_domain::journal_entry::values::JournalStatus::Posted);
    }
}
