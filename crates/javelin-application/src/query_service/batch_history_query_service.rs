// BatchHistoryQueryService - バッチ実行履歴クエリサービス
// 責務: バッチ実行履歴の読み取り専用クエリ

use std::future::Future;

use crate::error::ApplicationResult;

/// バッチ実行履歴項目
#[derive(Debug, Clone)]
pub struct BatchHistoryRecord {
    /// 実行ID
    pub execution_id: String,
    /// バッチ種別
    pub batch_type: String,
    /// 実行日時
    pub executed_at: String,
    /// 状態（Completed/Failed/Running）
    pub status: String,
    /// 実行時間（秒）
    pub duration_seconds: Option<u32>,
    /// 処理件数
    pub processed_count: usize,
    /// 結果サマリー
    pub result_summary: String,
}

/// バッチ実行履歴クエリ
#[derive(Debug, Clone)]
pub struct GetBatchHistoryQuery {
    /// バッチ種別（LedgerConsolidation, ClosingPreparation, etc.）
    pub batch_type: String,
    /// 取得件数制限
    pub limit: Option<usize>,
}

/// バッチ実行履歴クエリサービス
/// モダンプラクティス: async fn in traits は Rust 1.75+ で安定化済み
/// 戻り値の型を明示的に `impl Future + Send` として指定することで、
/// auto trait bounds を明確にする。
pub trait BatchHistoryQueryService: Send + Sync {
    /// バッチ実行履歴を取得
    fn get_batch_history(
        &self,
        query: GetBatchHistoryQuery,
    ) -> impl Future<Output = ApplicationResult<Vec<BatchHistoryRecord>>> + Send;
}
