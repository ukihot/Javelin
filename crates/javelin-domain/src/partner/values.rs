// 取引先集約の値オブジェクト

use serde::{Deserialize, Serialize};

use crate::value_object::ValueObject;

/// 取引先タイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartnerType {
    Customer, // 顧客
    Supplier, // 仕入先
    Both,     // 両方
}

impl ValueObject for PartnerType {
    fn validate(&self) -> crate::error::DomainResult<()> {
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
