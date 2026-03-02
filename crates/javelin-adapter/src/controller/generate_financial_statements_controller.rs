// GenerateFinancialStatementsController - 財務諸表生成コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{GenerateFinancialStatementsRequest, GenerateFinancialStatementsResponse},
    input_ports::GenerateFinancialStatementsUseCase,
};

use crate::error::AdapterResult;

/// 財務諸表生成コントローラ
pub struct GenerateFinancialStatementsController<U>
where
    U: GenerateFinancialStatementsUseCase,
{
    use_case: Arc<U>,
}

impl<U> GenerateFinancialStatementsController<U>
where
    U: GenerateFinancialStatementsUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// 財務諸表生成処理
    pub async fn generate_financial_statements(
        &self,
        request: GenerateFinancialStatementsRequest,
    ) -> AdapterResult<GenerateFinancialStatementsResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }
}
