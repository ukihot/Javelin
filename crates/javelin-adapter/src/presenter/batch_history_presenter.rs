// BatchHistoryPresenter実装
// バッチ実行履歴の出力を整形してビューに渡す

use javelin_application::query_service::BatchHistoryRecord;
use tokio::sync::mpsc;

use crate::views::layouts::templates::BatchHistoryItem;

/// バッチ履歴ViewModel
#[derive(Debug, Clone)]
pub struct BatchHistoryViewModel {
    pub items: Vec<BatchHistoryItem>,
}

/// バッチ履歴Presenter
#[derive(Clone)]
pub struct BatchHistoryPresenter {
    result_tx: mpsc::Sender<BatchHistoryViewModel>,
    error_tx: mpsc::Sender<String>,
}

pub struct BatchHistoryChannels {
    pub result_rx: mpsc::Receiver<BatchHistoryViewModel>,
    pub error_rx: mpsc::Receiver<String>,
}

impl BatchHistoryPresenter {
    pub fn new(
        result_tx: mpsc::Sender<BatchHistoryViewModel>,
        error_tx: mpsc::Sender<String>,
    ) -> Self {
        Self { result_tx, error_tx }
    }

    pub fn create_channels() -> (BatchHistoryPresenter, BatchHistoryChannels) {
        let (result_tx, result_rx) = mpsc::channel(100);
        let (error_tx, error_rx) = mpsc::channel(100);

        let presenter = BatchHistoryPresenter::new(result_tx, error_tx);
        let channels = BatchHistoryChannels { result_rx, error_rx };

        (presenter, channels)
    }

    /// ステータスの日本語表示を取得
    fn format_status(status: &str) -> String {
        match status {
            "Completed" => "完了",
            "Failed" => "エラー",
            "Running" => "実行中",
            _ => "不明",
        }
        .to_string()
    }

    /// 実行時間をフォーマット
    fn format_duration(seconds: Option<u32>) -> String {
        match seconds {
            Some(s) if s < 60 => format!("{}秒", s),
            Some(s) => {
                let minutes = s / 60;
                let secs = s % 60;
                format!("{}分{:02}秒", minutes, secs)
            }
            None => "-".to_string(),
        }
    }

    /// DTOからViewModelへの変換
    fn to_view_model(&self, records: Vec<BatchHistoryRecord>) -> BatchHistoryViewModel {
        let items = records
            .into_iter()
            .map(|record| BatchHistoryItem {
                execution_id: record.execution_id,
                executed_at: record.executed_at,
                status: Self::format_status(&record.status),
                duration: Self::format_duration(record.duration_seconds),
                processed_count: record.processed_count,
                result_summary: record.result_summary,
            })
            .collect();

        BatchHistoryViewModel { items }
    }

    /// 履歴結果を提示
    pub fn present_history(&self, records: Vec<BatchHistoryRecord>) {
        let view_model = self.to_view_model(records);
        let _ = self.result_tx.try_send(view_model);
    }

    /// エラーを提示
    pub fn present_error(&self, error_message: String) {
        let _ = self.error_tx.try_send(error_message);
    }

    /// 結果なしを提示
    pub fn present_no_results(&self) {
        let view_model = BatchHistoryViewModel { items: vec![] };
        let _ = self.result_tx.try_send(view_model);
    }
}
