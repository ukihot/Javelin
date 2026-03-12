// SubsidiaryAccountMaster Entity - 補助科目マスタエンティティ

use crate::chart_of_accounts::values::{AccountCode, SubsidiaryAccountCode, SubsidiaryAccountName};

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
