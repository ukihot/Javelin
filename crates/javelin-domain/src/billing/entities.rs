// Billing Entities - 請求集約のエンティティ

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::values::{
    BankInfo, BillingRecipient, DocumentPhase, InvoiceId, InvoiceLineItem, InvoiceNumber,
    IssueType, IssuerInfo, SettlementStatus, TaxRate,
};
use crate::{
    common::Amount,
    entity::Entity,
    error::{DomainError, DomainResult},
};

/// 請求書エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    id: InvoiceId,
    invoice_number: InvoiceNumber,
    recipient: BillingRecipient,
    issuer: IssuerInfo,
    bank_info: BankInfo,
    issue_date: NaiveDate,
    due_date: NaiveDate,
    issue_type: IssueType,
    document_phase: DocumentPhase,
    settlement_status: SettlementStatus,
    line_items: Vec<InvoiceLineItem>,
    notes: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Invoice {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        invoice_number: InvoiceNumber,
        recipient: BillingRecipient,
        issuer: IssuerInfo,
        bank_info: BankInfo,
        issue_date: NaiveDate,
        due_date: NaiveDate,
        issue_type: IssueType,
        document_phase: DocumentPhase,
        line_items: Vec<InvoiceLineItem>,
        notes: Option<String>,
    ) -> DomainResult<Self> {
        if line_items.is_empty() {
            return Err(DomainError::ValidationError("請求明細は1件以上必要です".to_string()));
        }

        if due_date < issue_date {
            return Err(DomainError::ValidationError(
                "支払期限は発行日以降である必要があります".to_string(),
            ));
        }

        let now = chrono::Utc::now();
        let settlement_status =
            SettlementStatus::determine(due_date, false, chrono::Utc::now().date_naive());

        Ok(Self {
            id: InvoiceId::new(),
            invoice_number,
            recipient,
            issuer,
            bank_info,
            issue_date,
            due_date,
            issue_type,
            document_phase,
            settlement_status,
            line_items,
            notes,
            created_at: now,
            updated_at: now,
        })
    }

    /// 請求書を再構築（リポジトリからの復元用）
    #[allow(clippy::too_many_arguments)]
    pub fn reconstruct(
        id: InvoiceId,
        invoice_number: InvoiceNumber,
        recipient: BillingRecipient,
        issuer: IssuerInfo,
        bank_info: BankInfo,
        issue_date: NaiveDate,
        due_date: NaiveDate,
        issue_type: IssueType,
        document_phase: DocumentPhase,
        settlement_status: SettlementStatus,
        line_items: Vec<InvoiceLineItem>,
        notes: Option<String>,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            invoice_number,
            recipient,
            issuer,
            bank_info,
            issue_date,
            due_date,
            issue_type,
            document_phase,
            settlement_status,
            line_items,
            notes,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn invoice_number(&self) -> &InvoiceNumber {
        &self.invoice_number
    }

    pub fn recipient(&self) -> &BillingRecipient {
        &self.recipient
    }

    pub fn issuer(&self) -> &IssuerInfo {
        &self.issuer
    }

    pub fn bank_info(&self) -> &BankInfo {
        &self.bank_info
    }

    pub fn issue_date(&self) -> NaiveDate {
        self.issue_date
    }

    pub fn due_date(&self) -> NaiveDate {
        self.due_date
    }

    pub fn issue_type(&self) -> IssueType {
        self.issue_type
    }

    pub fn document_phase(&self) -> DocumentPhase {
        self.document_phase
    }

    pub fn settlement_status(&self) -> SettlementStatus {
        self.settlement_status
    }

    pub fn line_items(&self) -> &[InvoiceLineItem] {
        &self.line_items
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }

    /// 小計合計（税抜）
    pub fn subtotal(&self) -> Amount {
        self.line_items
            .iter()
            .map(|item| item.subtotal())
            .fold(Amount::zero(), |acc, amount| &acc + &amount)
    }

    /// 消費税額合計
    pub fn total_tax(&self) -> Amount {
        self.line_items
            .iter()
            .map(|item| item.tax_amount())
            .fold(Amount::zero(), |acc, amount| &acc + &amount)
    }

    /// 総合計（税込）
    pub fn grand_total(&self) -> Amount {
        &self.subtotal() + &self.total_tax()
    }

    /// 税率別集計
    pub fn tax_summary(&self) -> Vec<TaxSummary> {
        let mut summary_map: std::collections::HashMap<u8, (Amount, Amount)> =
            std::collections::HashMap::new();

        for item in &self.line_items {
            let rate = item.tax_rate().rate();
            let entry = summary_map.entry(rate).or_insert((Amount::zero(), Amount::zero()));
            entry.0 = &entry.0 + &item.subtotal();
            entry.1 = &entry.1 + &item.tax_amount();
        }

        let mut summaries: Vec<TaxSummary> = summary_map
            .into_iter()
            .map(|(rate, (subtotal, tax))| TaxSummary {
                tax_rate: TaxRate::new(rate).unwrap(),
                subtotal,
                tax_amount: tax,
            })
            .collect();

        summaries.sort_by_key(|s| s.tax_rate.rate());
        summaries
    }

    /// 支払済みにマーク
    pub fn mark_as_paid(&mut self) -> DomainResult<()> {
        if self.settlement_status == SettlementStatus::Paid {
            return Err(DomainError::ValidationError("既に支払済みです".to_string()));
        }

        self.settlement_status = SettlementStatus::Paid;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    /// 決済ステータスを更新（期限超過判定など）
    pub fn update_settlement_status(&mut self, current_date: NaiveDate) {
        let is_paid = self.settlement_status == SettlementStatus::Paid;
        let new_status = SettlementStatus::determine(self.due_date, is_paid, current_date);

        if new_status != self.settlement_status {
            self.settlement_status = new_status;
            self.updated_at = chrono::Utc::now();
        }
    }

    /// 訂正版として複製
    pub fn create_revised_copy(&self, new_invoice_number: InvoiceNumber) -> DomainResult<Self> {
        Self::new(
            new_invoice_number,
            self.recipient.clone(),
            self.issuer.clone(),
            self.bank_info.clone(),
            self.issue_date,
            self.due_date,
            IssueType::Revised,
            self.document_phase,
            self.line_items.clone(),
            self.notes.clone(),
        )
    }
}

impl Entity for Invoice {
    type Id = InvoiceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl PartialEq for Invoice {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Invoice {}

/// 税率別集計
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxSummary {
    pub tax_rate: TaxRate,
    pub subtotal: Amount,
    pub tax_amount: Amount,
}

impl TaxSummary {
    pub fn total(&self) -> Amount {
        &self.subtotal + &self.tax_amount
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    fn create_test_issuer() -> IssuerInfo {
        IssuerInfo::new(
            "Test Company".to_string(),
            Some("Sales Dept".to_string()),
            "123 Test St".to_string(),
            "03-1234-5678".to_string(),
            Some("test@example.com".to_string()),
            Some("T1234567890123".to_string()),
        )
        .unwrap()
    }

    fn create_test_bank_info() -> BankInfo {
        BankInfo::new(
            "Test Bank".to_string(),
            "Test Branch".to_string(),
            "普通".to_string(),
            "1234567".to_string(),
            "Test Company".to_string(),
        )
        .unwrap()
    }

    fn create_test_line_items() -> Vec<InvoiceLineItem> {
        vec![
            InvoiceLineItem::new(
                "Product A".to_string(),
                Amount::from_i64(10000),
                2,
                TaxRate::new(10).unwrap(),
            )
            .unwrap(),
            InvoiceLineItem::new(
                "Product B".to_string(),
                Amount::from_i64(5000),
                1,
                TaxRate::new(8).unwrap(),
            )
            .unwrap(),
        ]
    }

    #[test]
    fn test_invoice_creation() {
        let invoice = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-01".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            create_test_issuer(),
            create_test_bank_info(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            create_test_line_items(),
            None,
        );

        assert!(invoice.is_ok());
        let invoice = invoice.unwrap();
        assert_eq!(invoice.invoice_number().value(), "CI-2026-TEST-01");
        assert_eq!(invoice.line_items().len(), 2);
    }

    #[test]
    fn test_invoice_empty_line_items() {
        let invoice = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-02".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            create_test_issuer(),
            create_test_bank_info(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            vec![], // 空の明細
            None,
        );

        assert!(invoice.is_err());
    }

    #[test]
    fn test_invoice_invalid_due_date() {
        let invoice = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-03".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            create_test_issuer(),
            create_test_bank_info(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(), // 発行日より前
            IssueType::Original,
            DocumentPhase::Commercial,
            create_test_line_items(),
            None,
        );

        assert!(invoice.is_err());
    }

    #[test]
    fn test_invoice_calculations() {
        let invoice = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-04".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            create_test_issuer(),
            create_test_bank_info(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            create_test_line_items(),
            None,
        )
        .unwrap();

        // 小計: (10000 * 2) + (5000 * 1) = 25000
        assert_eq!(invoice.subtotal().to_i64(), Some(25000));

        // 税額: (20000 * 10%) + (5000 * 8%) = 2000 + 400 = 2400
        assert_eq!(invoice.total_tax().to_i64(), Some(2400));

        // 総合計: 25000 + 2400 = 27400
        assert_eq!(invoice.grand_total().to_i64(), Some(27400));
    }

    #[test]
    fn test_invoice_tax_summary() {
        let invoice = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-05".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            create_test_issuer(),
            create_test_bank_info(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            create_test_line_items(),
            None,
        )
        .unwrap();

        let summary = invoice.tax_summary();
        assert_eq!(summary.len(), 2); // 8%と10%の2種類

        // 税率でソートされているはず
        assert_eq!(summary[0].tax_rate.rate(), 8);
        assert_eq!(summary[1].tax_rate.rate(), 10);
    }

    #[test]
    fn test_invoice_equality() {
        let invoice1 = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-06".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            create_test_issuer(),
            create_test_bank_info(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            create_test_line_items(),
            None,
        )
        .unwrap();

        let invoice2 = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-07".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            create_test_issuer(),
            create_test_bank_info(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            create_test_line_items(),
            None,
        )
        .unwrap();

        // エンティティはIDで比較される（IDが異なるので等しくない）
        assert_ne!(invoice1, invoice2);

        // 同じインスタンスは等しい
        assert_eq!(invoice1, invoice1);
    }

    #[test]
    fn test_invoice_mark_as_paid() {
        let mut invoice = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-08".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            create_test_issuer(),
            create_test_bank_info(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            create_test_line_items(),
            None,
        )
        .unwrap();

        assert_eq!(invoice.settlement_status(), SettlementStatus::Unpaid);

        let result = invoice.mark_as_paid();
        assert!(result.is_ok());
        assert_eq!(invoice.settlement_status(), SettlementStatus::Paid);

        // 既に支払済みの場合はエラー
        let result = invoice.mark_as_paid();
        assert!(result.is_err());
    }

    #[test]
    fn test_invoice_update_settlement_status() {
        let mut invoice = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-09".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            create_test_issuer(),
            create_test_bank_info(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            create_test_line_items(),
            None,
        )
        .unwrap();

        // 期限前
        invoice.update_settlement_status(NaiveDate::from_ymd_opt(2026, 3, 15).unwrap());
        assert_eq!(invoice.settlement_status(), SettlementStatus::Unpaid);

        // 期限超過
        invoice.update_settlement_status(NaiveDate::from_ymd_opt(2026, 4, 5).unwrap());
        assert_eq!(invoice.settlement_status(), SettlementStatus::Overdue);
    }

    #[test]
    fn test_invoice_create_revised_copy() {
        let original = Invoice::new(
            InvoiceNumber::new("CI-2026-TEST-10".to_string()).unwrap(),
            BillingRecipient::new("Customer A".to_string()).unwrap(),
            create_test_issuer(),
            create_test_bank_info(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            IssueType::Original,
            DocumentPhase::Commercial,
            create_test_line_items(),
            None,
        )
        .unwrap();

        let revised = original
            .create_revised_copy(InvoiceNumber::new("CI-2026-TEST-10-R1".to_string()).unwrap());

        assert!(revised.is_ok());
        let revised = revised.unwrap();
        assert_eq!(revised.invoice_number().value(), "CI-2026-TEST-10-R1");
        assert_eq!(revised.issue_type(), IssueType::Revised);
        assert_ne!(revised.id(), original.id()); // 異なるID
    }
}
