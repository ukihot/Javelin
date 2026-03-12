// AdjustAccountsInteractor - 勘定補正処理
// 責務: 仮勘定整理・区分修正

use std::sync::Arc;

use crate::{
    dtos::{AdjustAccountsRequest, AdjustAccountsResponse},
    error::ApplicationResult,
    input_ports::AdjustAccountsUseCase,
    query_service::ledger_query_service::{GetTrialBalanceQuery, LedgerQueryService},
};

pub struct AdjustAccountsInteractor<Q>
where
    Q: LedgerQueryService,
{
    ledger_query_service: Arc<Q>,
}

impl<Q> AdjustAccountsInteractor<Q>
where
    Q: LedgerQueryService,
{
    pub fn new(ledger_query_service: Arc<Q>) -> Self {
        Self { ledger_query_service }
    }
}

impl<Q> AdjustAccountsUseCase for AdjustAccountsInteractor<Q>
where
    Q: LedgerQueryService,
{
    async fn execute(
        &self,
        request: AdjustAccountsRequest,
    ) -> ApplicationResult<AdjustAccountsResponse> {
        // 試算表を取得して補正対象を特定
        let _trial_balance = self
            .ledger_query_service
            .get_trial_balance(GetTrialBalanceQuery {
                period_year: request.fiscal_year as u32,
                period_month: request.period,
            })
            .await?;

        // TODO: 実際の勘定補正処理を実装
        // 1. 仮勘定を特定
        // 2. 補正仕訳を作成
        // 3. 区分修正を実行

        Ok(AdjustAccountsResponse {
            adjustment_entries_created: 0,
            reclassified_accounts: vec![],
            tax_effect_adjustments: vec![],
        })
    }
}
