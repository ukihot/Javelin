// 請求書印刷リクエストDTO

/// 請求書印刷リクエスト
#[derive(Debug, Clone)]
pub struct PrintInvoiceRequest {
    /// 請求書ID
    pub invoice_id: String,
}

impl PrintInvoiceRequest {
    pub fn new(invoice_id: String) -> Self {
        Self { invoice_id }
    }
}
