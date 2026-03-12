// ApplyIfrsValuationInteractor - IFRS評価処理
// 責務: ECL計算・減損判定・引当金計算・棚卸資産評価

use std::sync::Arc;

use crate::{
    dtos::{ApplyIfrsValuationRequest, ApplyIfrsValuationResponse},
    error::ApplicationResult,
    input_ports::ApplyIfrsValuationUseCase,
    query_service::ledger_query_service::LedgerQueryService,
};

pub struct ApplyIfrsValuationInteractor<Q>
where
    Q: LedgerQueryService,
{
    ledger_query_service: Arc<Q>,
}

impl<Q> ApplyIfrsValuationInteractor<Q>
where
    Q: LedgerQueryService,
{
    pub fn new(ledger_query_service: Arc<Q>) -> Self {
        Self { ledger_query_service }
    }
}

impl<Q> ApplyIfrsValuationUseCase for ApplyIfrsValuationInteractor<Q>
where
    Q: LedgerQueryService,
{
    async fn execute(
        &self,
        _request: ApplyIfrsValuationRequest,
    ) -> ApplicationResult<ApplyIfrsValuationResponse> {
        // TODO: IFRS評価処理の実装
        // 1. ECL計算
        // 2. 減損判定
        // 3. 引当金計算
        // 4. 棚卸資産評価

        Ok(ApplyIfrsValuationResponse {
            expected_credit_loss: 0.0,
            expected_credit_loss_currency: "JPY".to_string(),
            contingent_liabilities: vec![],
            inventory_write_downs: vec![],
            impairment_losses: vec![],
            fair_value_adjustments: vec![],
            lease_measurements: vec![],
        })
    }
}
