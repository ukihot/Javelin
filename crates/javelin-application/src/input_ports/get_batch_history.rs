// GetBatchHistoryUseCase - バッチ履歴取得ユースケース

use crate::{error::ApplicationResult, query_service::BatchHistoryRecord};

/// バッチ履歴取得ユースケース
#[allow(async_fn_in_trait)]
pub trait GetBatchHistoryUseCase: Send + Sync {
    /// バッチ履歴を取得
    ///
    /// # Arguments
    /// * `batch_type` - バッチタイプ
    /// * `limit` - 取得件数上限
    ///
    /// # Returns
    /// * `Ok(Vec<BatchHistoryRecord>)` - 取得成功
    /// * `Err(ApplicationError)` - 取得失敗
    async fn execute(
        &self,
        batch_type: String,
        limit: Option<usize>,
    ) -> ApplicationResult<Vec<BatchHistoryRecord>>;
}
