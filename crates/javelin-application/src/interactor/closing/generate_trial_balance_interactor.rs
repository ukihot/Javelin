// GenerateTrialBalanceInteractor - 試算表生成処理
// 責務: 残高検証・異常値抽出

use std::sync::Arc;

use crate::{
    dtos::{AccountBalanceDto, GenerateTrialBalanceRequest, GenerateTrialBalanceResponse},
    error::ApplicationResult,
    input_ports::GenerateTrialBalanceUseCase,
    query_service::ledger_query_service::{GetTrialBalanceQuery, LedgerQueryService},
};

pub struct GenerateTrialBalanceInteractor<Q>
where
    Q: LedgerQueryService,
{
    ledger_query_service: Arc<Q>,
}

impl<Q> GenerateTrialBalanceInteractor<Q>
where
    Q: LedgerQueryService,
{
    pub fn new(ledger_query_service: Arc<Q>) -> Self {
        Self { ledger_query_service }
    }
}

impl<Q> GenerateTrialBalanceUseCase for GenerateTrialBalanceInteractor<Q>
where
    Q: LedgerQueryService,
{
    async fn execute(
        &self,
        request: GenerateTrialBalanceRequest,
    ) -> ApplicationResult<GenerateTrialBalanceResponse> {
        // 試算表を取得
        let trial_balance = self
            .ledger_query_service
            .get_trial_balance(GetTrialBalanceQuery {
                period_year: request.fiscal_year as u32,
                period_month: request.period,
            })
            .await?;

        // 試算表エントリをDTOに変換
        let account_balances: Vec<AccountBalanceDto> = trial_balance
            .entries
            .iter()
            .map(|entry| AccountBalanceDto {
                account_code: entry.account_code.clone(),
                debit_balance: if entry.closing_balance >= 0.0 {
                    entry.closing_balance
                } else {
                    0.0
                },
                debit_balance_currency: "JPY".to_string(),
                credit_balance: if entry.closing_balance < 0.0 {
                    -entry.closing_balance
                } else {
                    0.0
                },
                credit_balance_currency: "JPY".to_string(),
                net_balance: entry.closing_balance,
                net_balance_currency: "JPY".to_string(),
            })
            .collect();

        Ok(GenerateTrialBalanceResponse {
            total_debit: trial_balance.total_debit,
            total_debit_currency: "JPY".to_string(),
            total_credit: trial_balance.total_credit,
            total_credit_currency: "JPY".to_string(),
            is_balanced: (trial_balance.total_debit - trial_balance.total_credit).abs() < 0.01,
            account_balances,
            temporary_account_balances: vec![],
            foreign_exchange_differences: vec![],
        })
    }
}
