// JournalEntryEventHandler実装
// ドメインイベントを受け取り、副作用を実行する

use std::sync::Arc;

use javelin_domain::financial_close::journal_entry::events::JournalEntryEvent;

use crate::{
    error::InfrastructureResult,
    queries::journal_entry_projection_worker::JournalEntryProjectionWorker,
};

/// JournalEntryEventHandler
///
/// JournalEntryEventを受け取り、以下の処理を実行する：
/// - Projectionの更新
/// - 通知の送信（将来実装）
/// - 監査ログの記録（将来実装）
pub struct JournalEntryEventHandler {
    projection_worker: Arc<JournalEntryProjectionWorker>,
}

impl JournalEntryEventHandler {
    /// 新しいハンドラインスタンスを作成
    pub fn new(projection_worker: Arc<JournalEntryProjectionWorker>) -> Self {
        Self { projection_worker }
    }

    /// イベントを処理
    pub async fn handle(&self, event: &JournalEntryEvent) -> InfrastructureResult<()> {
        // Projectionを更新
        self.update_projection(event).await?;

        // イベントタイプに応じた追加処理
        match event {
            JournalEntryEvent::DraftCreated { .. } => {
                // 下書き作成時の処理
                self.on_draft_created(event).await?;
            }
            JournalEntryEvent::ApprovalRequested { .. } => {
                // 承認申請時の処理（通知など）
                self.on_approval_requested(event).await?;
            }
            JournalEntryEvent::Posted { .. } => {
                // 記帳時の処理（元帳更新など）
                self.on_posted(event).await?;
            }
            JournalEntryEvent::Reversed { .. } => {
                // 取消時の処理
                self.on_reversed(event).await?;
            }
            JournalEntryEvent::Corrected { .. } => {
                // 修正時の処理
                self.on_corrected(event).await?;
            }
            _ => {
                // その他のイベント
            }
        }

        Ok(())
    }

    /// Projectionを更新
    async fn update_projection(&self, event: &JournalEntryEvent) -> InfrastructureResult<()> {
        let entry_id = event.aggregate_id();
        self.projection_worker.update_aggregate(entry_id).await?;
        Ok(())
    }

    /// 下書き作成時の処理
    async fn on_draft_created(&self, _event: &JournalEntryEvent) -> InfrastructureResult<()> {
        // 将来実装：
        // - 監査ログの記録
        // - メトリクスの更新
        Ok(())
    }

    /// 承認申請時の処理
    async fn on_approval_requested(&self, _event: &JournalEntryEvent) -> InfrastructureResult<()> {
        // 将来実装：
        // - 承認者への通知送信
        // - ワークフローの開始
        Ok(())
    }

    /// 記帳時の処理
    async fn on_posted(&self, _event: &JournalEntryEvent) -> InfrastructureResult<()> {
        // 将来実装：
        // - 元帳Projectionの更新
        // - 財務レポートの更新
        // - 通知送信
        Ok(())
    }

    /// 取消時の処理
    async fn on_reversed(&self, _event: &JournalEntryEvent) -> InfrastructureResult<()> {
        // 将来実装：
        // - 元帳Projectionの更新（逆仕訳）
        // - 監査ログの記録
        Ok(())
    }

    /// 修正時の処理
    async fn on_corrected(&self, _event: &JournalEntryEvent) -> InfrastructureResult<()> {
        // 将来実装：
        // - 元帳Projectionの更新
        // - 監査ログの記録
        Ok(())
    }

    /// バッチでイベントを処理
    pub async fn handle_batch(&self, events: &[JournalEntryEvent]) -> InfrastructureResult<()> {
        for event in events {
            self.handle(event).await?;
        }
        Ok(())
    }
}

/// イベントハンドラビルダー
pub struct JournalEntryEventHandlerBuilder {
    projection_worker: Option<Arc<JournalEntryProjectionWorker>>,
}

impl JournalEntryEventHandlerBuilder {
    /// 新しいビルダーを作成
    pub fn new() -> Self {
        Self { projection_worker: None }
    }

    /// ProjectionWorkerを設定
    pub fn with_projection_worker(mut self, worker: Arc<JournalEntryProjectionWorker>) -> Self {
        self.projection_worker = Some(worker);
        self
    }

    /// ハンドラを構築
    pub fn build(self) -> Result<JournalEntryEventHandler, String> {
        let projection_worker = self.projection_worker.ok_or("ProjectionWorker is required")?;

        Ok(JournalEntryEventHandler::new(projection_worker))
    }
}

impl Default for JournalEntryEventHandlerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use tempfile::TempDir;

    use super::*;
    use crate::{event_store::EventStore, projection_db::ProjectionDb};

    #[tokio::test]
    async fn test_event_handler_creation() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());

        let worker = Arc::new(JournalEntryProjectionWorker::new(
            event_store,
            projection_db,
            "journal_entries".to_string(),
            1,
        ));

        let handler = JournalEntryEventHandlerBuilder::new()
            .with_projection_worker(worker)
            .build()
            .unwrap();

        assert!(std::ptr::addr_of!(handler).is_aligned());
    }

    #[tokio::test]
    async fn test_handle_draft_created() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());

        let worker = Arc::new(JournalEntryProjectionWorker::new(
            event_store.clone(),
            projection_db,
            "journal_entries".to_string(),
            1,
        ));

        let handler = JournalEntryEventHandler::new(worker);

        let event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        // イベントをEventStoreに保存
        let payload = serde_json::to_vec(&event).unwrap();
        event_store
            .append_event(
                event.event_type(),
                event.aggregate_id(),
                1,
                crate::types::ExpectedVersion::any(),
                &payload,
            )
            .await
            .unwrap();

        // ハンドラで処理
        handler.handle(&event).await.unwrap();
    }

    #[tokio::test]
    async fn test_handle_batch() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());

        let worker = Arc::new(JournalEntryProjectionWorker::new(
            event_store.clone(),
            projection_db,
            "journal_entries".to_string(),
            1,
        ));

        let handler = JournalEntryEventHandler::new(worker);

        let events = vec![
            JournalEntryEvent::DraftCreated {
                entry_id: "JE001".to_string(),
                transaction_date: "2024-01-01".to_string(),
                voucher_number: "V001".to_string(),
                lines: vec![],
                created_by: "user1".to_string(),
                created_at: Utc::now(),
            },
            JournalEntryEvent::DraftCreated {
                entry_id: "JE002".to_string(),
                transaction_date: "2024-01-02".to_string(),
                voucher_number: "V002".to_string(),
                lines: vec![],
                created_by: "user1".to_string(),
                created_at: Utc::now(),
            },
        ];

        // イベントをEventStoreに保存
        for (i, event) in events.iter().enumerate() {
            let payload = serde_json::to_vec(event).unwrap();
            event_store
                .append_event(
                    event.event_type(),
                    event.aggregate_id(),
                    (i + 1) as u64,
                    crate::types::ExpectedVersion::any(),
                    &payload,
                )
                .await
                .unwrap();
        }

        // バッチ処理
        handler.handle_batch(&events).await.unwrap();
    }
}
