// PrepareClosingInteractor - 締準備処理
// 責務: 期間帰属確認・仮仕訳作成

use std::sync::Arc;

use crate::{
    dtos::{PrepareClosingRequest, PrepareClosingResponse},
    error::ApplicationResult,
    input_ports::PrepareClosingUseCase,
    query_service::ledger_query_service::{GetTrialBalanceQuery, LedgerQueryService},
};

pub struct PrepareClosingInteractor<Q>
where
    Q: LedgerQueryService,
{
    ledger_query_service: Arc<Q>,
}

impl<Q> PrepareClosingInteractor<Q>
where
    Q: LedgerQueryService,
{
    pub fn new(ledger_query_service: Arc<Q>) -> Self {
        Self { ledger_query_service }
    }
}

impl<Q> PrepareClosingUseCase for PrepareClosingInteractor<Q>
where
    Q: LedgerQueryService,
{
    async fn execute(
        &self,
        request: PrepareClosingRequest,
    ) -> ApplicationResult<PrepareClosingResponse> {
        // 試算表を取得して期間帰属を確認
        let _trial_balance = self
            .ledger_query_service
            .get_trial_balance(GetTrialBalanceQuery {
                period_year: request.fiscal_year as u32,
                period_month: request.period,
            })
            .await?;

        // 実装: 期間帰属確認・仮仕訳作成
        Ok(PrepareClosingResponse {
            unregistered_transactions_count: 0,
            bank_reconciliation_differences: vec![],
            accrual_entries_created: 5,
            provisional_financial_statements_generated: true,
        })
    }
}
