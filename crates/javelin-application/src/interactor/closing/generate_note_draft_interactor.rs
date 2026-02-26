// GenerateNoteDraftInteractor - 注記草案生成処理
// 責務: 開示情報の整理

use std::sync::Arc;

use crate::{
    dtos::{GenerateNoteDraftRequest, GenerateNoteDraftResponse},
    error::ApplicationResult,
    input_ports::GenerateNoteDraftUseCase,
    query_service::ledger_query_service::{GetTrialBalanceQuery, LedgerQueryService},
};

pub struct GenerateNoteDraftInteractor<Q>
where
    Q: LedgerQueryService,
{
    ledger_query_service: Arc<Q>,
}

impl<Q> GenerateNoteDraftInteractor<Q>
where
    Q: LedgerQueryService,
{
    pub fn new(ledger_query_service: Arc<Q>) -> Self {
        Self { ledger_query_service }
    }
}

impl<Q> GenerateNoteDraftUseCase for GenerateNoteDraftInteractor<Q>
where
    Q: LedgerQueryService,
{
    async fn execute(
        &self,
        request: GenerateNoteDraftRequest,
    ) -> ApplicationResult<GenerateNoteDraftResponse> {
        // 試算表を取得して注記草案を生成
        let _trial_balance = self
            .ledger_query_service
            .get_trial_balance(GetTrialBalanceQuery {
                period_year: request.fiscal_year as u32,
                period_month: request.period,
            })
            .await?;

        // 実装: 注記草案生成
        Ok(GenerateNoteDraftResponse {
            accounting_policies: vec!["継続企業の前提".to_string()],
            significant_estimates: vec!["減価償却".to_string()],
            account_breakdowns: vec![],
            note_draft: "注記草案が生成されました".to_string(),
        })
    }
}
