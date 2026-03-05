// 請求書クエリサービス
// CQRS: 読み取り専用のクエリサービス

use crate::error::ApplicationResult;

/// 請求書クエリ結果
#[derive(Debug, Clone)]
pub struct InvoiceQueryResult {
    pub invoice_id: String,
    pub invoice_number: String,
    pub recipient_name: String,
    pub recipient_address: Option<String>,
    pub recipient_contact: Option<String>,
    pub issuer_name: String,
    pub issuer_department: Option<String>,
    pub issuer_address: String,
    pub issuer_tel: String,
    pub issuer_email: Option<String>,
    pub registration_number: Option<String>,
    pub bank_name: String,
    pub branch_name: String,
    pub account_type: String,
    pub account_number: String,
    pub account_holder: String,
    pub issue_date: String,
    pub due_date: String,
    pub issue_type: String,
    pub document_phase: String,
    pub settlement_status: String,
    pub line_items: Vec<InvoiceLineItemQueryResult>,
    pub notes: Option<String>,
}

/// 請求明細クエリ結果
#[derive(Debug, Clone)]
pub struct InvoiceLineItemQueryResult {
    pub description: String,
    pub unit_price: i64,
    pub quantity: u32,
    pub tax_rate: u8,
}

/// 請求書クエリサービス
///
/// CQRS原則に従い、読み取り専用のクエリを提供する。
/// ProjectionDBから請求書データを取得する。
pub trait InvoiceQueryService: Send + Sync {
    /// 請求書IDで請求書を取得
    fn find_by_id(&self, invoice_id: &str) -> ApplicationResult<Option<InvoiceQueryResult>>;

    /// 請求書番号で請求書を取得
    fn find_by_number(&self, invoice_number: &str)
    -> ApplicationResult<Option<InvoiceQueryResult>>;
}
