// GenerateFinancialStatementsInteractor - 財務諸表生成処理
// 責務: 貸借対照表・損益計算書の生成

use std::sync::Arc;

use crate::{
    dtos::{GenerateFinancialStatementsRequest, GenerateFinancialStatementsResponse},
    error::ApplicationResult,
    input_ports::GenerateFinancialStatementsUseCase,
    query_service::ledger_query_service::LedgerQueryService,
};

pub struct GenerateFinancialStatementsInteractor<Q>
where
    Q: LedgerQueryService,
{
    ledger_query_service: Arc<Q>,
}

impl<Q> GenerateFinancialStatementsInteractor<Q>
where
    Q: LedgerQueryService,
{
    pub fn new(ledger_query_service: Arc<Q>) -> Self {
        Self { ledger_query_service }
    }
}

impl<Q> GenerateFinancialStatementsUseCase for GenerateFinancialStatementsInteractor<Q>
where
    Q: LedgerQueryService,
{
    async fn execute(
        &self,
        _request: GenerateFinancialStatementsRequest,
    ) -> ApplicationResult<GenerateFinancialStatementsResponse> {
        // TODO: 財務諸表生成処理の実装
        // 1. 試算表から貸借対照表を生成
        // 2. 試算表から損益計算書を生成
        // 3. キャッシュフロー計算書を生成

        Ok(GenerateFinancialStatementsResponse {
            statement_of_financial_position:
                crate::dtos::response::closing_process::StatementOfFinancialPositionDto {
                    current_assets: 0.0,
                    current_assets_currency: "JPY".to_string(),
                    non_current_assets: 0.0,
                    non_current_assets_currency: "JPY".to_string(),
                    current_liabilities: 0.0,
                    current_liabilities_currency: "JPY".to_string(),
                    non_current_liabilities: 0.0,
                    non_current_liabilities_currency: "JPY".to_string(),
                    equity: 0.0,
                    equity_currency: "JPY".to_string(),
                },
            statement_of_profit_or_loss:
                crate::dtos::response::closing_process::StatementOfProfitOrLossDto {
                    revenue: 0.0,
                    revenue_currency: "JPY".to_string(),
                    cost_of_sales: 0.0,
                    cost_of_sales_currency: "JPY".to_string(),
                    gross_profit: 0.0,
                    gross_profit_currency: "JPY".to_string(),
                    operating_expenses: 0.0,
                    operating_expenses_currency: "JPY".to_string(),
                    operating_profit: 0.0,
                    operating_profit_currency: "JPY".to_string(),
                    net_profit: 0.0,
                    net_profit_currency: "JPY".to_string(),
                },
            statement_of_changes_in_equity:
                crate::dtos::response::closing_process::StatementOfChangesInEquityDto {
                    opening_balance: 0.0,
                    opening_balance_currency: "JPY".to_string(),
                    net_profit: 0.0,
                    net_profit_currency: "JPY".to_string(),
                    dividends: 0.0,
                    dividends_currency: "JPY".to_string(),
                    closing_balance: 0.0,
                    closing_balance_currency: "JPY".to_string(),
                },
            statement_of_cash_flows:
                crate::dtos::response::closing_process::StatementOfCashFlowsDto {
                    operating_activities: 0.0,
                    operating_activities_currency: "JPY".to_string(),
                    investing_activities: 0.0,
                    investing_activities_currency: "JPY".to_string(),
                    financing_activities: 0.0,
                    financing_activities_currency: "JPY".to_string(),
                    net_change_in_cash: 0.0,
                    net_change_in_cash_currency: "JPY".to_string(),
                },
            financial_indicators: crate::dtos::response::closing_process::FinancialIndicatorsDto {
                roe: 0.0,
                roa: 0.0,
                current_ratio: 0.0,
                debt_to_equity_ratio: 0.0,
            },
            cross_check_passed: true,
        })
    }
}
