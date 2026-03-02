// ConsolidateLedgerController - 元帳集約コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{ConsolidateLedgerRequest, ConsolidateLedgerResponse},
    input_ports::ConsolidateLedgerUseCase,
};

use crate::error::AdapterResult;

/// 元帳集約コントローラ
pub struct ConsolidateLedgerController<U>
where
    U: ConsolidateLedgerUseCase,
{
    use_case: Arc<U>,
}

impl<U> ConsolidateLedgerController<U>
where
    U: ConsolidateLedgerUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// 元帳集約処理
    pub async fn consolidate_ledger(
        &self,
        request: ConsolidateLedgerRequest,
    ) -> AdapterResult<ConsolidateLedgerResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }
}
