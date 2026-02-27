// Batch Execution Events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// バッチ実行イベント
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BatchExecutionEvent {
    /// バッチ実行開始
    BatchStarted {
        execution_id: String,
        batch_type: String,
        started_at: DateTime<Utc>,
        started_by: String,
    },

    /// バッチ実行完了
    BatchCompleted {
        execution_id: String,
        completed_at: DateTime<Utc>,
        duration_seconds: u32,
        processed_count: usize,
        result_summary: String,
    },

    /// バッチ実行失敗
    BatchFailed {
        execution_id: String,
        failed_at: DateTime<Utc>,
        duration_seconds: u32,
        error_message: String,
    },
}

impl BatchExecutionEvent {
    /// イベントタイプを取得
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::BatchStarted { .. } => "BatchStarted",
            Self::BatchCompleted { .. } => "BatchCompleted",
            Self::BatchFailed { .. } => "BatchFailed",
        }
    }

    /// 集約IDを取得
    pub fn aggregate_id(&self) -> &str {
        match self {
            Self::BatchStarted { execution_id, .. }
            | Self::BatchCompleted { execution_id, .. }
            | Self::BatchFailed { execution_id, .. } => execution_id,
        }
    }
}
