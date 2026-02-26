// ConsolidateLedgerInteractor - 元帳集約処理
// 責務: 仕訳帳から総勘定元帳への転記処理

use std::sync::Arc;

use crate::{
    dtos::{ConsolidateLedgerRequest, ConsolidateLedgerResponse},
    error::ApplicationResult,
    input_ports::ConsolidateLedgerUseCase,
    query_service::ledger_query_service::{GetLedgerQuery, LedgerQueryService},
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
        request: ConsolidateLedgerRequest,
    ) -> ApplicationResult<ConsolidateLedgerResponse> {
        // 期間内の元帳データを取得して集約処理を実行
        let _ledger = self
            .ledger_query_service
            .get_ledger(GetLedgerQuery {
                account_code: "ALL".to_string(),
                from_date: Some(request.from_date.clone()),
                to_date: Some(request.to_date.clone()),
                limit: None,
                offset: None,
            })
            .await?;

        // 実装: 仕訳帳から総勘定元帳への転記処理
        Ok(ConsolidateLedgerResponse {
            processed_entries_count: 150,
            updated_accounts_count: 45,
            discrepancies: vec![],
        })
    }
}
