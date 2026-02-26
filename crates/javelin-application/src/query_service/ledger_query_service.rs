// LedgerQueryService - 元帳照会サービス

use serde::{Deserialize, Serialize};

use crate::error::ApplicationResult;

/// 元帳照会クエリ
#[derive(Debug, Clone)]
pub struct GetLedgerQuery {
    pub account_code: String,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// 試算表照会クエリ
#[derive(Debug, Clone)]
pub struct GetTrialBalanceQuery {
    pub period_year: u32,
    pub period_month: u8,
}

/// 元帳明細
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub transaction_date: String,
    pub entry_number: String,
    pub entry_id: String,
    pub description: String,
    pub debit_amount: f64,
    pub credit_amount: f64,
    pub balance: f64,
}

/// 元帳結果
#[derive(Debug, Clone)]
pub struct LedgerResult {
    pub account_code: String,
    pub account_name: String,
    pub opening_balance: f64,
    pub entries: Vec<LedgerEntry>,
    pub closing_balance: f64,
    pub total_debit: f64,
    pub total_credit: f64,
}

/// 試算表明細
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalanceEntry {
    pub account_code: String,
    pub account_name: String,
    pub opening_balance: f64,
    pub debit_amount: f64,
    pub credit_amount: f64,
    pub closing_balance: f64,
}

/// 試算表結果
#[derive(Debug, Clone)]
pub struct TrialBalanceResult {
    pub period_year: u32,
    pub period_month: u8,
    pub entries: Vec<TrialBalanceEntry>,
    pub total_debit: f64,
    pub total_credit: f64,
}

/// 元帳照会サービス（Application層トレイト）
#[allow(async_fn_in_trait)]
pub trait LedgerQueryService: Send + Sync {
    /// 元帳を取得
    async fn get_ledger(&self, query: GetLedgerQuery) -> ApplicationResult<LedgerResult>;

    /// 試算表を取得
    async fn get_trial_balance(
        &self,
        query: GetTrialBalanceQuery,
    ) -> ApplicationResult<TrialBalanceResult>;
}
