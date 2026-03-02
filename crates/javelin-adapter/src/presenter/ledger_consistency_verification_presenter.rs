// LedgerConsistencyVerificationPresenter - 元帳整合性検証Presenter
// 責務: 元帳整合性検証結果をViewModelに変換してPageに通知

use javelin_application::dtos::VerifyLedgerConsistencyResponse;
use tokio::sync::mpsc;

/// 元帳整合性検証ViewModel
#[derive(Debug, Clone)]
pub struct LedgerConsistencyVerificationViewModel {
    pub verification_id: String,
    pub is_consistent: bool,
    pub discrepancy_count: usize,
    pub anomaly_alert_count: usize,
    pub temporary_account_count: usize,
    pub balance_change_count: usize,
    pub is_success: bool,
    pub error_message: Option<String>,
}

/// 元帳整合性検証Presenter
#[derive(Clone)]
pub struct LedgerConsistencyVerificationPresenter {
    result_sender: mpsc::UnboundedSender<LedgerConsistencyVerificationViewModel>,
    progress_sender: mpsc::UnboundedSender<String>,
}

/// チャネル作成の戻り値型
pub type LedgerConsistencyVerificationChannels = (
    mpsc::UnboundedSender<LedgerConsistencyVerificationViewModel>,
    mpsc::UnboundedReceiver<LedgerConsistencyVerificationViewModel>,
    mpsc::UnboundedSender<String>,
    mpsc::UnboundedReceiver<String>,
);

impl LedgerConsistencyVerificationPresenter {
    pub fn new(
        result_sender: mpsc::UnboundedSender<LedgerConsistencyVerificationViewModel>,
        progress_sender: mpsc::UnboundedSender<String>,
    ) -> Self {
        Self { result_sender, progress_sender }
    }

    /// チャネルを作成
    pub fn create_channels() -> LedgerConsistencyVerificationChannels {
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
        let view_model = LedgerConsistencyVerificationViewModel {
            verification_id: String::new(),
            is_consistent: false,
            discrepancy_count: 0,
            anomaly_alert_count: 0,
            temporary_account_count: 0,
            balance_change_count: 0,
            is_success: false,
            error_message: Some(error_message),
        };
        let _ = self.result_sender.send(view_model);
    }

    /// 元帳整合性検証結果を通知
    pub async fn present_result(&self, response: VerifyLedgerConsistencyResponse) {
        let view_model = LedgerConsistencyVerificationViewModel {
            verification_id: response.verification_id,
            is_consistent: response.is_consistent,
            discrepancy_count: response.discrepancy_count,
            anomaly_alert_count: response.anomaly_alerts.as_ref().map(|a| a.len()).unwrap_or(0),
            temporary_account_count: response
                .temporary_accounts
                .as_ref()
                .map(|t| t.len())
                .unwrap_or(0),
            balance_change_count: response.balance_changes.as_ref().map(|b| b.len()).unwrap_or(0),
            is_success: true,
            error_message: None,
        };
        let _ = self.result_sender.send(view_model);
    }
}
