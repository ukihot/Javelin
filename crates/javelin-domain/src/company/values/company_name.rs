// CompanyName - 会社名値オブジェクト

use crate::{error::DomainResult, value_object::ValueObject};

/// 会社名
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanyName(String);

impl CompanyName {
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        if name.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "会社名は空にできません".to_string(),
            ));
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for CompanyName {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "会社名は空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_name() {
        let name = CompanyName::new("テスト株式会社");
        assert!(name.is_ok());
        assert_eq!(name.unwrap().value(), "テスト株式会社");

        assert!(CompanyName::new("").is_err());
    }

    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            #[test]
            fn prop_valid_company_name(name in "[\\p{Hiragana}\\p{Katakana}\\p{Han}a-zA-Z ]{1,100}") {
                let company_name = CompanyName::new(name.clone());
                prop_assert!(company_name.is_ok());
                let company_name = company_name.unwrap();
                prop_assert_eq!(company_name.value(), name.as_str());
            }

            #[test]
            fn prop_empty_company_name_fails(_unit in Just(())) {
                let company_name = CompanyName::new("");
                prop_assert!(company_name.is_err());
            }
        }
    }
}
