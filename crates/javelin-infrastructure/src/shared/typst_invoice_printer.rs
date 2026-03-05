// Typst請求書印刷ドライバ
//
// 請求書テンプレートからTypstコードを生成する。
// 実際のPDF生成は外部のtypstコマンドまたは将来的なtypstライブラリ統合で行う。

use std::path::PathBuf;

use javelin_application::{
    dtos::response::PrintInvoiceResponse, error::ApplicationResult, output_ports::InvoicePrinter,
};

/// Typst請求書印刷ドライバ
pub struct TypstInvoicePrinter {
    template_path: PathBuf,
}

impl TypstInvoicePrinter {
    pub fn new(template_path: PathBuf) -> Self {
        Self { template_path }
    }

    /// テンプレートファイルを読み込む
    fn load_template(&self) -> ApplicationResult<String> {
        std::fs::read_to_string(&self.template_path).map_err(|e| {
            javelin_application::error::ApplicationError::Unknown(format!(
                "Failed to load template: {}",
                e
            ))
        })
    }

    /// 請求書データをTypstコードに変換
    fn generate_typst_code(&self, invoice: &PrintInvoiceResponse) -> String {
        let items_code = invoice
            .items
            .iter()
            .map(|item| {
                format!(
                    "    (name: \"{}\", price: {}, qty: {}, tax-rate: {})",
                    item.name.replace('"', "\\\""),
                    item.price,
                    item.qty,
                    item.tax_rate
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            r#"#show: invoice.with(
  receiver: "{}",
  invoice-no: "{}",
  date: "{}",
  due-date: "{}",
  issue-type: "{}",
  document-phase: "{}",
  settlement: "{}",
  registration-no: "{}",
  issuer: (
    name: "{}",
    department: "{}",
    address: "{}",
    email: "{}",
    tel: "{}",
  ),
  bank-info: (
    bank: "{}",
    branch: "{}",
    type: "{}",
    number: "{}",
    name: "{}",
  ),
  items: (
{}
  ),
)
"#,
            invoice.receiver.replace('"', "\\\""),
            invoice.invoice_no.replace('"', "\\\""),
            invoice.date.replace('"', "\\\""),
            invoice.due_date.replace('"', "\\\""),
            invoice.issue_type,
            invoice.document_phase,
            invoice.settlement,
            invoice.registration_no.replace('"', "\\\""),
            invoice.issuer.name.replace('"', "\\\""),
            invoice.issuer.department.replace('"', "\\\""),
            invoice.issuer.address.replace('"', "\\\""),
            invoice.issuer.email.replace('"', "\\\""),
            invoice.issuer.tel.replace('"', "\\\""),
            invoice.bank_info.bank.replace('"', "\\\""),
            invoice.bank_info.branch.replace('"', "\\\""),
            invoice.bank_info.account_type.replace('"', "\\\""),
            invoice.bank_info.number.replace('"', "\\\""),
            invoice.bank_info.name.replace('"', "\\\""),
            items_code
        )
    }
}

impl InvoicePrinter for TypstInvoicePrinter {
    fn print_to_pdf(&self, invoice_data: &PrintInvoiceResponse) -> ApplicationResult<Vec<u8>> {
        // 1. テンプレートを読み込む
        let template = self.load_template()?;

        // 2. データを埋め込んだTypstコードを生成
        let typst_code = self.generate_typst_code(invoice_data);

        // 3. テンプレートとデータを結合
        let full_code = format!("{}\n{}", template, typst_code);

        // TODO: 実際のPDF生成
        // 現在は生成されたTypstコードをバイト列として返す（開発用）
        // 将来的には以下のいずれかで実装：
        // - typstコマンドを外部プロセスとして実行
        // - typst-rsなどのライブラリを使用
        // - typst WebAssembly版を使用
        Ok(full_code.into_bytes())
    }
}

impl Default for TypstInvoicePrinter {
    fn default() -> Self {
        Self::new(PathBuf::from("crates/javelin-infrastructure/templates/invoice.typ"))
    }
}

#[cfg(test)]
mod tests {
    use javelin_application::dtos::response::{BankInfoDto, InvoiceItemDto, IssuerDto};

    use super::*;

    fn create_test_invoice() -> PrintInvoiceResponse {
        PrintInvoiceResponse {
            receiver: "Test Customer".to_string(),
            invoice_no: "CI-2026-TEST-01".to_string(),
            date: "March 5, 2026".to_string(),
            due_date: "April 30, 2026".to_string(),
            issue_type: "ORIGINAL".to_string(),
            document_phase: "COMMERCIAL".to_string(),
            settlement: "UNPAID".to_string(),
            registration_no: "T1234567890123".to_string(),
            issuer: IssuerDto {
                name: "Test Company".to_string(),
                department: "Sales Dept".to_string(),
                address: "123 Test St, Tokyo".to_string(),
                email: "test@example.com".to_string(),
                tel: "03-1234-5678".to_string(),
            },
            bank_info: BankInfoDto {
                bank: "Test Bank".to_string(),
                branch: "Test Branch".to_string(),
                account_type: "普通".to_string(),
                number: "1234567".to_string(),
                name: "Test Company".to_string(),
            },
            items: vec![
                InvoiceItemDto {
                    name: "Product A".to_string(),
                    price: 10000,
                    qty: 2,
                    tax_rate: 10,
                },
                InvoiceItemDto { name: "Product B".to_string(), price: 5000, qty: 1, tax_rate: 8 },
            ],
        }
    }

    #[test]
    fn test_generate_typst_code() {
        let printer = TypstInvoicePrinter::new(PathBuf::from("dummy.typ"));
        let invoice = create_test_invoice();
        let code = printer.generate_typst_code(&invoice);

        assert!(code.contains("Test Customer"));
        assert!(code.contains("CI-2026-TEST-01"));
        assert!(code.contains("Product A"));
        assert!(code.contains("Product B"));
        assert!(code.contains("price: 10000"));
        assert!(code.contains("qty: 2"));
        assert!(code.contains("tax-rate: 10"));
    }
}
