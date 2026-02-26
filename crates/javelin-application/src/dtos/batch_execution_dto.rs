// BatchExecutionDTO - バッチ実行の進捗情報
// 責務: バッチ処理の進捗状態をアダプター層に伝達

use serde::{Deserialize, Serialize};

/// バッチ実行ステップの状態
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BatchStepStatus {
    /// 待機中
    Waiting,
    /// 実行中
    Running,
    /// 完了
    Completed,
    /// エラー
    Failed { message: String },
}

/// バッチ実行ステップ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchExecutionStep {
    /// ステップID
    pub id: String,
    /// ステップ名
    pub name: String,
    /// 状態
    pub status: BatchStepStatus,
    /// 進捗率（0-100）
    pub progress: u8,
}

/// バッチ実行の進捗情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchExecutionProgress {
    /// バッチ処理ID
    pub batch_id: String,
    /// バッチ処理名
    pub batch_name: String,
    /// ステップリスト
    pub steps: Vec<BatchExecutionStep>,
    /// 全体の進捗率（0-100）
    pub overall_progress: u8,
    /// ログメッセージ
    pub log_messages: Vec<String>,
}

/// バッチ実行開始リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartBatchExecutionRequest {
    /// バッチ処理ID
    pub batch_id: String,
}

/// バッチ実行開始レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartBatchExecutionResponse {
    /// 実行が開始されたか
    pub started: bool,
    /// 初期進捗情報
    pub progress: BatchExecutionProgress,
}
