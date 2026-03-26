// 取引先集約の値オブジェクト

use serde::{Deserialize, Serialize};

use crate::value_object::ValueObject;

/// 取引先タイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartnerType {
    Customer, // 顧客
    Supplier, // 仕入先
    Both,     // 両方
    Generic,  // 諸口（一過性・一時的な取引先）
}

impl ValueObject for PartnerType {
    fn validate(&self) -> crate::error::DomainResult<()> {
        Ok(())
    }
}

/// 銀行口座情報
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BankAccount {
    bank_code: String,      // 銀行コード
    branch_code: String,    // 支店コード
    account_type: String,   // 口座種別（普通、当座など）
    account_number: String, // 口座番号
}

impl BankAccount {
    pub fn new(
        bank_code: String,
        branch_code: String,
        account_type: String,
        account_number: String,
    ) -> crate::error::DomainResult<Self> {
        if bank_code.is_empty() || branch_code.is_empty() || account_number.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "銀行口座情報は空にできません".to_string(),
            ));
        }
        Ok(Self { bank_code, branch_code, account_type, account_number })
    }

    pub fn bank_code(&self) -> &str {
        &self.bank_code
    }

    pub fn branch_code(&self) -> &str {
        &self.branch_code
    }

    pub fn account_type(&self) -> &str {
        &self.account_type
    }

    pub fn account_number(&self) -> &str {
        &self.account_number
    }
}

impl ValueObject for BankAccount {
    fn validate(&self) -> crate::error::DomainResult<()> {
        if self.bank_code.is_empty()
            || self.branch_code.is_empty()
            || self.account_number.is_empty()
        {
            return Err(crate::error::DomainError::ValidationError(
                "銀行口座情報は必須です".to_string(),
            ));
        }
        Ok(())
    }
}

/// インボイス登録番号
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceRegistrationNumber(String);

impl InvoiceRegistrationNumber {
    pub fn new(number: String) -> crate::error::DomainResult<Self> {
        if number.trim().is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "インボイス登録番号は空にできません".to_string(),
            ));
        }
        Ok(Self(number))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for InvoiceRegistrationNumber {
    fn validate(&self) -> crate::error::DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "インボイス登録番号は必須です".to_string(),
            ));
        }
        Ok(())
    }
}

/// 連絡先情報
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactInfo {
    address: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    contact_person: Option<String>,
}

impl ContactInfo {
    pub fn new(
        address: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        contact_person: Option<String>,
    ) -> Self {
        Self { address, phone, email, contact_person }
    }

    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }

    pub fn phone(&self) -> Option<&str> {
        self.phone.as_deref()
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn contact_person(&self) -> Option<&str> {
        self.contact_person.as_deref()
    }
}

impl ValueObject for ContactInfo {
    fn validate(&self) -> crate::error::DomainResult<()> {
        Ok(())
    }
}
