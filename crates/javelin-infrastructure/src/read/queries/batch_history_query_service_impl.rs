// BatchHistoryQueryServiceImpl - バッチ実行履歴クエリサービス実装
// 責務: バッチ実行履歴の読み取り（モックデータ）

use async_trait::async_trait;
use javelin_application::{
    error::ApplicationResult,
    query_service::{BatchHistoryQueryService, BatchHistoryRecord, GetBatchHistoryQuery},
};

pub struct BatchHistoryQueryServiceImpl;

impl BatchHistoryQueryServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BatchHistoryQueryService for BatchHistoryQueryServiceImpl {
    async fn get_batch_history(
        &self,
        query: GetBatchHistoryQuery,
    ) -> ApplicationResult<Vec<BatchHistoryRecord>> {
        // TODO: 実際のデータベースから取得
        // 現在はモックデータを返す
        let mock_data = match query.batch_type.as_str() {
            "LedgerConsolidation" => vec![
                BatchHistoryRecord {
                    execution_id: "20240224-001".to_string(),
                    batch_type: "LedgerConsolidation".to_string(),
                    executed_at: "2024-02-24 10:30:00".to_string(),
                    status: "Completed".to_string(),
                    duration_seconds: Some(150),
                    processed_count: 150,
                    result_summary: "正常終了".to_string(),
                },
                BatchHistoryRecord {
                    execution_id: "20240223-002".to_string(),
                    batch_type: "LedgerConsolidation".to_string(),
                    executed_at: "2024-02-23 15:45:00".to_string(),
                    status: "Completed".to_string(),
                    duration_seconds: Some(135),
                    processed_count: 145,
                    result_summary: "正常終了".to_string(),
                },
                BatchHistoryRecord {
                    execution_id: "20240222-001".to_string(),
                    batch_type: "LedgerConsolidation".to_string(),
                    executed_at: "2024-02-22 09:00:00".to_string(),
                    status: "Failed".to_string(),
                    duration_seconds: Some(65),
                    processed_count: 0,
                    result_summary: "データ検証エラー".to_string(),
                },
            ],
            "ClosingPreparation" => vec![
                BatchHistoryRecord {
                    execution_id: "20240224-001".to_string(),
                    batch_type: "ClosingPreparation".to_string(),
                    executed_at: "2024-02-24 16:00:00".to_string(),
                    status: "Completed".to_string(),
                    duration_seconds: Some(320),
                    processed_count: 320,
                    result_summary: "正常終了".to_string(),
                },
                BatchHistoryRecord {
                    execution_id: "20240131-001".to_string(),
                    batch_type: "ClosingPreparation".to_string(),
                    executed_at: "2024-01-31 17:30:00".to_string(),
                    status: "Completed".to_string(),
                    duration_seconds: Some(345),
                    processed_count: 315,
                    result_summary: "正常終了".to_string(),
                },
            ],
            "AccountAdjustment" => vec![BatchHistoryRecord {
                execution_id: "20240224-001".to_string(),
                batch_type: "AccountAdjustment".to_string(),
                executed_at: "2024-02-24 14:00:00".to_string(),
                status: "Completed".to_string(),
                duration_seconds: Some(190),
                processed_count: 85,
                result_summary: "振替45件、税効果40件".to_string(),
            }],
            "IfrsValuation" => vec![BatchHistoryRecord {
                execution_id: "20240224-001".to_string(),
                batch_type: "IfrsValuation".to_string(),
                executed_at: "2024-02-24 11:00:00".to_string(),
                status: "Completed".to_string(),
                duration_seconds: Some(510),
                processed_count: 250,
                result_summary: "公正価値評価完了".to_string(),
            }],
            "FinancialStatement" => vec![BatchHistoryRecord {
                execution_id: "20240224-001".to_string(),
                batch_type: "FinancialStatement".to_string(),
                executed_at: "2024-02-24 17:00:00".to_string(),
                status: "Completed".to_string(),
                duration_seconds: Some(285),
                processed_count: 5,
                result_summary: "BS/PL/CF/注記生成完了".to_string(),
            }],
            _ => vec![],
        };

        let limited = if let Some(limit) = query.limit {
            mock_data.into_iter().take(limit).collect()
        } else {
            mock_data
        };

        Ok(limited)
    }
}

impl Default for BatchHistoryQueryServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
