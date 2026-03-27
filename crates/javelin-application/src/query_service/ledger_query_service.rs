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

/// 元帳明細 - レスポンスDTO
/// すべての金額をString（BigDecimal由来の10進数文字列）で保持
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub transaction_date: String,
    pub entry_number: String,
    pub entry_id: String,
    pub description: String,
    pub debit_amount: String,  // 10進数文字列形式
    pub credit_amount: String, // 10進数文字列形式
    pub balance: String,       // 10進数文字列形式
}

/// 元帳結果 - レスポンスDTO
/// すべての金額をString（BigDecimal由来の10進数文字列）で保持
#[derive(Debug, Clone)]
pub struct LedgerResult {
    pub account_code: String,
    pub account_name: String,
    pub opening_balance: String,                     // 10進数文字列形式
    pub provisional_opening_balance: Option<String>, // 10進数文字列形式
    pub entries: Vec<LedgerEntry>,
    pub closing_balance: String, // 10進数文字列形式
    pub total_debit: String,     // 10進数文字列形式
    pub total_credit: String,    // 10進数文字列形式
}

/// 試算表明細 - レスポンスDTO
/// すべての金額をString（BigDecimal由来の10進数文字列）で保持
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalanceEntry {
    pub account_code: String,
    pub account_name: String,
    pub opening_balance: String, // 10進数文字列形式
    pub debit_amount: String,    // 10進数文字列形式
    pub credit_amount: String,   // 10進数文字列形式
    pub closing_balance: String, // 10進数文字列形式
}

/// 試算表結果 - レスポンスDTO
/// すべての金額をString（BigDecimal由来の10進数文字列）で保持
#[derive(Debug, Clone)]
pub struct TrialBalanceResult {
    pub period_year: u32,
    pub period_month: u8,
    pub entries: Vec<TrialBalanceEntry>,
    pub total_debit: String,  // 10進数文字列形式
    pub total_credit: String, // 10進数文字列形式
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
