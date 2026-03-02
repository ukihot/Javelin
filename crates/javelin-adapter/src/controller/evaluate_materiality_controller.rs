// EvaluateMaterialityController - 重要性判定コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{EvaluateMaterialityRequest, EvaluateMaterialityResponse},
    input_ports::EvaluateMaterialityUseCase,
};

use crate::{error::AdapterResult, presenter::MaterialityEvaluationPresenter};

/// 重要性判定コントローラ
pub struct EvaluateMaterialityController<U>
where
    U: EvaluateMaterialityUseCase,
{
    use_case: Arc<U>,
}

impl<U> EvaluateMaterialityController<U>
where
    U: EvaluateMaterialityUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// 重要性判定処理
    pub async fn evaluate_materiality(
        &self,
        request: EvaluateMaterialityRequest,
    ) -> AdapterResult<EvaluateMaterialityResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 重要性判定処理（Presenter経由）
    pub async fn handle_evaluate_materiality(
        &self,
        request: EvaluateMaterialityRequest,
        presenter: Arc<MaterialityEvaluationPresenter>,
    ) {
        presenter.notify_progress("重要性判定処理を開始します".to_string()).await;
        presenter.notify_progress("財務指標を取得中...".to_string()).await;
        presenter.notify_progress("金額的重要性を判定中...".to_string()).await;

        match self.use_case.execute(request).await {
            Ok(response) => {
                presenter.notify_progress("重要性判定が完了しました".to_string()).await;
                presenter.present_result(response).await;
            }
            Err(e) => {
                presenter.notify_error(format!("重要性判定に失敗しました: {}", e)).await;
            }
        }
    }
}
