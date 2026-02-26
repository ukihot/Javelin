// SystemMaster - システムマスタ集約

use super::{
    account_master::AccountMaster, company_master::CompanyMaster, system_settings::SystemSettings,
    user_settings::UserSettings,
};
use crate::{
    entity::{Entity, EntityId},
    error::DomainResult,
    value_object::ValueObject,
};

/// システムマスタ集約の識別子
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemMasterId(String);

impl SystemMasterId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl EntityId for SystemMasterId {
    fn value(&self) -> &str {
        &self.0
    }
}

/// システムマスタ集約
#[derive(Debug, Clone)]
pub struct SystemMaster {
    id: SystemMasterId,
    version: u64,
    account_masters: Vec<AccountMaster>,
    company_masters: Vec<CompanyMaster>,
    user_settings: UserSettings,
    system_settings: SystemSettings,
}

impl Entity for SystemMaster {
    type Id = SystemMasterId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl SystemMaster {
    pub fn new(
        id: SystemMasterId,
        account_masters: Vec<AccountMaster>,
        company_masters: Vec<CompanyMaster>,
        user_settings: UserSettings,
        system_settings: SystemSettings,
    ) -> Self {
        Self { id, version: 1, account_masters, company_masters, user_settings, system_settings }
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn account_masters(&self) -> &[AccountMaster] {
        &self.account_masters
    }

    pub fn company_masters(&self) -> &[CompanyMaster] {
        &self.company_masters
    }

    pub fn user_settings(&self) -> &UserSettings {
        &self.user_settings
    }

    pub fn system_settings(&self) -> &SystemSettings {
        &self.system_settings
    }

    pub fn update_account_masters(&mut self, account_masters: Vec<AccountMaster>) {
        self.account_masters = account_masters;
        self.version += 1;
    }

    pub fn update_company_masters(&mut self, company_masters: Vec<CompanyMaster>) {
        self.company_masters = company_masters;
        self.version += 1;
    }

    pub fn update_user_settings(&mut self, user_settings: UserSettings) {
        self.user_settings = user_settings;
        self.version += 1;
    }

    pub fn update_system_settings(&mut self, system_settings: SystemSettings) {
        self.system_settings = system_settings;
        self.version += 1;
    }

    pub fn add_account_master(&mut self, account_master: AccountMaster) {
        self.account_masters.push(account_master);
        self.version += 1;
    }

    pub fn remove_account_master(&mut self, account_code: &str) -> Option<AccountMaster> {
        if let Some(pos) =
            self.account_masters.iter().position(|am| am.code().value() == account_code)
        {
            let account_master = self.account_masters.remove(pos);
            self.version += 1;
            Some(account_master)
        } else {
            None
        }
    }

    pub fn find_account_master(&self, account_code: &str) -> Option<&AccountMaster> {
        self.account_masters.iter().find(|am| am.code().value() == account_code)
    }

    pub fn find_company_master(&self, company_code: &str) -> Option<&CompanyMaster> {
        self.company_masters.iter().find(|cm| cm.code().value() == company_code)
    }

    pub fn validate(&self) -> DomainResult<()> {
        // バリデーションロジック
        // 各マスタのバリデーションを実行
        for account_master in &self.account_masters {
            account_master.code().validate()?;
            account_master.name().validate()?;
        }

        for company_master in &self.company_masters {
            company_master.code().validate()?;
            company_master.name().validate()?;
        }

        self.user_settings.validate()?;
        self.system_settings.validate()?;

        Ok(())
    }
}
