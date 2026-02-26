// SubsidiaryAccountMaster - 補助科目マスタドメイン

use super::account_master::AccountCode;
use crate::{error::DomainResult, value_object::ValueObject};

/// 補助科目マスタ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubsidiaryAccountMaster {
    code: SubsidiaryAccountCode,
    name: SubsidiaryAccountName,
    parent_account_code: AccountCode,
    is_active: bool,
}

impl SubsidiaryAccountMaster {
    pub fn new(
        code: SubsidiaryAccountCode,
        name: SubsidiaryAccountName,
        parent_account_code: AccountCode,
        is_active: bool,
    ) -> Self {
        Self { code, name, parent_account_code, is_active }
    }

    pub fn code(&self) -> &SubsidiaryAccountCode {
        &self.code
    }

    pub fn name(&self) -> &SubsidiaryAccountName {
        &self.name
    }

    pub fn parent_account_code(&self) -> &AccountCode {
        &self.parent_account_code
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

/// 補助科目コード
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubsidiaryAccountCode(String);

impl SubsidiaryAccountCode {
    pub fn new(code: impl Into<String>) -> DomainResult<Self> {
        let code = code.into();
        if code.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "補助科目コードは空にできません".to_string(),
            ));
        }
        Ok(Self(code))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for SubsidiaryAccountCode {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "補助科目コードは空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

/// 補助科目名
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubsidiaryAccountName(String);

impl SubsidiaryAccountName {
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        if name.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "補助科目名は空にできません".to_string(),
            ));
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for SubsidiaryAccountName {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "補助科目名は空にできません".to_string(),
            ));
        }
        Ok(())
    }
}
