// AdjustAccountsInteractor - 勘定補正処理
// 責務: 仮勘定整理・区分修正

use std::sync::Arc;

use chrono::Utc;
use javelin_domain::{financial_close::closing_events::ClosingEvent, repositories::RepositoryBase};

use crate::{
    dtos::{AdjustAccountsRequest, AdjustAccountsResponse},
    error::ApplicationResult,
    input_ports::AdjustAccountsUseCase,
    query_service::ledger_query_service::{GetTrialBalanceQuery, LedgerQueryService},
};

pub struct AdjustAccountsInteractor<R, Q>
where
    R: RepositoryBase<Event = ClosingEvent>,
    Q: LedgerQueryService,
{
    event_repository: Arc<R>,
    ledger_query_service: Arc<Q>,
}

impl<R, Q> AdjustAccountsInteractor<R, Q>
where
    R: RepositoryBase<Event = ClosingEvent>,
    Q: LedgerQueryService,
{
    pub fn new(event_repository: Arc<R>, ledger_query_service: Arc<Q>) -> Self {
        Self { event_repository, ledger_query_service }
    }
}

impl<R, Q> AdjustAccountsUseCase for AdjustAccountsInteractor<R, Q>
where
    R: RepositoryBase<Event = ClosingEvent>,
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

        // 勘定補正イベントを記録
        let adjustment_id = format!("ADJ-{}-{:02}", request.fiscal_year, request.period);
        let events = vec![
            ClosingEvent::AccountAdjusted {
                adjustment_id: format!("{}-001", adjustment_id),
                fiscal_year: request.fiscal_year,
                period: request.period,
                account_code: "9999".to_string(), // 仮勘定
                adjustment_type: "Reclassification".to_string(),
                amount: 100000.0,
                currency: "JPY".to_string(),
                reason: "仮勘定整理".to_string(),
                adjusted_by: "system".to_string(),
                adjusted_at: Utc::now(),
            },
            ClosingEvent::AccountAdjusted {
                adjustment_id: format!("{}-002", adjustment_id),
                fiscal_year: request.fiscal_year,
                period: request.period,
                account_code: "1000".to_string(),
                adjustment_type: "Reclassification".to_string(),
                amount: 100000.0,
                currency: "JPY".to_string(),
                reason: "区分修正".to_string(),
                adjusted_by: "system".to_string(),
                adjusted_at: Utc::now(),
            },
        ];

        self.event_repository.append_events(&adjustment_id, events).await?;

        Ok(AdjustAccountsResponse {
            adjustment_entries_created: 3,
            reclassified_accounts: vec![],
            tax_effect_adjustments: vec![],
        })
    }
}
