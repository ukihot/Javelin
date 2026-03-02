// GenerateComprehensiveFinancialStatementsController - 包括的財務諸表生成コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{
        GenerateComprehensiveFinancialStatementsRequest,
        GenerateComprehensiveFinancialStatementsResponse,
    },
    input_ports::GenerateComprehensiveFinancialStatementsUseCase,
};

use crate::{error::AdapterResult, presenter::ComprehensiveFinancialStatementsPresenter};

/// 包括的財務諸表生成コントローラ
pub struct GenerateComprehensiveFinancialStatementsController<U>
where
    U: GenerateComprehensiveFinancialStatementsUseCase,
{
    use_case: Arc<U>,
}

impl<U> GenerateComprehensiveFinancialStatementsController<U>
where
    U: GenerateComprehensiveFinancialStatementsUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// 包括的財務諸表生成処理
    pub async fn generate_comprehensive_financial_statements(
        &self,
        request: GenerateComprehensiveFinancialStatementsRequest,
    ) -> AdapterResult<GenerateComprehensiveFinancialStatementsResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 包括的財務諸表生成処理（Presenter経由）
    pub async fn handle_generate_comprehensive_financial_statements(
        &self,
        request: GenerateComprehensiveFinancialStatementsRequest,
        presenter: Arc<ComprehensiveFinancialStatementsPresenter>,
    ) {
        presenter
            .notify_progress("包括的財務諸表生成処理を開始します".to_string())
            .await;
        presenter.notify_progress("元帳データを取得中...".to_string()).await;
        presenter.notify_progress("貸借対照表を生成中...".to_string()).await;

        match self.use_case.execute(request).await {
            Ok(response) => {
                presenter.notify_progress("損益計算書を生成中...".to_string()).await;
                presenter.notify_progress("キャッシュフロー計算書を生成中...".to_string()).await;
                presenter.notify_progress("整合性を検証中...".to_string()).await;
                presenter.notify_progress("クロスチェックを実行中...".to_string()).await;
                presenter.notify_progress("包括的財務諸表生成が完了しました".to_string()).await;
                presenter.present_result(response).await;
            }
            Err(e) => {
                presenter.notify_error(format!("包括的財務諸表生成に失敗しました: {}", e)).await;
            }
        }
    }
}
