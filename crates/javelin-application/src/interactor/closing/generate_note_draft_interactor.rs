// GenerateNoteDraftInteractor - 注記草案生成処理
// 責務: 財務諸表注記の草案生成

use std::sync::Arc;

use crate::{
    dtos::{GenerateNoteDraftRequest, GenerateNoteDraftResponse},
    error::ApplicationResult,
    input_ports::GenerateNoteDraftUseCase,
    query_service::ledger_query_service::LedgerQueryService,
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
        _request: GenerateNoteDraftRequest,
    ) -> ApplicationResult<GenerateNoteDraftResponse> {
        // TODO: 注記草案生成処理の実装
        // 1. 重要な会計方針の注記
        // 2. 重要な後発事象の注記
        // 3. その他の注記

        Ok(GenerateNoteDraftResponse {
            accounting_policies: vec![],
            significant_estimates: vec![],
            account_breakdowns: vec![],
            note_draft: String::new(),
        })
    }
}
