// 包括的財務諸表生成ユースケース（整合性検証付き）

use crate::{
    dtos::{
        GenerateComprehensiveFinancialStatementsRequest,
        GenerateComprehensiveFinancialStatementsResponse,
    },
    error::ApplicationResult,
};

pub trait GenerateComprehensiveFinancialStatementsUseCase: Send + Sync {
    fn execute(
        &self,
        request: GenerateComprehensiveFinancialStatementsRequest,
    ) -> impl std::future::Future<
        Output = ApplicationResult<GenerateComprehensiveFinancialStatementsResponse>,
    > + Send;
}
