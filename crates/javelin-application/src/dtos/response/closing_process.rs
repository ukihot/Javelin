// 決算処理関連 - Response DTOs
// すべてのプロパティはプリミティブ型

/// 元帳集約処理レスポンス
#[derive(Debug, Clone)]
pub struct ConsolidateLedgerResponse {
    pub processed_entries_count: usize,
    pub updated_accounts_count: usize,
    pub discrepancies: Vec<LedgerDiscrepancyDto>,
}

#[derive(Debug, Clone)]
pub struct LedgerDiscrepancyDto {
    pub account_code: String,
    pub general_ledger_balance: f64,
    pub general_ledger_currency: String,
    pub subsidiary_ledger_balance: f64,
    pub subsidiary_ledger_currency: String,
    pub difference: f64,
    pub difference_currency: String,
}

/// 締準備処理レスポンス
#[derive(Debug, Clone)]
pub struct PrepareClosingResponse {
    pub unregistered_transactions_count: usize,
    pub bank_reconciliation_differences: Vec<BankReconciliationDifferenceDto>,
    pub accrual_entries_created: usize,
    pub provisional_financial_statements_generated: bool,
}

#[derive(Debug, Clone)]
pub struct BankReconciliationDifferenceDto {
    pub bank_account: String,
    pub bank_balance: f64,
    pub bank_balance_currency: String,
    pub cash_log_balance: f64,
    pub cash_log_balance_currency: String,
    pub difference: f64,
    pub difference_currency: String,
}

/// 締日固定処理レスポンス
#[derive(Debug, Clone)]
pub struct LockClosingPeriodResponse {
    pub locked_entries_count: usize,
    pub locked_at: String, // ISO 8601 format
    pub audit_log_id: String,
}

/// 試算表生成処理レスポンス
#[derive(Debug, Clone)]
pub struct GenerateTrialBalanceResponse {
    pub total_debit: f64,
    pub total_debit_currency: String,
    pub total_credit: f64,
    pub total_credit_currency: String,
    pub is_balanced: bool,
    pub account_balances: Vec<AccountBalanceDto>,
    pub temporary_account_balances: Vec<AccountBalanceDto>,
    pub foreign_exchange_differences: Vec<ForeignExchangeDifferenceDto>,
}

#[derive(Debug, Clone)]
pub struct AccountBalanceDto {
    pub account_code: String,
    pub debit_balance: f64,
    pub debit_balance_currency: String,
    pub credit_balance: f64,
    pub credit_balance_currency: String,
    pub net_balance: f64,
    pub net_balance_currency: String,
}

#[derive(Debug, Clone)]
pub struct ForeignExchangeDifferenceDto {
    pub account_code: String,
    pub original_amount: f64,
    pub original_currency: String,
    pub exchange_rate: f64,
    pub converted_amount: f64,
    pub converted_currency: String,
    pub difference: f64,
    pub difference_currency: String,
}

/// 注記草案生成処理レスポンス
#[derive(Debug, Clone)]
pub struct GenerateNoteDraftResponse {
    pub accounting_policies: Vec<String>,
    pub significant_estimates: Vec<String>,
    pub account_breakdowns: Vec<AccountBreakdownDto>,
    pub note_draft: String,
}

#[derive(Debug, Clone)]
pub struct AccountBreakdownDto {
    pub account_code: String,
    pub components: Vec<String>,
}

/// 勘定補正処理レスポンス
#[derive(Debug, Clone)]
pub struct AdjustAccountsResponse {
    pub adjustment_entries_created: usize,
    pub reclassified_accounts: Vec<AccountReclassificationDto>,
    pub tax_effect_adjustments: Vec<TaxEffectAdjustmentDto>,
}

#[derive(Debug, Clone)]
pub struct AccountReclassificationDto {
    pub from_account: String,
    pub to_account: String,
    pub amount: f64,
    pub currency: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct TaxEffectAdjustmentDto {
    pub temporary_difference: f64,
    pub temporary_difference_currency: String,
    pub tax_rate: f64,
    pub deferred_tax_amount: f64,
    pub deferred_tax_currency: String,
}

/// IFRS評価処理レスポンス
#[derive(Debug, Clone)]
pub struct ApplyIfrsValuationResponse {
    pub expected_credit_loss: f64,
    pub expected_credit_loss_currency: String,
    pub contingent_liabilities: Vec<ContingentLiabilityDto>,
    pub inventory_write_downs: Vec<InventoryWriteDownDto>,
    pub impairment_losses: Vec<ImpairmentLossDto>,
    pub fair_value_adjustments: Vec<FairValueAdjustmentDto>,
    pub lease_measurements: Vec<LeaseMeasurementDto>,
}

#[derive(Debug, Clone)]
pub struct ContingentLiabilityDto {
    pub description: String,
    pub probability: f64,
    pub estimated_amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone)]
pub struct InventoryWriteDownDto {
    pub item: String,
    pub cost: f64,
    pub cost_currency: String,
    pub net_realizable_value: f64,
    pub net_realizable_value_currency: String,
    pub write_down_amount: f64,
    pub write_down_currency: String,
}

#[derive(Debug, Clone)]
pub struct ImpairmentLossDto {
    pub asset: String,
    pub carrying_amount: f64,
    pub carrying_amount_currency: String,
    pub recoverable_amount: f64,
    pub recoverable_amount_currency: String,
    pub impairment_loss: f64,
    pub impairment_loss_currency: String,
}

#[derive(Debug, Clone)]
pub struct FairValueAdjustmentDto {
    pub financial_asset: String,
    pub book_value: f64,
    pub book_value_currency: String,
    pub fair_value: f64,
    pub fair_value_currency: String,
    pub adjustment: f64,
    pub adjustment_currency: String,
}

#[derive(Debug, Clone)]
pub struct LeaseMeasurementDto {
    pub lease_contract: String,
    pub right_of_use_asset: f64,
    pub right_of_use_asset_currency: String,
    pub lease_liability: f64,
    pub lease_liability_currency: String,
}

/// 財務諸表生成処理レスポンス
#[derive(Debug, Clone)]
pub struct GenerateFinancialStatementsResponse {
    pub statement_of_financial_position: StatementOfFinancialPositionDto,
    pub statement_of_profit_or_loss: StatementOfProfitOrLossDto,
    pub statement_of_changes_in_equity: StatementOfChangesInEquityDto,
    pub statement_of_cash_flows: StatementOfCashFlowsDto,
    pub financial_indicators: FinancialIndicatorsDto,
    pub cross_check_passed: bool,
}

#[derive(Debug, Clone)]
pub struct StatementOfFinancialPositionDto {
    pub current_assets: f64,
    pub current_assets_currency: String,
    pub non_current_assets: f64,
    pub non_current_assets_currency: String,
    pub current_liabilities: f64,
    pub current_liabilities_currency: String,
    pub non_current_liabilities: f64,
    pub non_current_liabilities_currency: String,
    pub equity: f64,
    pub equity_currency: String,
}

#[derive(Debug, Clone)]
pub struct StatementOfProfitOrLossDto {
    pub revenue: f64,
    pub revenue_currency: String,
    pub cost_of_sales: f64,
    pub cost_of_sales_currency: String,
    pub gross_profit: f64,
    pub gross_profit_currency: String,
    pub operating_expenses: f64,
    pub operating_expenses_currency: String,
    pub operating_profit: f64,
    pub operating_profit_currency: String,
    pub net_profit: f64,
    pub net_profit_currency: String,
}

#[derive(Debug, Clone)]
pub struct StatementOfChangesInEquityDto {
    pub opening_balance: f64,
    pub opening_balance_currency: String,
    pub net_profit: f64,
    pub net_profit_currency: String,
    pub dividends: f64,
    pub dividends_currency: String,
    pub closing_balance: f64,
    pub closing_balance_currency: String,
}

#[derive(Debug, Clone)]
pub struct StatementOfCashFlowsDto {
    pub operating_activities: f64,
    pub operating_activities_currency: String,
    pub investing_activities: f64,
    pub investing_activities_currency: String,
    pub financing_activities: f64,
    pub financing_activities_currency: String,
    pub net_change_in_cash: f64,
    pub net_change_in_cash_currency: String,
}

#[derive(Debug, Clone)]
pub struct FinancialIndicatorsDto {
    pub roe: f64, // Return on Equity
    pub roa: f64, // Return on Assets
    pub current_ratio: f64,
    pub debt_to_equity_ratio: f64,
}
