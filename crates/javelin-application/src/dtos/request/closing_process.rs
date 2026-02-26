// 決算処理関連 - Request DTOs
// すべてのプロパティはプリミティブ型

/// 元帳集約処理
#[derive(Debug, Clone)]
pub struct ConsolidateLedgerRequest {
    pub fiscal_year: i32,
    pub period: u8,
    pub from_date: String, // YYYY-MM-DD format
    pub to_date: String,   // YYYY-MM-DD format
}

/// 締準備処理
#[derive(Debug, Clone)]
pub struct PrepareClosingRequest {
    pub fiscal_year: i32,
    pub period: u8,
}

/// 締日固定処理
#[derive(Debug, Clone)]
pub struct LockClosingPeriodRequest {
    pub fiscal_year: i32,
    pub period: u8,
    pub locked_by: String,
}

/// 試算表生成処理
#[derive(Debug, Clone)]
pub struct GenerateTrialBalanceRequest {
    pub fiscal_year: i32,
    pub period: u8,
}

/// 注記草案生成処理
#[derive(Debug, Clone)]
pub struct GenerateNoteDraftRequest {
    pub fiscal_year: i32,
    pub period: u8,
}

/// 勘定補正処理
#[derive(Debug, Clone)]
pub struct AdjustAccountsRequest {
    pub fiscal_year: i32,
    pub period: u8,
}

/// IFRS評価処理
#[derive(Debug, Clone)]
pub struct ApplyIfrsValuationRequest {
    pub fiscal_year: i32,
    pub period: u8,
    pub user_id: String,
}

/// 財務諸表生成処理
#[derive(Debug, Clone)]
pub struct GenerateFinancialStatementsRequest {
    pub fiscal_year: i32,
    pub period: u8,
}
