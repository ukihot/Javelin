// Typst請求書印刷ドライバ
//
// 請求書テンプレートからTypstコードを生成し、PDFにコンパイルする。

use std::path::{Path, PathBuf};

use javelin_application::{
    dtos::response::PrintInvoiceResponse, error::ApplicationResult, output_ports::InvoicePrinter,
};
use typst::{
    Library, LibraryExt, World,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_pdf::PdfOptions;

/// Typst請求書印刷ドライバ
pub struct TypstInvoicePrinter {
    template_path: PathBuf,
    fonts_dir: PathBuf,
}

impl TypstInvoicePrinter {
    pub fn new(template_path: &Path) -> Self {
        let fonts_dir = template_path
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("fonts"))
            .unwrap_or_else(|| PathBuf::from("crates/javelin-infrastructure/fonts"));

        Self { template_path: template_path.to_path_buf(), fonts_dir }
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

    /// フォントを読み込む
    fn load_fonts(&self) -> Vec<Font> {
        let mut fonts = Vec::new();

        // システムフォントとカスタムフォントを読み込む
        if self.fonts_dir.exists()
            && let Ok(entries) = std::fs::read_dir(&self.fonts_dir)
        {
            for entry in entries.flatten() {
                let path = entry.path();
                if (path.extension().and_then(|s| s.to_str()) == Some("ttf")
                    || path.extension().and_then(|s| s.to_str()) == Some("otf"))
                    && let Ok(data) = std::fs::read(&path)
                {
                    // Vec<u8>をBytesに変換してフォントを作成
                    let bytes = Bytes::new(data);
                    if let Some(font) = Font::new(bytes, 0) {
                        fonts.push(font);
                    }
                }
            }
        }

        fonts
    }

    /// TypstコードをPDFにコンパイル
    fn compile_to_pdf(&self, typst_code: String) -> ApplicationResult<Vec<u8>> {
        // フォントを読み込む
        let fonts = self.load_fonts();
        let font_book = FontBook::from_fonts(&fonts);

        // Worldを作成
        let world = SimpleWorld::new(typst_code, fonts, font_book);

        // コンパイル
        let result = typst::compile(&world);
        let document = match result.output {
            Ok(doc) => doc,
            Err(errors) => {
                let error_msg =
                    errors.iter().map(|e| format!("{:?}", e)).collect::<Vec<_>>().join("\n");
                return Err(javelin_application::error::ApplicationError::Unknown(format!(
                    "Typst compilation failed: {}",
                    error_msg
                )));
            }
        };

        // PDFに変換
        let pdf_options = PdfOptions::default();
        let pdf_result = typst_pdf::pdf(&document, &pdf_options);

        match pdf_result {
            Ok(pdf_data) => Ok(pdf_data),
            Err(errors) => {
                let error_msg =
                    errors.iter().map(|e| format!("{:?}", e)).collect::<Vec<_>>().join("\n");
                Err(javelin_application::error::ApplicationError::Unknown(format!(
                    "PDF generation failed: {}",
                    error_msg
                )))
            }
        }
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

        // 4. PDFにコンパイル
        self.compile_to_pdf(full_code)
    }
}

impl Default for TypstInvoicePrinter {
    fn default() -> Self {
        Self::new(Path::new("crates/javelin-infrastructure/templates/invoice.typ"))
    }
}

/// シンプルなTypst World実装
struct SimpleWorld {
    source: Source,
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
}

impl SimpleWorld {
    fn new(content: String, fonts: Vec<Font>, book: FontBook) -> Self {
        let source = Source::detached(content);

        Self {
            source,
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(book),
            fonts,
        }
    }
}

impl World for SimpleWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.source.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(id.vpath().as_rootless_path().to_path_buf()))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(id.vpath().as_rootless_path().to_path_buf()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        Some(Datetime::from_ymd(2026, 3, 5).unwrap())
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
        let printer = TypstInvoicePrinter::new(Path::new("dummy.typ"));
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
