// 請求書印刷ドライバの抽象トレイト

use crate::{dtos::response::PrintInvoiceResponse, error::ApplicationResult};

/// 請求書印刷ドライバ
///
/// インフラ層で実装される印刷機能の抽象インターフェース。
/// Typstなどの具体的な印刷エンジンに依存しない。
pub trait InvoicePrinter: Send + Sync {
    /// 請求書をPDFとして印刷
    ///
    /// # Arguments
    /// * `invoice_data` - 請求書データ
    ///
    /// # Returns
    /// PDFバイナリデータ
    fn print_to_pdf(&self, invoice_data: &PrintInvoiceResponse) -> ApplicationResult<Vec<u8>>;
}
