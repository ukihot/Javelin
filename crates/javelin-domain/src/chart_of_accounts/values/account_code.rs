// AccountCode - 勘定科目コード値オブジェクト

use crate::{error::DomainResult, value_object::ValueObject};

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

    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            #[test]
            fn prop_valid_account_code(code in "[0-9]{4,10}") {
                let account_code = AccountCode::new(code.clone());
                prop_assert!(account_code.is_ok());
                let account_code = account_code.unwrap();
                prop_assert_eq!(account_code.value(), code.as_str());
            }

            #[test]
            fn prop_empty_account_code_fails(_unit in Just(())) {
                let account_code = AccountCode::new("");
                prop_assert!(account_code.is_err());
            }
        }
    }
}
