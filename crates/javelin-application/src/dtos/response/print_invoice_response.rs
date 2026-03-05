// 請求書印刷レスポンスDTO
// invoice.typが想定するデータ構造に対応

/// 請求書印刷レスポンス
#[derive(Debug, Clone)]
pub struct PrintInvoiceResponse {
    /// 請求先名
    pub receiver: String,
    /// 請求書番号
    pub invoice_no: String,
    /// 発行日
    pub date: String,
    /// 支払期限
    pub due_date: String,
    /// 発行区分（ORIGINAL, DUPLICATE, REVISED）
    pub issue_type: String,
    /// 文書フェーズ（COMMERCIAL, PROFORMA）
    pub document_phase: String,
    /// 決済ステータス（UNPAID, PAID, OVERDUE）
    pub settlement: String,
    /// インボイス登録番号
    pub registration_no: String,
    /// 発行者情報
    pub issuer: IssuerDto,
    /// 銀行情報
    pub bank_info: BankInfoDto,
    /// 明細行
    pub items: Vec<InvoiceItemDto>,
}

/// 発行者情報DTO
#[derive(Debug, Clone)]
pub struct IssuerDto {
    pub name: String,
    pub department: String,
    pub address: String,
    pub email: String,
    pub tel: String,
}

/// 銀行情報DTO
#[derive(Debug, Clone)]
pub struct BankInfoDto {
    pub bank: String,
    pub branch: String,
    pub account_type: String,
    pub number: String,
    pub name: String,
}

/// 請求明細DTO
#[derive(Debug, Clone)]
pub struct InvoiceItemDto {
    pub name: String,
    pub price: i64,
    pub qty: u32,
    pub tax_rate: u8,
}
