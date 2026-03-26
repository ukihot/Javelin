// JournalEntryRepository実装
// Event Sourcingパターンに基づく仕訳伝票の永続化

use std::sync::Arc;

use javelin_domain::{
    common::RepositoryBase,
    error::DomainResult,
    journal_entry::{
        domain_events::JournalEntryEvent, entities::JournalEntry,
        repositories::JournalEntryRepository,
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
        let _events: Vec<JournalEntryEvent> = stored_events
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
