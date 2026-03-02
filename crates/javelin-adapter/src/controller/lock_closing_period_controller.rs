// LockClosingPeriodController - 締日固定コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{LockClosingPeriodRequest, LockClosingPeriodResponse},
    input_ports::LockClosingPeriodUseCase,
};

use crate::error::AdapterResult;

/// 締日固定コントローラ
pub struct LockClosingPeriodController<U>
where
    U: LockClosingPeriodUseCase,
{
    use_case: Arc<U>,
}

impl<U> LockClosingPeriodController<U>
where
    U: LockClosingPeriodUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// 締日固定処理
    pub async fn lock_closing_period(
        &self,
        request: LockClosingPeriodRequest,
    ) -> AdapterResult<LockClosingPeriodResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }
}
