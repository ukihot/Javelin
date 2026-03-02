// AdjustAccountsController - 勘定補正コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{AdjustAccountsRequest, AdjustAccountsResponse},
    input_ports::AdjustAccountsUseCase,
};

use crate::error::AdapterResult;

/// 勘定補正コントローラ
pub struct AdjustAccountsController<U>
where
    U: AdjustAccountsUseCase,
{
    use_case: Arc<U>,
}

impl<U> AdjustAccountsController<U>
where
    U: AdjustAccountsUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// 勘定補正処理
    pub async fn adjust_accounts(
        &self,
        request: AdjustAccountsRequest,
    ) -> AdapterResult<AdjustAccountsResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }
}
