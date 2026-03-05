// 請求書印刷ユースケース

use std::sync::Arc;

use crate::{
    dtos::{
        request::PrintInvoiceRequest,
        response::{BankInfoDto, InvoiceItemDto, IssuerDto, PrintInvoiceResponse},
    },
    error::{ApplicationError, ApplicationResult},
    output_ports::InvoicePrinter,
    query_service::InvoiceQueryService,
};

/// 請求書印刷インタラクター
pub struct PrintInvoiceInteractor<Q, P>
where
    Q: InvoiceQueryService,
    P: InvoicePrinter,
{
    query_service: Arc<Q>,
    printer: Arc<P>,
}

impl<Q, P> PrintInvoiceInteractor<Q, P>
where
    Q: InvoiceQueryService,
    P: InvoicePrinter,
{
    pub fn new(query_service: Arc<Q>, printer: Arc<P>) -> Self {
        Self { query_service, printer }
    }

    /// 請求書を印刷してPDFを生成
    pub fn execute(&self, request: PrintInvoiceRequest) -> ApplicationResult<Vec<u8>> {
        // 1. クエリサービスから請求書データを取得
        let invoice = self
            .query_service
            .find_by_id(&request.invoice_id)?
            .ok_or_else(|| ApplicationError::NotFound("Invoice not found".to_string()))?;

        // 2. DTOに変換
        let response = PrintInvoiceResponse {
            receiver: invoice.recipient_name,
            invoice_no: invoice.invoice_number,
            date: invoice.issue_date,
            due_date: invoice.due_date,
            issue_type: invoice.issue_type,
            document_phase: invoice.document_phase,
            settlement: invoice.settlement_status,
            registration_no: invoice.registration_number.unwrap_or_default(),
            issuer: IssuerDto {
                name: invoice.issuer_name,
                department: invoice.issuer_department.unwrap_or_default(),
                address: invoice.issuer_address,
                email: invoice.issuer_email.unwrap_or_default(),
                tel: invoice.issuer_tel,
            },
            bank_info: BankInfoDto {
                bank: invoice.bank_name,
                branch: invoice.branch_name,
                account_type: invoice.account_type,
                number: invoice.account_number,
                name: invoice.account_holder,
            },
            items: invoice
                .line_items
                .into_iter()
                .map(|item| InvoiceItemDto {
                    name: item.description,
                    price: item.unit_price,
                    qty: item.quantity,
                    tax_rate: item.tax_rate,
                })
                .collect(),
        };

        // 3. 印刷ドライバでPDF生成
        self.printer.print_to_pdf(&response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query_service::{InvoiceLineItemQueryResult, InvoiceQueryResult};

    struct MockQueryService;

    impl InvoiceQueryService for MockQueryService {
        fn find_by_id(&self, _invoice_id: &str) -> ApplicationResult<Option<InvoiceQueryResult>> {
            Ok(Some(InvoiceQueryResult {
                invoice_id: "test-id".to_string(),
                invoice_number: "CI-2026-TEST-01".to_string(),
                recipient_name: "Test Customer".to_string(),
                recipient_address: None,
                recipient_contact: None,
                issuer_name: "Test Company".to_string(),
                issuer_department: Some("Sales".to_string()),
                issuer_address: "123 Test St".to_string(),
                issuer_tel: "03-1234-5678".to_string(),
                issuer_email: Some("test@example.com".to_string()),
                registration_number: Some("T1234567890123".to_string()),
                bank_name: "Test Bank".to_string(),
                branch_name: "Test Branch".to_string(),
                account_type: "普通".to_string(),
                account_number: "1234567".to_string(),
                account_holder: "Test Company".to_string(),
                issue_date: "2026-03-01".to_string(),
                due_date: "2026-03-31".to_string(),
                issue_type: "ORIGINAL".to_string(),
                document_phase: "COMMERCIAL".to_string(),
                settlement_status: "UNPAID".to_string(),
                line_items: vec![InvoiceLineItemQueryResult {
                    description: "Test Item".to_string(),
                    unit_price: 10000,
                    quantity: 1,
                    tax_rate: 10,
                }],
                notes: None,
            }))
        }

        fn find_by_number(
            &self,
            _invoice_number: &str,
        ) -> ApplicationResult<Option<InvoiceQueryResult>> {
            Ok(None)
        }
    }

    struct MockPrinter;

    impl InvoicePrinter for MockPrinter {
        fn print_to_pdf(&self, _invoice_data: &PrintInvoiceResponse) -> ApplicationResult<Vec<u8>> {
            Ok(vec![0x25, 0x50, 0x44, 0x46]) // "%PDF" magic number
        }
    }

    #[test]
    fn test_print_invoice_success() {
        let query_service = Arc::new(MockQueryService);
        let printer = Arc::new(MockPrinter);
        let interactor = PrintInvoiceInteractor::new(query_service, printer);

        let request = PrintInvoiceRequest::new("test-id".to_string());
        let result = interactor.execute(request);

        assert!(result.is_ok());
        let pdf = result.unwrap();
        assert!(!pdf.is_empty());
    }
}
