// PrepareClosingController - 締準備コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{PrepareClosingRequest, PrepareClosingResponse},
    input_ports::PrepareClosingUseCase,
};

use crate::error::AdapterResult;

/// 締準備コントローラ
pub struct PrepareClosingController<U>
where
    U: PrepareClosingUseCase,
{
    use_case: Arc<U>,
}

impl<U> PrepareClosingController<U>
where
    U: PrepareClosingUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// 締準備処理
    pub async fn prepare_closing(
        &self,
        request: PrepareClosingRequest,
    ) -> AdapterResult<PrepareClosingResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }
}
