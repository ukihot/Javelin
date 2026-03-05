// Billing Values - 請求集約の値オブジェクト

use bigdecimal::ToPrimitive;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::Amount,
    entity::EntityId,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 請求書ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InvoiceId(String);

impl InvoiceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Default for InvoiceId {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityId for InvoiceId {
    fn value(&self) -> &str {
        &self.0
    }
}

/// 請求書番号
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceNumber(String);

impl InvoiceNumber {
    pub fn new(number: String) -> DomainResult<Self> {
        if number.trim().is_empty() {
            return Err(DomainError::ValidationError("請求書番号は空にできません".to_string()));
        }
        Ok(Self(number))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for InvoiceNumber {
    fn validate(&self) -> DomainResult<()> {
        if self.0.trim().is_empty() {
            return Err(DomainError::ValidationError("請求書番号は空にできません".to_string()));
        }
        Ok(())
    }
}

/// 請求先情報
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingRecipient {
    name: String,
    address: Option<String>,
    contact_person: Option<String>,
}

impl BillingRecipient {
    pub fn new(name: String) -> DomainResult<Self> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError("請求先名は空にできません".to_string()));
        }
        Ok(Self { name, address: None, contact_person: None })
    }

    pub fn with_details(
        name: String,
        address: Option<String>,
        contact_person: Option<String>,
    ) -> DomainResult<Self> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError("請求先名は空にできません".to_string()));
        }
        Ok(Self { name, address, contact_person })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }

    pub fn contact_person(&self) -> Option<&str> {
        self.contact_person.as_deref()
    }
}

impl ValueObject for BillingRecipient {
    fn validate(&self) -> DomainResult<()> {
        if self.name.trim().is_empty() {
            return Err(DomainError::ValidationError("請求先名は空にできません".to_string()));
        }
        Ok(())
    }
}

/// 発行者情報
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssuerInfo {
    name: String,
    department: Option<String>,
    address: String,
    tel: String,
    email: Option<String>,
    registration_number: Option<String>, // インボイス登録番号
}

impl IssuerInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        department: Option<String>,
        address: String,
        tel: String,
        email: Option<String>,
        registration_number: Option<String>,
    ) -> DomainResult<Self> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError("発行者名は空にできません".to_string()));
        }
        if address.trim().is_empty() {
            return Err(DomainError::ValidationError("住所は空にできません".to_string()));
        }
        if tel.trim().is_empty() {
            return Err(DomainError::ValidationError("電話番号は空にできません".to_string()));
        }
        Ok(Self { name, department, address, tel, email, registration_number })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn department(&self) -> Option<&str> {
        self.department.as_deref()
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn tel(&self) -> &str {
        &self.tel
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn registration_number(&self) -> Option<&str> {
        self.registration_number.as_deref()
    }
}

impl ValueObject for IssuerInfo {
    fn validate(&self) -> DomainResult<()> {
        if self.name.trim().is_empty() {
            return Err(DomainError::ValidationError("発行者名は空にできません".to_string()));
        }
        if self.address.trim().is_empty() {
            return Err(DomainError::ValidationError("住所は空にできません".to_string()));
        }
        if self.tel.trim().is_empty() {
            return Err(DomainError::ValidationError("電話番号は空にできません".to_string()));
        }
        Ok(())
    }
}

/// 銀行情報
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BankInfo {
    bank_name: String,
    branch_name: String,
    account_type: String, // 普通/当座
    account_number: String,
    account_holder: String,
}

impl BankInfo {
    pub fn new(
        bank_name: String,
        branch_name: String,
        account_type: String,
        account_number: String,
        account_holder: String,
    ) -> DomainResult<Self> {
        if bank_name.trim().is_empty() {
            return Err(DomainError::ValidationError("銀行名は空にできません".to_string()));
        }
        if branch_name.trim().is_empty() {
            return Err(DomainError::ValidationError("支店名は空にできません".to_string()));
        }
        if account_number.trim().is_empty() {
            return Err(DomainError::ValidationError("口座番号は空にできません".to_string()));
        }
        if account_holder.trim().is_empty() {
            return Err(DomainError::ValidationError("口座名義は空にできません".to_string()));
        }
        Ok(Self { bank_name, branch_name, account_type, account_number, account_holder })
    }

    pub fn bank_name(&self) -> &str {
        &self.bank_name
    }

    pub fn branch_name(&self) -> &str {
        &self.branch_name
    }

    pub fn account_type(&self) -> &str {
        &self.account_type
    }

    pub fn account_number(&self) -> &str {
        &self.account_number
    }

    pub fn account_holder(&self) -> &str {
        &self.account_holder
    }
}

impl ValueObject for BankInfo {
    fn validate(&self) -> DomainResult<()> {
        if self.bank_name.trim().is_empty() {
            return Err(DomainError::ValidationError("銀行名は空にできません".to_string()));
        }
        if self.branch_name.trim().is_empty() {
            return Err(DomainError::ValidationError("支店名は空にできません".to_string()));
        }
        if self.account_number.trim().is_empty() {
            return Err(DomainError::ValidationError("口座番号は空にできません".to_string()));
        }
        if self.account_holder.trim().is_empty() {
            return Err(DomainError::ValidationError("口座名義は空にできません".to_string()));
        }
        Ok(())
    }
}

/// 発行区分
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueType {
    Original,  // 原本
    Duplicate, // 控え
    Revised,   // 訂正版
}

/// 文書フェーズ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentPhase {
    Commercial, // 正式請求書
    Proforma,   // 見積請求書（Pro forma invoice）
}

/// 決済ステータス
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettlementStatus {
    Unpaid,  // 未払い
    Paid,    // 支払済み
    Overdue, // 期限超過
}

impl SettlementStatus {
    /// 支払期限と支払状況から決済ステータスを判定
    pub fn determine(due_date: NaiveDate, is_paid: bool, current_date: NaiveDate) -> Self {
        if is_paid {
            Self::Paid
        } else if current_date > due_date {
            Self::Overdue
        } else {
            Self::Unpaid
        }
    }
}

/// 税率
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TaxRate(u8);

impl TaxRate {
    pub fn new(rate: u8) -> DomainResult<Self> {
        if rate > 100 {
            return Err(DomainError::ValidationError(
                "税率は0-100の範囲である必要があります".to_string(),
            ));
        }
        Ok(Self(rate))
    }

    pub fn rate(&self) -> u8 {
        self.0
    }

    pub fn calculate_tax(&self, amount: &Amount) -> Amount {
        let tax_value = amount.value() * bigdecimal::BigDecimal::from(self.0);
        let divisor = bigdecimal::BigDecimal::from(100);
        Amount::from(tax_value / divisor)
    }
}

impl ValueObject for TaxRate {
    fn validate(&self) -> DomainResult<()> {
        if self.0 > 100 {
            return Err(DomainError::ValidationError(
                "税率は0-100の範囲である必要があります".to_string(),
            ));
        }
        Ok(())
    }
}

/// 請求明細
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    description: String,
    unit_price: Amount,
    quantity: u32,
    tax_rate: TaxRate,
}

impl InvoiceLineItem {
    pub fn new(
        description: String,
        unit_price: Amount,
        quantity: u32,
        tax_rate: TaxRate,
    ) -> DomainResult<Self> {
        if description.trim().is_empty() {
            return Err(DomainError::ValidationError("明細説明は空にできません".to_string()));
        }
        if quantity == 0 {
            return Err(DomainError::ValidationError(
                "数量は1以上である必要があります".to_string(),
            ));
        }
        Ok(Self { description, unit_price, quantity, tax_rate })
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn unit_price(&self) -> &Amount {
        &self.unit_price
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn tax_rate(&self) -> TaxRate {
        self.tax_rate
    }

    /// 小計（税抜）
    pub fn subtotal(&self) -> Amount {
        Amount::from_i64(self.unit_price.value().to_i64().unwrap_or(0) * self.quantity as i64)
    }

    /// 税額
    pub fn tax_amount(&self) -> Amount {
        self.tax_rate.calculate_tax(&self.subtotal())
    }

    /// 合計（税込）
    pub fn total(&self) -> Amount {
        &self.subtotal() + &self.tax_amount()
    }
}

impl ValueObject for InvoiceLineItem {
    fn validate(&self) -> DomainResult<()> {
        if self.description.trim().is_empty() {
            return Err(DomainError::ValidationError("明細説明は空にできません".to_string()));
        }
        if self.quantity == 0 {
            return Err(DomainError::ValidationError(
                "数量は1以上である必要があります".to_string(),
            ));
        }
        self.tax_rate.validate()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invoice_id_equality() {
        let id1 = InvoiceId::from_string("test-id-1".to_string());
        let id2 = InvoiceId::from_string("test-id-1".to_string());
        let id3 = InvoiceId::from_string("test-id-2".to_string());

        // 同じIDは等しい
        assert_eq!(id1, id2);
        // 異なるIDは等しくない
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_invoice_number_validation() {
        let valid = InvoiceNumber::new("CI-2026-BEEF-01".to_string());
        assert!(valid.is_ok());

        let invalid = InvoiceNumber::new("".to_string());
        assert!(invalid.is_err());

        let whitespace = InvoiceNumber::new("   ".to_string());
        assert!(whitespace.is_err());
    }

    #[test]
    fn test_billing_recipient_equality() {
        let recipient1 = BillingRecipient::new("Company A".to_string()).unwrap();
        let recipient2 = BillingRecipient::new("Company A".to_string()).unwrap();
        let recipient3 = BillingRecipient::new("Company B".to_string()).unwrap();

        // 値オブジェクトは値が同じなら等しい
        assert_eq!(recipient1, recipient2);
        assert_ne!(recipient1, recipient3);
    }

    #[test]
    fn test_tax_rate_calculation() {
        let rate = TaxRate::new(10).unwrap();
        let amount = Amount::from_i64(1000);
        let tax = rate.calculate_tax(&amount);

        // 10%の税額は100
        assert_eq!(tax.to_i64(), Some(100));
    }

    #[test]
    fn test_invoice_line_item_calculations() {
        let item = InvoiceLineItem::new(
            "Test Item".to_string(),
            Amount::from_i64(1000),
            3,
            TaxRate::new(10).unwrap(),
        )
        .unwrap();

        // 小計: 1000 * 3 = 3000
        assert_eq!(item.subtotal().to_i64(), Some(3000));
        // 税額: 3000 * 10% = 300
        assert_eq!(item.tax_amount().to_i64(), Some(300));
        // 合計: 3000 + 300 = 3300
        assert_eq!(item.total().to_i64(), Some(3300));
    }

    #[test]
    fn test_invoice_line_item_equality() {
        let item1 = InvoiceLineItem::new(
            "Item A".to_string(),
            Amount::from_i64(1000),
            2,
            TaxRate::new(10).unwrap(),
        )
        .unwrap();

        let item2 = InvoiceLineItem::new(
            "Item A".to_string(),
            Amount::from_i64(1000),
            2,
            TaxRate::new(10).unwrap(),
        )
        .unwrap();

        let item3 = InvoiceLineItem::new(
            "Item B".to_string(),
            Amount::from_i64(1000),
            2,
            TaxRate::new(10).unwrap(),
        )
        .unwrap();

        // 値オブジェクトは値が同じなら等しい
        assert_eq!(item1, item2);
        assert_ne!(item1, item3);
    }

    #[test]
    fn test_settlement_status_determination() {
        use chrono::NaiveDate;

        let due_date = NaiveDate::from_ymd_opt(2026, 3, 31).unwrap();
        let before_due = NaiveDate::from_ymd_opt(2026, 3, 15).unwrap();
        let after_due = NaiveDate::from_ymd_opt(2026, 4, 5).unwrap();

        // 支払済み
        assert_eq!(SettlementStatus::determine(due_date, true, before_due), SettlementStatus::Paid);
        assert_eq!(SettlementStatus::determine(due_date, true, after_due), SettlementStatus::Paid);

        // 未払い（期限前）
        assert_eq!(
            SettlementStatus::determine(due_date, false, before_due),
            SettlementStatus::Unpaid
        );

        // 期限超過
        assert_eq!(
            SettlementStatus::determine(due_date, false, after_due),
            SettlementStatus::Overdue
        );
    }

    #[test]
    fn test_issue_type_and_document_phase() {
        // Enumの基本的な動作確認
        assert_eq!(IssueType::Original, IssueType::Original);
        assert_ne!(IssueType::Original, IssueType::Duplicate);

        assert_eq!(DocumentPhase::Commercial, DocumentPhase::Commercial);
        assert_ne!(DocumentPhase::Commercial, DocumentPhase::Proforma);
    }
}
