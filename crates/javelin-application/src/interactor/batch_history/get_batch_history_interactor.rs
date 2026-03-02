// GetBatchHistoryInteractor - バッチ履歴取得インタラクター

use std::sync::Arc;

use crate::{
    error::ApplicationResult,
    input_ports::GetBatchHistoryUseCase,
    query_service::{BatchHistoryQueryService, BatchHistoryRecord, GetBatchHistoryQuery},
};

/// バッチ履歴取得インタラクター
pub struct GetBatchHistoryInteractor<Q>
where
    Q: BatchHistoryQueryService,
{
    query_service: Arc<Q>,
}

impl<Q> GetBatchHistoryInteractor<Q>
where
    Q: BatchHistoryQueryService,
{
    /// 新しいインタラクターインスタンスを作成
    pub fn new(query_service: Arc<Q>) -> Self {
        Self { query_service }
    }
}

impl<Q> GetBatchHistoryUseCase for GetBatchHistoryInteractor<Q>
where
    Q: BatchHistoryQueryService,
{
    async fn execute(
        &self,
        batch_type: String,
        limit: Option<usize>,
    ) -> ApplicationResult<Vec<BatchHistoryRecord>> {
        let query = GetBatchHistoryQuery { batch_type, limit };
        self.query_service.get_batch_history(query).await
    }
}
