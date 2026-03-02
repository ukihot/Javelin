// ComprehensiveFinancialStatementsPresenter - 包括的財務諸表生成Presenter
// 責務: 包括的財務諸表生成結果をViewModelに変換してPageに通知

use javelin_application::dtos::GenerateComprehensiveFinancialStatementsResponse;
use tokio::sync::mpsc;

/// 包括的財務諸表生成ViewModel
#[derive(Debug, Clone)]
pub struct ComprehensiveFinancialStatementsViewModel {
    pub statement_count: usize,
    pub is_consistent: Option<bool>,
    pub inconsistency_count: usize,
    pub cross_check_passed: Option<bool>,
    pub checks_passed: usize,
    pub checks_failed: usize,
    pub approval_status: String,
    pub is_success: bool,
    pub error_message: Option<String>,
}

/// 包括的財務諸表生成Presenter
#[derive(Clone)]
pub struct ComprehensiveFinancialStatementsPresenter {
    result_sender: mpsc::UnboundedSender<ComprehensiveFinancialStatementsViewModel>,
    progress_sender: mpsc::UnboundedSender<String>,
}

/// チャネル作成の戻り値型
pub type ComprehensiveFinancialStatementsChannels = (
    mpsc::UnboundedSender<ComprehensiveFinancialStatementsViewModel>,
    mpsc::UnboundedReceiver<ComprehensiveFinancialStatementsViewModel>,
    mpsc::UnboundedSender<String>,
    mpsc::UnboundedReceiver<String>,
);

impl ComprehensiveFinancialStatementsPresenter {
    pub fn new(
        result_sender: mpsc::UnboundedSender<ComprehensiveFinancialStatementsViewModel>,
        progress_sender: mpsc::UnboundedSender<String>,
    ) -> Self {
        Self { result_sender, progress_sender }
    }

    /// チャネルを作成
    pub fn create_channels() -> ComprehensiveFinancialStatementsChannels {
        let (result_tx, result_rx) = mpsc::unbounded_channel();
        let (progress_tx, progress_rx) = mpsc::unbounded_channel();
        (result_tx, result_rx, progress_tx, progress_rx)
    }

    /// 進捗メッセージを通知
    pub async fn notify_progress(&self, message: String) {
        let _ = self.progress_sender.send(message);
    }

    /// エラーを通知
    pub async fn notify_error(&self, error_message: String) {
        let view_model = ComprehensiveFinancialStatementsViewModel {
            statement_count: 0,
            is_consistent: None,
            inconsistency_count: 0,
            cross_check_passed: None,
            checks_passed: 0,
            checks_failed: 0,
            approval_status: String::new(),
            is_success: false,
            error_message: Some(error_message),
        };
        let _ = self.result_sender.send(view_model);
    }

    /// 包括的財務諸表生成結果を通知
    pub async fn present_result(&self, response: GenerateComprehensiveFinancialStatementsResponse) {
        let view_model = ComprehensiveFinancialStatementsViewModel {
            statement_count: response.statements.len(),
            is_consistent: response.consistency_check.as_ref().map(|c| c.is_consistent),
            inconsistency_count: response
                .consistency_check
                .as_ref()
                .map(|c| c.inconsistency_count)
                .unwrap_or(0),
            cross_check_passed: response.cross_check.as_ref().map(|c| c.passed),
            checks_passed: response.cross_check.as_ref().map(|c| c.checks_passed).unwrap_or(0),
            checks_failed: response.cross_check.as_ref().map(|c| c.checks_failed).unwrap_or(0),
            approval_status: format!("{:?}", response.approval_status),
            is_success: true,
            error_message: None,
        };
        let _ = self.result_sender.send(view_model);
    }
}
