// ApplyIfrsValuationController - IFRS評価コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{ApplyIfrsValuationRequest, ApplyIfrsValuationResponse},
    input_ports::ApplyIfrsValuationUseCase,
};

use crate::error::AdapterResult;

/// IFRS評価コントローラ
pub struct ApplyIfrsValuationController<U>
where
    U: ApplyIfrsValuationUseCase,
{
    use_case: Arc<U>,
}

impl<U> ApplyIfrsValuationController<U>
where
    U: ApplyIfrsValuationUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// IFRS評価処理
    pub async fn apply_ifrs_valuation(
        &self,
        request: ApplyIfrsValuationRequest,
    ) -> AdapterResult<ApplyIfrsValuationResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }
}
