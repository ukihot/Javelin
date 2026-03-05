// Billing Services - 請求集約のドメインサービス
//
// CQRS原則に従い、ドメインサービスはDBアクセスを行わない。
// 2つの請求書を比較して同一性を判定するなど、純粋なビジネスロジックのみを提供する。

use super::entities::Invoice;

/// 請求書ドメインサービス
pub struct InvoiceDomainService;

impl InvoiceDomainService {
    /// 2つの請求書が同一かどうかを判定
    ///
    /// エンティティの同一性はIDで判定される。
    /// この関数は、ドメインロジックとして明示的に同一性チェックが必要な場合に使用する。
    pub fn are_same_invoice(invoice1: &Invoice, invoice2: &Invoice) -> bool {
        invoice1 == invoice2
    }

    /// 2つの請求書の内容が等価かどうかを判定
    ///
    /// IDは異なるが、請求書番号や内容が同じかどうかをチェックする。
    /// 重複請求書の検出などに使用する。
    pub fn have_same_content(invoice1: &Invoice, invoice2: &Invoice) -> bool {
        invoice1.invoice_number() == invoice2.invoice_number()
            && invoice1.recipient() == invoice2.recipient()
            && invoice1.issue_date() == invoice2.issue_date()
            && invoice1.grand_total() == invoice2.grand_total()
    }

    /// 請求書が訂正可能かどうかを判定
    ///
    /// 支払済みの請求書は訂正できない。
    pub fn can_revise(invoice: &Invoice) -> bool {
        !matches!(invoice.settlement_status(), super::values::SettlementStatus::Paid)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;
    use crate::{
        billing::values::{
            BankInfo, BillingRecipient, DocumentPhase, InvoiceLineItem, InvoiceNumber, IssueType,
            IssuerInfo, TaxRate,
        },
        common::Amount,
    };

    fn create_test_invoice() -> Invoice {
        let issuer = IssuerInfo::new(
            "Test Company".to_string(),
            Some("Sales".to_string()),
            "123 Test St".to_string(),
            "03-1234-5678".to_string(),
            None,
            None,
        )
        .unwrap();

        let bank_info = BankInfo::new(
            "Test Bank".to_string(),
            "Test Branch".to_string(),
            "普通".to_string(),
            "1234567".to_string(),
            "Test Company".to_string(),
        )
        .unwrap();

        let line_items = vec![
            InvoiceLineItem::new(
                "Test Item".to_string(),
                Amount::from_i64(10000),
                1,
                TaxRate::new(10).unwrap(),
            )
            .unwrap(),
        ];

        Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-01".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            issuer,
            bank_info,
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            line_items,
            None,
        )
        .unwrap()
    }

    #[test]
    fn test_are_same_invoice() {
        let invoice1 = create_test_invoice();
        let invoice2 = create_test_invoice();

        // 異なるインスタンス（異なるID）なので同一ではない
        assert!(!InvoiceDomainService::are_same_invoice(&invoice1, &invoice2));

        // 同じインスタンスは同一
        assert!(InvoiceDomainService::are_same_invoice(&invoice1, &invoice1));
    }

    #[test]
    fn test_have_same_content() {
        let invoice1 = create_test_invoice();

        // 同じ内容で別のインスタンスを作成
        let issuer = IssuerInfo::new(
            "Test Company".to_string(),
            Some("Sales".to_string()),
            "123 Test St".to_string(),
            "03-1234-5678".to_string(),
            None,
            None,
        )
        .unwrap();

        let bank_info = BankInfo::new(
            "Test Bank".to_string(),
            "Test Branch".to_string(),
            "普通".to_string(),
            "1234567".to_string(),
            "Test Company".to_string(),
        )
        .unwrap();

        let line_items = vec![
            InvoiceLineItem::new(
                "Test Item".to_string(),
                Amount::from_i64(10000),
                1,
                TaxRate::new(10).unwrap(),
            )
            .unwrap(),
        ];

        let invoice2 = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-01".to_string()).unwrap(), // 同じ請求書番号
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            issuer,
            bank_info,
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            line_items,
            None,
        )
        .unwrap();

        // 内容が同じ
        assert!(InvoiceDomainService::have_same_content(&invoice1, &invoice2));
    }

    #[test]
    fn test_have_different_content() {
        let invoice1 = create_test_invoice();

        let issuer = IssuerInfo::new(
            "Test Company".to_string(),
            Some("Sales".to_string()),
            "123 Test St".to_string(),
            "03-1234-5678".to_string(),
            None,
            None,
        )
        .unwrap();

        let bank_info = BankInfo::new(
            "Test Bank".to_string(),
            "Test Branch".to_string(),
            "普通".to_string(),
            "1234567".to_string(),
            "Test Company".to_string(),
        )
        .unwrap();

        let line_items = vec![
            InvoiceLineItem::new(
                "Test Item".to_string(),
                Amount::from_i64(10000),
                1,
                TaxRate::new(10).unwrap(),
            )
            .unwrap(),
        ];

        let invoice2 = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-02".to_string()).unwrap(), // 異なる請求書番号
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            issuer,
            bank_info,
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            line_items,
            None,
        )
        .unwrap();

        // 内容が異なる
        assert!(!InvoiceDomainService::have_same_content(&invoice1, &invoice2));
    }

    #[test]
    fn test_can_revise_unpaid() {
        let invoice = create_test_invoice();
        // 未払いの請求書は訂正可能
        assert!(InvoiceDomainService::can_revise(&invoice));
    }

    #[test]
    fn test_cannot_revise_paid() {
        let mut invoice = create_test_invoice();
        invoice.mark_as_paid().unwrap();
        // 支払済みの請求書は訂正不可
        assert!(!InvoiceDomainService::can_revise(&invoice));
    }
}
