// VerifyLedgerConsistencyController - 元帳整合性検証コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{VerifyLedgerConsistencyRequest, VerifyLedgerConsistencyResponse},
    input_ports::VerifyLedgerConsistencyUseCase,
};

use crate::{error::AdapterResult, presenter::LedgerConsistencyVerificationPresenter};

/// 元帳整合性検証コントローラ
pub struct VerifyLedgerConsistencyController<U>
where
    U: VerifyLedgerConsistencyUseCase,
{
    use_case: Arc<U>,
}

impl<U> VerifyLedgerConsistencyController<U>
where
    U: VerifyLedgerConsistencyUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// 元帳整合性検証処理
    pub async fn verify_ledger_consistency(
        &self,
        request: VerifyLedgerConsistencyRequest,
    ) -> AdapterResult<VerifyLedgerConsistencyResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 元帳整合性検証処理（Presenter経由）
    pub async fn handle_verify_ledger_consistency(
        &self,
        request: VerifyLedgerConsistencyRequest,
        presenter: Arc<LedgerConsistencyVerificationPresenter>,
    ) {
        presenter.notify_progress("元帳整合性検証処理を開始します".to_string()).await;
        presenter.notify_progress("元帳データを取得中...".to_string()).await;
        presenter.notify_progress("基本整合性を検証中...".to_string()).await;

        match self.use_case.execute(request).await {
            Ok(response) => {
                presenter.notify_progress("残高変動を分析中...".to_string()).await;
                presenter.notify_progress("異常値を検出中...".to_string()).await;
                presenter.notify_progress("仮勘定を分析中...".to_string()).await;
                presenter.notify_progress("元帳整合性検証が完了しました".to_string()).await;
                presenter.present_result(response).await;
            }
            Err(e) => {
                presenter.notify_error(format!("元帳整合性検証に失敗しました: {}", e)).await;
            }
        }
    }
}
