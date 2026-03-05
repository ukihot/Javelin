// 請求書印刷Input Port

use crate::{dtos::request::PrintInvoiceRequest, error::ApplicationResult};

/// 請求書印刷Input Port
pub trait PrintInvoiceInputPort: Send + Sync {
    /// 請求書を印刷してPDFを生成
    fn execute(&self, request: PrintInvoiceRequest) -> ApplicationResult<Vec<u8>>;
}
