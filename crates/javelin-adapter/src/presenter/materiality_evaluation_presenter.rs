// MaterialityEvaluationPresenter - 重要性判定Presenter
// 責務: 重要性判定結果をViewModelに変換してPageに通知

use javelin_application::dtos::EvaluateMaterialityResponse;
use tokio::sync::mpsc;

/// 重要性判定ViewModel
#[derive(Debug, Clone)]
pub struct MaterialityEvaluationViewModel {
    pub judgment_id: String,
    pub is_material: bool,
    pub approval_level: String,
    pub threshold_type: String,
    pub threshold_amount: i64,
    pub threshold_excess_rate: Option<f64>,
    pub qualitative_materiality: Option<bool>,
    pub judgment_reason: String,
    pub is_success: bool,
    pub error_message: Option<String>,
}

/// 重要性判定Presenter
#[derive(Clone)]
pub struct MaterialityEvaluationPresenter {
    result_sender: mpsc::UnboundedSender<MaterialityEvaluationViewModel>,
    progress_sender: mpsc::UnboundedSender<String>,
}

/// チャネル作成の戻り値型
pub type MaterialityEvaluationChannels = (
    mpsc::UnboundedSender<MaterialityEvaluationViewModel>,
    mpsc::UnboundedReceiver<MaterialityEvaluationViewModel>,
    mpsc::UnboundedSender<String>,
    mpsc::UnboundedReceiver<String>,
);

impl MaterialityEvaluationPresenter {
    pub fn new(
        result_sender: mpsc::UnboundedSender<MaterialityEvaluationViewModel>,
        progress_sender: mpsc::UnboundedSender<String>,
    ) -> Self {
        Self { result_sender, progress_sender }
    }

    /// チャネルを作成
    pub fn create_channels() -> MaterialityEvaluationChannels {
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
        let view_model = MaterialityEvaluationViewModel {
            judgment_id: String::new(),
            is_material: false,
            approval_level: String::new(),
            threshold_type: String::new(),
            threshold_amount: 0,
            threshold_excess_rate: None,
            qualitative_materiality: None,
            judgment_reason: String::new(),
            is_success: false,
            error_message: Some(error_message),
        };
        let _ = self.result_sender.send(view_model);
    }

    /// 重要性判定結果を通知
    pub async fn present_result(&self, response: EvaluateMaterialityResponse) {
        let view_model = MaterialityEvaluationViewModel {
            judgment_id: response.judgment_id,
            is_material: response.is_material,
            approval_level: format!("{:?}", response.approval_level),
            threshold_type: response.applied_threshold.threshold_type,
            threshold_amount: response.applied_threshold.threshold_amount,
            threshold_excess_rate: response.threshold_excess_rate,
            qualitative_materiality: response.qualitative_materiality,
            judgment_reason: response.judgment_reason,
            is_success: true,
            error_message: None,
        };
        let _ = self.result_sender.send(view_model);
    }
}
