// 現代Rust設計によるProjectionWorker実装
// - Iterator指向のイベント処理
// - Trait化されたProjection適用
// - std::iter::chain によるストリーム合成

use std::sync::Arc;

use tokio::time::{Duration, interval};

use crate::{
    error::InfrastructureResult, event_store::EventStore, event_stream::StoredEvent,
    projection_db::ProjectionDb, projection_trait::ProjectionStrategy, types::Sequence,
};

/// ProjectionWorker実装
pub struct ProjectionWorker<S: ProjectionStrategy> {
    event_store: Arc<EventStore>,
    projection_db: Arc<ProjectionDb>,
    projection_name: String,
    projection_version: u32,
    strategy: S,
    poll_interval: Duration,
}

impl<S: ProjectionStrategy> ProjectionWorker<S> {
    pub fn new(
        event_store: Arc<EventStore>,
        projection_db: Arc<ProjectionDb>,
        projection_name: String,
        projection_version: u32,
        strategy: S,
    ) -> Self {
        Self {
            event_store,
            projection_db,
            projection_name,
            projection_version,
            strategy,
            poll_interval: Duration::from_secs(1),
        }
    }

    pub fn with_poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Projectionを開始位置から再構築
    pub async fn rebuild(&self) -> InfrastructureResult<()> {
        self.process_from(Sequence::new(0)).await
    }

    /// 指定位置からProjectionを処理
    pub async fn process_from(&self, from_sequence: Sequence) -> InfrastructureResult<()> {
        let stream = self.event_store.stream_events(from_sequence);
        let batch_size = self.strategy.batch_size();

        let mut batch = Vec::new();
        let mut last_sequence = from_sequence;

        for event_result in stream.iter() {
            let event = event_result?;

            // 戦略に基づいてフィルタリング
            if !self.strategy.should_update(&event) {
                continue;
            }

            batch.push(event.clone());
            last_sequence = Sequence::new(event.global_sequence);

            // バッチサイズに達したら処理
            if batch.len() >= batch_size {
                self.process_batch(&batch, last_sequence).await?;
                batch.clear();
            }
        }

        // 残りのバッチを処理
        if !batch.is_empty() {
            self.process_batch(&batch, last_sequence).await?;
        }

        Ok(())
    }

    /// バッチ処理
    async fn process_batch(
        &self,
        events: &[StoredEvent],
        last_sequence: Sequence,
    ) -> InfrastructureResult<()> {
        let mut updates = Vec::new();

        for event in events {
            // ここでイベントをProjectionに変換
            // 実際の実装では、イベントタイプに応じた処理を行う
            let key = format!("{}:{}", event.aggregate_id, event.version);
            let value = serde_json::to_vec(event).map_err(|e| {
                crate::error::InfrastructureError::SerializationFailed(e.to_string())
            })?;

            updates.push((key, value));
        }

        // バッチ更新
        self.projection_db
            .update_projection_batch(
                &self.projection_name,
                self.projection_version,
                updates,
                last_sequence.as_u64(),
            )
            .await?;

        Ok(())
    }

    /// 継続的にProjectionを更新（ポーリング）
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

    /// 複数のイベントストリームを合成して処理
    /// std::iter::chain の活用例
    pub async fn process_multiple_streams(
        &self,
        streams: Vec<Sequence>,
    ) -> InfrastructureResult<()> {
        // 複数のストリームを chain で合成
        let mut combined_events = Vec::new();

        for start_seq in streams {
            let stream = self.event_store.stream_events(start_seq);
            for event_result in stream.iter() {
                combined_events.push(event_result?);
            }
        }

        // 合成されたイベントを処理
        let mut batch = Vec::new();
        let mut last_sequence = Sequence::new(0);

        for event in combined_events {
            if self.strategy.should_update(&event) {
                batch.push(event.clone());
                last_sequence = Sequence::new(event.global_sequence);

                if batch.len() >= self.strategy.batch_size() {
                    self.process_batch(&batch, last_sequence).await?;
                    batch.clear();
                }
            }
        }

        if !batch.is_empty() {
            self.process_batch(&batch, last_sequence).await?;
        }

        Ok(())
    }
}
