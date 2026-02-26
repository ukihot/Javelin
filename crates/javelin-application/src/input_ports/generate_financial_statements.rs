// 4.9 財務諸表生成処理（月次）
// 目的: 評価調整後の残高を基に制度開示要件を満たす財務諸表を作成

use crate::{
    dtos::{GenerateFinancialStatementsRequest, GenerateFinancialStatementsResponse},
    error::ApplicationResult,
};

/// 財務諸表生成ユースケース
#[allow(async_fn_in_trait)]
pub trait GenerateFinancialStatementsUseCase: Send + Sync {
    async fn execute(
        &self,
        request: GenerateFinancialStatementsRequest,
    ) -> ApplicationResult<GenerateFinancialStatementsResponse>;
}
