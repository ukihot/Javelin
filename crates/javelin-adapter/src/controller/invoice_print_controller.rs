// 請求書印刷コントローラー

use std::{path::PathBuf, sync::Arc};

use javelin_application::{
    dtos::request::PrintInvoiceRequest, input_ports::PrintInvoiceInputPort,
    output_ports::InvoicePrintOutputPort,
};

/// 請求書印刷コントローラー
pub struct InvoicePrintController<I, O>
where
    I: PrintInvoiceInputPort + 'static,
    O: InvoicePrintOutputPort + 'static,
{
    input_port: Arc<I>,
    output_port: Arc<O>,
}

impl<I, O> InvoicePrintController<I, O>
where
    I: PrintInvoiceInputPort + 'static,
    O: InvoicePrintOutputPort + 'static,
{
    pub fn new(input_port: Arc<I>, output_port: Arc<O>) -> Self {
        Self { input_port, output_port }
    }

    /// 請求書を印刷してPDFを保存
    pub fn print_invoice(&self, invoice_id: String, output_dir: PathBuf) {
        let input_port = Arc::clone(&self.input_port);
        let output_port = Arc::clone(&self.output_port);

        tokio::spawn(async move {
            // リクエストを作成
            let request = PrintInvoiceRequest::new(invoice_id.clone());

            // ユースケースを実行（内部でoutput_portに通知される）
            match input_port.execute(request) {
                Ok(pdf_data) => {
                    // PDFをファイルに保存
                    let file_name = format!("invoice_{}.pdf", invoice_id);
                    let file_path = output_dir.join(&file_name);

                    match std::fs::write(&file_path, pdf_data) {
                        Ok(_) => {
                            // ファイル保存成功を通知
                            output_port
                                .notify_print_success(file_path.to_string_lossy().to_string())
                                .await;
                        }
                        Err(e) => {
                            // ファイル保存失敗を通知
                            output_port
                                .notify_print_error(format!("ファイル保存失敗: {}", e))
                                .await;
                        }
                    }
                }
                Err(_e) => {
                    // エラーは既にinteractor内でoutput_portに通知済み
                }
            }
        });
    }

    /// モックデータで請求書を印刷（開発用）
    pub fn print_mock_invoice(&self, output_dir: PathBuf) {
        self.print_invoice("mock-invoice-001".to_string(), output_dir);
    }
}
