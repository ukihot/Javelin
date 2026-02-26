// BatchHistoryController実装
// バッチ実行履歴に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::query_service::{BatchHistoryQueryService, GetBatchHistoryQuery};
use javelin_infrastructure::read::queries::BatchHistoryQueryServiceImpl;

use crate::navigation::PresenterRegistry;

/// バッチ履歴コントローラ
///
/// バッチ実行履歴に関するすべての操作を受け付ける。
/// クエリサービスへの委譲のみを行い、ビジネスロジックは含まない。
pub struct BatchHistoryController {
    query_service: Arc<BatchHistoryQueryServiceImpl>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl BatchHistoryController {
    /// 新しいコントローラインスタンスを作成
    pub fn new(
        query_service: Arc<BatchHistoryQueryServiceImpl>,
        presenter_registry: Arc<PresenterRegistry>,
    ) -> Self {
        Self { query_service, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// バッチ実行履歴を取得
    ///
    /// # Arguments
    /// * `page_id` - ページインスタンスID（PresenterRegistry検索用）
    /// * `batch_type` - バッチタイプ（例: "LedgerConsolidation", "ClosingPreparation"）
    ///
    /// # Returns
    /// * `Ok(())` - 取得成功（結果はPresenter経由で通知）
    /// * `Err(String)` - 取得失敗
    pub async fn handle_get_history(
        &self,
        page_id: uuid::Uuid,
        batch_type: String,
    ) -> Result<(), String> {
        // PresenterRegistryからpage_id用のPresenterを取得
        if let Some(presenter_arc) = self.presenter_registry.get_batch_history_presenter(page_id) {
            let query = GetBatchHistoryQuery { batch_type, limit: Some(100) };

            // クエリサービスを実行
            match self.query_service.get_batch_history(query).await {
                Ok(records) => {
                    if records.is_empty() {
                        presenter_arc.present_no_results();
                    } else {
                        presenter_arc.present_history(records);
                    }
                    Ok(())
                }
                Err(e) => {
                    presenter_arc.present_error(e.to_string());
                    Err(e.to_string())
                }
            }
        } else {
            Err(format!("BatchHistoryPresenter not found for page_id: {}", page_id))
        }
    }
}
