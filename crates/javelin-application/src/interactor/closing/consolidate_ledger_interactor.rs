// ConsolidateLedgerInteractor - 元帳統合処理
// 責務: 補助元帳→総勘定元帳の統合

use std::sync::Arc;

use crate::{
    dtos::{ConsolidateLedgerRequest, ConsolidateLedgerResponse},
    error::ApplicationResult,
    input_ports::ConsolidateLedgerUseCase,
    query_service::ledger_query_service::LedgerQueryService,
};

pub struct ConsolidateLedgerInteractor<Q>
where
    Q: LedgerQueryService,
{
    ledger_query_service: Arc<Q>,
}

impl<Q> ConsolidateLedgerInteractor<Q>
where
    Q: LedgerQueryService,
{
    pub fn new(ledger_query_service: Arc<Q>) -> Self {
        Self { ledger_query_service }
    }
}

impl<Q> ConsolidateLedgerUseCase for ConsolidateLedgerInteractor<Q>
where
    Q: LedgerQueryService,
{
    async fn execute(
        &self,
        _request: ConsolidateLedgerRequest,
    ) -> ApplicationResult<ConsolidateLedgerResponse> {
        // TODO: 元帳統合処理の実装
        // 1. 補助元帳の集計
        // 2. 総勘定元帳への統合
        // 3. 差異の検証

        Ok(ConsolidateLedgerResponse {
            processed_entries_count: 0,
            updated_accounts_count: 0,
            discrepancies: vec![],
        })
    }
}
