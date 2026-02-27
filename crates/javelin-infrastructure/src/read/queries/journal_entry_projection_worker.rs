// JournalEntryProjectionWorker実装
// 仕訳イベントをProjectionに反映するワーカー

use std::sync::Arc;

use javelin_domain::financial_close::journal_entry::events::JournalEntryEvent;
use tokio::time::{Duration, interval};

use crate::{
    error::InfrastructureResult,
    event_store::EventStore,
    projection_db::ProjectionDb,
    projection_trait::ProjectionStrategy,
    queries::journal_entry_projection::{JournalEntryProjection, JournalEntryProjectionStrategy},
    types::Sequence,
};

/// JournalEntryProjectionWorker
///
/// JournalEntryEventを監視し、Projectionを更新する。
pub struct JournalEntryProjectionWorker {
    event_store: Arc<EventStore>,
    projection_db: Arc<ProjectionDb>,
    projection_name: String,
    projection_version: u32,
    poll_interval: Duration,
}

impl JournalEntryProjectionWorker {
    /// 新しいWorkerインスタンスを作成
    pub fn new(
        event_store: Arc<EventStore>,
        projection_db: Arc<ProjectionDb>,
        projection_name: String,
        projection_version: u32,
    ) -> Self {
        Self {
            event_store,
            projection_db,
            projection_name,
            projection_version,
            poll_interval: Duration::from_secs(1),
        }
    }

    /// ポーリング間隔を設定
    pub fn with_poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Projectionを最初から再構築
    pub async fn rebuild(&self) -> InfrastructureResult<()> {
        self.process_from(Sequence::new(0)).await
    }

    /// 指定位置からProjectionを処理
    pub async fn process_from(&self, from_sequence: Sequence) -> InfrastructureResult<()> {
        let stream = self.event_store.stream_events(from_sequence);
        let strategy = JournalEntryProjectionStrategy;

        let mut projections: std::collections::HashMap<String, JournalEntryProjection> =
            std::collections::HashMap::new();
        let mut last_sequence = from_sequence;

        for event_result in stream.iter() {
            let event = event_result?;

            // 戦略に基づいてフィルタリング
            if !strategy.should_update(&event) {
                continue;
            }

            // イベントをデシリアライズ
            let journal_event: JournalEntryEvent =
                serde_json::from_slice(&event.payload).map_err(|e| {
                    crate::error::InfrastructureError::DeserializationFailed(e.to_string())
                })?;

            let entry_id = journal_event.aggregate_id().to_string();

            // Projectionを取得または作成
            let projection = projections
                .entry(entry_id.clone())
                .or_insert_with(|| JournalEntryProjection::new(entry_id.clone()));

            // イベントを適用
            use crate::projection_trait::Apply;
            projection.apply(journal_event)?;

            last_sequence = Sequence::new(event.global_sequence);
        }

        // Projectionを保存
        // モダンプラクティス: projections数で初期キャパシティを確保
        let mut updates = Vec::with_capacity(projections.len());
        for (entry_id, projection) in projections {
            use crate::projection_trait::ToReadModel;
            let read_model = projection.to_read_model();
            let value = serde_json::to_vec(&read_model).map_err(|e| {
                crate::error::InfrastructureError::SerializationFailed(e.to_string())
            })?;
            updates.push((entry_id, value));
        }

        if !updates.is_empty() {
            self.projection_db
                .update_projection_batch(
                    &self.projection_name,
                    self.projection_version,
                    updates,
                    last_sequence.as_u64(),
                )
                .await?;
        }

        Ok(())
    }

    /// 継続的にProjectionを更新
    pub async fn run_continuous(&self) -> InfrastructureResult<()> {
        let mut ticker = interval(self.poll_interval);

        loop {
            ticker.tick().await;

            // 現在のProjection位置を取得
            let current_position = self
                .projection_db
                .get_position(&self.projection_name, self.projection_version)
                .await?;

            // 最新のイベントシーケンスを取得
            let latest_sequence = self.event_store.get_latest_sequence().await?;

            // 処理すべきイベントがあれば処理
            if current_position < latest_sequence.as_u64() {
                self.process_from(Sequence::new(current_position + 1)).await?;
            }
        }
    }

    /// 特定の集約IDのProjectionを更新
    pub async fn update_aggregate(&self, entry_id: &str) -> InfrastructureResult<()> {
        let agg_id = crate::types::AggregateId::parse(entry_id)
            .map_err(crate::error::InfrastructureError::DeserializationFailed)?;
        let stream = self.event_store.stream_aggregate_events(agg_id, Sequence::new(0));

        let mut projection = JournalEntryProjection::new(entry_id.to_string());

        for event_result in stream.iter() {
            let event = event_result?;

            let journal_event: JournalEntryEvent =
                serde_json::from_slice(&event.payload).map_err(|e| {
                    crate::error::InfrastructureError::DeserializationFailed(e.to_string())
                })?;

            use crate::projection_trait::Apply;
            projection.apply(journal_event)?;
        }

        // Projectionを保存
        use crate::projection_trait::ToReadModel;
        let read_model = projection.to_read_model();
        let value = serde_json::to_vec(&read_model)
            .map_err(|e| crate::error::InfrastructureError::SerializationFailed(e.to_string()))?;

        // 最新のイベントシーケンスを取得
        let latest_sequence = self.event_store.get_latest_sequence().await?;

        self.projection_db
            .update_projection(entry_id, &value, latest_sequence.as_u64())
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_projection_worker_rebuild() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());

        // イベント追加
        let event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

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

        // Projection再構築
        let worker = JournalEntryProjectionWorker::new(
            event_store,
            projection_db.clone(),
            "journal_entries".to_string(),
            1,
        );

        worker.rebuild().await.unwrap();

        // Projection確認
        let position = projection_db.get_position("journal_entries", 1).await.unwrap();
        assert!(position > 0);
    }

    #[tokio::test]
    async fn test_update_aggregate() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());

        let entry_id = "JE002";

        // 複数イベント追加
        let events = [
            JournalEntryEvent::DraftCreated {
                entry_id: entry_id.to_string(),
                transaction_date: "2024-01-01".to_string(),
                voucher_number: "V002".to_string(),
                lines: vec![],
                created_by: "user1".to_string(),
                created_at: Utc::now(),
            },
            JournalEntryEvent::ApprovalRequested {
                entry_id: entry_id.to_string(),
                requested_by: "user1".to_string(),
                requested_at: Utc::now(),
            },
        ];

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

        // 特定集約のProjection更新
        let worker = JournalEntryProjectionWorker::new(
            event_store,
            projection_db.clone(),
            "journal_entries".to_string(),
            1,
        );

        worker.update_aggregate(entry_id).await.unwrap();

        // Projection確認
        let projection_data = projection_db.get_projection(entry_id).await.unwrap();
        assert!(projection_data.is_some());
    }
}
