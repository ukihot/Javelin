// GenerateComprehensiveFinancialStatementsInteractor - 包括財務諸表生成処理
// 責務: 包括利益計算書・株主資本等変動計算書の生成

use std::sync::Arc;

use crate::{
    dtos::{
        GenerateComprehensiveFinancialStatementsRequest,
        GenerateComprehensiveFinancialStatementsResponse,
    },
    error::ApplicationResult,
    input_ports::GenerateComprehensiveFinancialStatementsUseCase,
    query_service::ledger_query_service::LedgerQueryService,
};

pub struct GenerateComprehensiveFinancialStatementsInteractor<Q>
where
    Q: LedgerQueryService,
{
    ledger_query_service: Arc<Q>,
}

impl<Q> GenerateComprehensiveFinancialStatementsInteractor<Q>
where
    Q: LedgerQueryService,
{
    pub fn new(ledger_query_service: Arc<Q>) -> Self {
        Self { ledger_query_service }
    }
}

impl<Q> GenerateComprehensiveFinancialStatementsUseCase
    for GenerateComprehensiveFinancialStatementsInteractor<Q>
where
    Q: LedgerQueryService,
{
    async fn execute(
        &self,
        _request: GenerateComprehensiveFinancialStatementsRequest,
    ) -> ApplicationResult<GenerateComprehensiveFinancialStatementsResponse> {
        // TODO: 包括財務諸表生成処理の実装
        // 1. 包括利益計算書の生成
        // 2. 株主資本等変動計算書の生成

        Ok(GenerateComprehensiveFinancialStatementsResponse {
            statements: vec![],
            consistency_check: None,
            cross_check: None,
            generated_at: chrono::Utc::now(),
            approval_status:
                crate::dtos::response::comprehensive_financial_statements::ApprovalStatus::Draft,
        })
    }
}
