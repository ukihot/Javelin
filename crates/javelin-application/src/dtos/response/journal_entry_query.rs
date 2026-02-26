// 仕訳照会レスポンスDTO

/// 仕訳一覧アイテム
#[derive(Debug, Clone)]
pub struct JournalEntryListItem {
    pub entry_id: String,
    pub entry_number: Option<String>,
    pub status: String,
    pub transaction_date: String,
    pub voucher_number: String,
    pub total_debit: f64,
    pub total_credit: f64,
    pub created_by: String,
    pub created_at: String,
}

/// 仕訳一覧結果
#[derive(Debug, Clone)]
pub struct JournalEntryListResult {
    pub items: Vec<JournalEntryListItem>,
    pub total_count: u32,
}

/// 仕訳明細
#[derive(Debug, Clone)]
pub struct JournalEntryLineDetail {
    pub line_number: u32,
    pub side: String,
    pub account_code: String,
    pub account_name: String,
    pub sub_account_code: Option<String>,
    pub department_code: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub tax_type: String,
    pub tax_amount: f64,
}

/// 仕訳詳細レスポンス
#[derive(Debug, Clone)]
pub struct JournalEntryDetail {
    pub entry_id: String,
    pub entry_number: Option<String>,
    pub status: String,
    pub transaction_date: String,
    pub voucher_number: String,
    pub lines: Vec<JournalEntryLineDetail>,
    pub created_by: String,
    pub created_at: String,
    pub updated_by: Option<String>,
    pub updated_at: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
}
