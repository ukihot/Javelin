// モック請求書クエリサービス（開発用）

use javelin_application::{
    error::ApplicationResult,
    query_service::{InvoiceLineItemQueryResult, InvoiceQueryResult, InvoiceQueryService},
};

/// モック請求書クエリサービス
pub struct MockInvoiceQueryService;

impl MockInvoiceQueryService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MockInvoiceQueryService {
    fn default() -> Self {
        Self::new()
    }
}

impl InvoiceQueryService for MockInvoiceQueryService {
    fn find_by_id(&self, invoice_id: &str) -> ApplicationResult<Option<InvoiceQueryResult>> {
        // モックデータを返す
        if invoice_id == "mock-invoice-001" {
            Ok(Some(InvoiceQueryResult {
                invoice_id: invoice_id.to_string(),
                invoice_number: "CI-2026-MOCK-001".to_string(),
                recipient_name: "株式会社サンプル商事".to_string(),
                recipient_address: Some("東京都千代田区丸の内1-1-1".to_string()),
                recipient_contact: Some("山田太郎".to_string()),
                issuer_name: "Javelin会計システム株式会社".to_string(),
                issuer_department: Some("営業部".to_string()),
                issuer_address: "東京都港区六本木1-2-3 Javelinビル".to_string(),
                issuer_tel: "03-1234-5678".to_string(),
                issuer_email: Some("sales@javelin-accounting.example.com".to_string()),
                registration_number: Some("T1234567890123".to_string()),
                bank_name: "みずほ銀行".to_string(),
                branch_name: "東京営業部".to_string(),
                account_type: "普通".to_string(),
                account_number: "1234567".to_string(),
                account_holder: "ジャベリンカイケイシステム（カ".to_string(),
                issue_date: "2026年3月5日".to_string(),
                due_date: "2026年4月30日".to_string(),
                issue_type: "ORIGINAL".to_string(),
                document_phase: "COMMERCIAL".to_string(),
                settlement_status: "UNPAID".to_string(),
                line_items: vec![
                    InvoiceLineItemQueryResult {
                        description: "会計システム月額利用料（2026年3月分）".to_string(),
                        unit_price: 50000,
                        quantity: 1,
                        tax_rate: 10,
                    },
                    InvoiceLineItemQueryResult {
                        description: "追加ユーザーライセンス（5名分）".to_string(),
                        unit_price: 5000,
                        quantity: 5,
                        tax_rate: 10,
                    },
                    InvoiceLineItemQueryResult {
                        description: "データバックアップサービス".to_string(),
                        unit_price: 10000,
                        quantity: 1,
                        tax_rate: 10,
                    },
                ],
                notes: Some(
                    "お支払いは指定口座へお振込みください。振込手数料はお客様負担となります。"
                        .to_string(),
                ),
            }))
        } else {
            Ok(None)
        }
    }

    fn find_by_number(
        &self,
        _invoice_number: &str,
    ) -> ApplicationResult<Option<InvoiceQueryResult>> {
        // モックでは未実装
        Ok(None)
    }
}
