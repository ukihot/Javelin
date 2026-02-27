// BatchHistoryQueryServiceImpl - バッチ実行履歴クエリサービス実装
// 責務: バッチ実行履歴の読み取り（ProjectionDBから取得）

use std::sync::Arc;

use javelin_application::{
    error::ApplicationResult,
    query_service::{BatchHistoryQueryService, BatchHistoryRecord, GetBatchHistoryQuery},
};

use crate::read::projections::{
    batch_history_projection::BatchHistoryReadModel, projection_db::ProjectionDb,
};

pub struct BatchHistoryQueryServiceImpl {
    projection_db: Arc<ProjectionDb>,
}

impl BatchHistoryQueryServiceImpl {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection_db }
    }
}

// モダンプラクティス: async fn in traits は Rust 1.75+ で安定化済み
impl BatchHistoryQueryService for BatchHistoryQueryServiceImpl {
    async fn get_batch_history(
        &self,
        query: GetBatchHistoryQuery,
    ) -> ApplicationResult<Vec<BatchHistoryRecord>> {
        // ProjectionDBからバッチ履歴を取得
        // キー形式: "batch_history:{batch_type}:{execution_id}"

        let mut results = Vec::with_capacity(100);

        // バッチ種別でフィルタリングしながら取得
        // 注意: LMDBは範囲検索をサポートしているが、簡易実装として
        // 全件取得してフィルタリングする
        for i in 0..1000 {
            let key = format!("batch_history:{}:{:04}", query.batch_type, i);

            if let Some(data) = self.projection_db.get_projection(&key).await.map_err(|e| {
                javelin_application::error::ApplicationError::ProjectionDatabaseError(e.to_string())
            })? {
                let read_model: BatchHistoryReadModel =
                    serde_json::from_slice(&data).map_err(|e| {
                        javelin_application::error::ApplicationError::ProjectionDatabaseError(
                            e.to_string(),
                        )
                    })?;

                results.push(BatchHistoryRecord {
                    execution_id: read_model.execution_id,
                    batch_type: read_model.batch_type,
                    executed_at: read_model.started_at,
                    status: read_model.status,
                    duration_seconds: read_model.duration_seconds,
                    processed_count: read_model.processed_count.unwrap_or(0),
                    result_summary: read_model
                        .result_summary
                        .or(read_model.error_message)
                        .unwrap_or_else(|| "実行中".to_string()),
                });

                // limit適用
                if let Some(limit) = query.limit
                    && results.len() >= limit
                {
                    break;
                }
            } else {
                // データが見つからなくなったら終了
                break;
            }
        }

        // 実行日時の降順でソート（新しい順）
        results.sort_by(|a, b| b.executed_at.cmp(&a.executed_at));

        Ok(results)
    }
}

impl Default for BatchHistoryQueryServiceImpl {
    fn default() -> Self {
        // デフォルトパスでProjectionDBを作成
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let projection_db = runtime
            .block_on(ProjectionDb::new(std::path::Path::new("./data/projections")))
            .expect("Failed to create ProjectionDb");

        Self::new(Arc::new(projection_db))
    }
}
