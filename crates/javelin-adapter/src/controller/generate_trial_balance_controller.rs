// GenerateTrialBalanceController - 試算表生成コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{GenerateTrialBalanceRequest, GenerateTrialBalanceResponse},
    input_ports::GenerateTrialBalanceUseCase,
};

use crate::error::AdapterResult;

/// 試算表生成コントローラ
pub struct GenerateTrialBalanceController<U>
where
    U: GenerateTrialBalanceUseCase,
{
    use_case: Arc<U>,
}

impl<U> GenerateTrialBalanceController<U>
where
    U: GenerateTrialBalanceUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// 試算表生成処理
    pub async fn generate_trial_balance(
        &self,
        request: GenerateTrialBalanceRequest,
    ) -> AdapterResult<GenerateTrialBalanceResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }
}
