// SearchPresenter実装
// 仕訳検索結果の出力を整形してビューに渡す

use javelin_application::{
    dtos::response::JournalEntrySearchResultDto, output_ports::SearchOutputPort,
};
use tokio::sync::mpsc;

/// 検索結果ViewModel
#[derive(Debug, Clone)]
pub struct SearchResultViewModel {
    pub items: Vec<JournalEntryItemViewModel>,
    pub total_count: usize,
}

/// 仕訳項目ViewModel
#[derive(Debug, Clone)]
pub struct JournalEntryItemViewModel {
    pub entry_id: String,
    pub entry_number: Option<String>,
    pub status: String,
    pub status_label: String,
    pub transaction_date: String,
    pub lines: Vec<JournalEntryLineItemViewModel>,
}

/// 仕訳明細項目ViewModel
#[derive(Debug, Clone)]
pub struct JournalEntryLineItemViewModel {
    pub line_number: u32,
    pub side: String,
    pub side_label: String,
    pub account_code: String,
    pub account_name: String,
    pub description: String,
    pub amount: f64,
}

/// 検索Presenter
#[derive(Clone)]
pub struct SearchPresenter {
    result_tx: mpsc::Sender<SearchResultViewModel>,
    error_tx: mpsc::Sender<String>,
    progress_tx: mpsc::Sender<String>,
    execution_time_tx: mpsc::Sender<usize>,
}

pub struct SearchChannels {
    pub result_rx: mpsc::Receiver<SearchResultViewModel>,
    pub error_rx: mpsc::Receiver<String>,
    pub progress_rx: mpsc::Receiver<String>,
    pub execution_time_rx: mpsc::Receiver<usize>,
}

impl SearchPresenter {
    pub fn new(
        result_tx: mpsc::Sender<SearchResultViewModel>,
        error_tx: mpsc::Sender<String>,
        progress_tx: mpsc::Sender<String>,
        execution_time_tx: mpsc::Sender<usize>,
    ) -> Self {
        Self { result_tx, error_tx, progress_tx, execution_time_tx }
    }

    pub fn create_channels() -> (SearchPresenter, SearchChannels) {
        let (result_tx, result_rx) = mpsc::channel(100);
        let (error_tx, error_rx) = mpsc::channel(100);
        let (progress_tx, progress_rx) = mpsc::channel(100);
        let (execution_time_tx, execution_time_rx) = mpsc::channel(100);

        let presenter = SearchPresenter::new(result_tx, error_tx, progress_tx, execution_time_tx);
        let channels = SearchChannels { result_rx, error_rx, progress_rx, execution_time_rx };

        (presenter, channels)
    }

    /// ステータスの日本語表示を取得
    fn format_status_label(status: &str) -> String {
        match status {
            "Draft" => "下書き",
            "PendingApproval" => "承認待ち",
            "Posted" => "記帳済",
            "Reversed" => "取消済",
            "Corrected" => "修正済",
            "Closed" => "締め済",
            "Deleted" => "削除済",
            _ => "不明",
        }
        .to_string()
    }

    /// 借方貸方の日本語表示を取得
    fn format_side_label(side: &str) -> String {
        match side {
            "Debit" => "借方",
            "Credit" => "貸方",
            _ => "不明",
        }
        .to_string()
    }

    /// DTOからViewModelへの変換
    fn to_view_model(&self, dto: JournalEntrySearchResultDto) -> SearchResultViewModel {
        let items = dto
            .entries
            .into_iter()
            .map(|entry| {
                let lines = entry
                    .lines
                    .into_iter()
                    .map(|line| JournalEntryLineItemViewModel {
                        line_number: line.line_number,
                        side: line.side.clone(),
                        side_label: Self::format_side_label(&line.side),
                        account_code: line.account_code,
                        account_name: line.account_name,
                        description: line.description.unwrap_or_default(),
                        amount: line.amount,
                    })
                    .collect();

                JournalEntryItemViewModel {
                    entry_id: entry.entry_id,
                    entry_number: entry.entry_number,
                    status: entry.status.clone(),
                    status_label: Self::format_status_label(&entry.status),
                    transaction_date: entry.transaction_date,
                    lines,
                }
            })
            .collect();

        SearchResultViewModel { items, total_count: dto.total_count as usize }
    }
}

impl SearchOutputPort for SearchPresenter {
    fn notify_error(&self, error_message: String) {
        eprintln!("[Search Error] {}", error_message);
    }

    fn present_search_result(&self, result: JournalEntrySearchResultDto) {
        let view_model = self.to_view_model(result);
        let _ = self.result_tx.try_send(view_model);
    }

    fn present_validation_error(&self, error_message: String) {
        let _ = self.error_tx.try_send(error_message);
    }

    fn present_no_results(&self) {
        let view_model = SearchResultViewModel { items: vec![], total_count: 0 };
        let _ = self.result_tx.try_send(view_model);
    }

    fn present_progress(&self, message: String) {
        let _ = self.progress_tx.try_send(message);
    }

    fn present_execution_time(&self, elapsed_ms: usize) {
        let _ = self.execution_time_tx.try_send(elapsed_ms);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_status_label() {
        assert_eq!(SearchPresenter::format_status_label("Draft"), "下書き");
        assert_eq!(SearchPresenter::format_status_label("Posted"), "記帳済");
        assert_eq!(SearchPresenter::format_status_label("Reversed"), "取消済");
        assert_eq!(SearchPresenter::format_status_label("Unknown"), "不明");
    }

    #[test]
    fn test_format_side_label() {
        assert_eq!(SearchPresenter::format_side_label("Debit"), "借方");
        assert_eq!(SearchPresenter::format_side_label("Credit"), "貸方");
        assert_eq!(SearchPresenter::format_side_label("Unknown"), "不明");
    }

    #[tokio::test]
    async fn test_present_search_result() {
        let (presenter, mut channels) = SearchPresenter::create_channels();

        let dto = JournalEntrySearchResultDto { entries: vec![], total_count: 0 };

        presenter.present_search_result(dto);

        let result = channels.result_rx.recv().await.unwrap();
        assert_eq!(result.total_count, 0);
        assert_eq!(result.items.len(), 0);
    }

    #[tokio::test]
    async fn test_present_validation_error() {
        let (presenter, mut channels) = SearchPresenter::create_channels();

        presenter.present_validation_error("Invalid date range".to_string());

        let error = channels.error_rx.recv().await.unwrap();
        assert_eq!(error, "Invalid date range");
    }

    #[tokio::test]
    async fn test_present_no_results() {
        let (presenter, mut channels) = SearchPresenter::create_channels();

        presenter.present_no_results();

        let result = channels.result_rx.recv().await.unwrap();
        assert_eq!(result.total_count, 0);
        assert_eq!(result.items.len(), 0);
    }
}
