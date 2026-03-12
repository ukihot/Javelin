// AccountMaster Entity - 勘定科目マスタエンティティ

use crate::chart_of_accounts::values::{AccountCode, AccountName, AccountType};

/// 勘定科目マスタ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountMaster {
    code: AccountCode,
    name: AccountName,
    account_type: AccountType,
    is_active: bool,
}

impl AccountMaster {
    pub fn new(
        code: AccountCode,
        name: AccountName,
        account_type: AccountType,
        is_active: bool,
    ) -> Self {
        Self { code, name, account_type, is_active }
    }

    pub fn code(&self) -> &AccountCode {
        &self.code
    }

    pub fn name(&self) -> &AccountName {
        &self.name
    }

    pub fn account_type(&self) -> AccountType {
        self.account_type
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_master() {
        let code = AccountCode::new("1000").unwrap();
        let name = AccountName::new("現金").unwrap();
        let master = AccountMaster::new(code, name, AccountType::Asset, true);

        assert_eq!(master.code().value(), "1000");
        assert_eq!(master.name().value(), "現金");
        assert_eq!(master.account_type(), AccountType::Asset);
        assert!(master.is_active());
    }

    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            #[test]
            fn prop_account_master_state_changes(
                code in "[0-9]{4,10}",
                name in "[\\p{Hiragana}\\p{Katakana}\\p{Han}a-zA-Z ]{1,50}",
                account_type in prop_oneof![
                    Just(AccountType::Asset),
                    Just(AccountType::Liability),
                    Just(AccountType::Equity),
                    Just(AccountType::Revenue),
                    Just(AccountType::Expense),
                ],
                is_active in any::<bool>()
            ) {
                let account_code = AccountCode::new(code).unwrap();
                let account_name = AccountName::new(name).unwrap();
                let mut master = AccountMaster::new(
                    account_code,
                    account_name,
                    account_type,
                    is_active
                );

                master.activate();
                prop_assert!(master.is_active());

                master.deactivate();
                prop_assert!(!master.is_active());
            }

            #[test]
            fn prop_account_type_consistency(
                account_type in prop_oneof![
                    Just(AccountType::Asset),
                    Just(AccountType::Liability),
                    Just(AccountType::Equity),
                    Just(AccountType::Revenue),
                    Just(AccountType::Expense),
                ]
            ) {
                let code = AccountCode::new("1000").unwrap();
                let name = AccountName::new("テスト").unwrap();
                let master = AccountMaster::new(code, name, account_type, true);

                prop_assert_eq!(master.account_type(), account_type);
            }
        }
    }
}
