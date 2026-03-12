// AccountName - 勘定科目名値オブジェクト

use crate::{error::DomainResult, value_object::ValueObject};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_name() {
        let name = AccountName::new("現金");
        assert!(name.is_ok());
        assert_eq!(name.unwrap().value(), "現金");

        assert!(AccountName::new("").is_err());
    }

    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            #[test]
            fn prop_valid_account_name(name in "[\\p{Hiragana}\\p{Katakana}\\p{Han}a-zA-Z ]{1,50}") {
                let account_name = AccountName::new(name.clone());
                prop_assert!(account_name.is_ok());
                let account_name = account_name.unwrap();
                prop_assert_eq!(account_name.value(), name.as_str());
            }

            #[test]
            fn prop_empty_account_name_fails(_unit in Just(())) {
                let account_name = AccountName::new("");
                prop_assert!(account_name.is_err());
            }
        }
    }
}
