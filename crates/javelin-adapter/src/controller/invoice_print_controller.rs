// 請求書印刷コントローラー

use std::sync::Arc;

use javelin_application::{
    dtos::request::PrintInvoiceRequest, input_ports::PrintInvoiceInputPort,
    interactor::PrintInvoiceInteractor,
};
use javelin_infrastructure::{
    read::invoice::MockInvoiceQueryService, shared::typst_invoice_printer::TypstInvoicePrinter,
};

use crate::presenter::InvoicePrintPresenter;

/// 請求書印刷コントローラー
pub struct InvoicePrintController {
    query_service: Arc<MockInvoiceQueryService>,
    printer: Arc<TypstInvoicePrinter>,
}

impl InvoicePrintController {
    pub fn new(
        query_service: Arc<MockInvoiceQueryService>,
        printer: Arc<TypstInvoicePrinter>,
    ) -> Self {
        Self { query_service, printer }
    }

    /// 請求書を印刷（プレゼンターを受け取って動的にインタラクターを作成）
    pub fn print_invoice_with_presenter(
        &self,
        invoice_id: String,
        presenter: Arc<InvoicePrintPresenter>,
    ) {
        let query_service = Arc::clone(&self.query_service);
        let printer = Arc::clone(&self.printer);

        tokio::spawn(async move {
            // プレゼンターをOutput Portとして注入してインタラクターを作成（静的ディスパッチ）
            let interactor = PrintInvoiceInteractor::new(query_service, printer, presenter);

            // リクエストを作成してInput Port経由で実行
            let request = PrintInvoiceRequest::new(invoice_id);
            let _ = interactor.execute(request);
        });
    }
}
