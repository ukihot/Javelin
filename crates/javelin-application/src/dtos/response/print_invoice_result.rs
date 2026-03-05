// 請求書印刷結果DTO

/// 請求書印刷結果
#[derive(Debug, Clone)]
pub struct PrintInvoiceResult {
    /// 生成されたPDFデータ
    pub pdf_data: Vec<u8>,
    /// 保存されたファイルパス
    pub file_path: String,
}

impl PrintInvoiceResult {
    pub fn new(pdf_data: Vec<u8>, file_path: String) -> Self {
        Self { pdf_data, file_path }
    }
}
