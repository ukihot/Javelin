// AccountMaster - 勘定科目マスタドメイン

use crate::{error::DomainResult, value_object::ValueObject};

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

/// 勘定科目コード
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountCode(String);

impl AccountCode {
    pub fn new(code: impl Into<String>) -> DomainResult<Self> {
        let code = code.into();
        if code.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "勘定科目コードは空にできません".to_string(),
            ));
        }
        Ok(Self(code))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for AccountCode {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "勘定科目コードは空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

/// 勘定科目名
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountName(String);

impl AccountName {
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        if name.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "勘定科目名は空にできません".to_string(),
            ));
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for AccountName {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "勘定科目名は空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

/// 勘定科目タイプ
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_code() {
        let code = AccountCode::new("1000");
        assert!(code.is_ok());
        assert_eq!(code.unwrap().value(), "1000");

        assert!(AccountCode::new("").is_err());
    }

    #[test]
    fn test_account_name() {
        let name = AccountName::new("現金");
        assert!(name.is_ok());
        assert_eq!(name.unwrap().value(), "現金");

        assert!(AccountName::new("").is_err());
    }

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

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            // プロパティ1: 非空文字列の勘定科目コードは常に作成可能
            #[test]
            fn prop_valid_account_code(code in "[0-9]{4,10}") {
                let account_code = AccountCode::new(code.clone());
                prop_assert!(account_code.is_ok());
                let account_code = account_code.unwrap();
                prop_assert_eq!(account_code.value(), code.as_str());
            }

            // プロパティ2: 空文字列の勘定科目コードは常にエラー
            #[test]
            fn prop_empty_account_code_fails(_unit in Just(())) {
                let account_code = AccountCode::new("");
                prop_assert!(account_code.is_err());
            }

            // プロパティ3: 非空文字列の勘定科目名は常に作成可能
            #[test]
            fn prop_valid_account_name(name in "[\\p{Hiragana}\\p{Katakana}\\p{Han}a-zA-Z ]{1,50}") {
                let account_name = AccountName::new(name.clone());
                prop_assert!(account_name.is_ok());
                let account_name = account_name.unwrap();
                prop_assert_eq!(account_name.value(), name.as_str());
            }

            // プロパティ4: 空文字列の勘定科目名は常にエラー
            #[test]
            fn prop_empty_account_name_fails(_unit in Just(())) {
                let account_name = AccountName::new("");
                prop_assert!(account_name.is_err());
            }

            // プロパティ5: 勘定科目マスタの作成と状態変更
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

                // 状態変更のテスト
                master.activate();
                prop_assert!(master.is_active());

                master.deactivate();
                prop_assert!(!master.is_active());
            }

            // プロパティ6: 勘定科目タイプの一貫性
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
