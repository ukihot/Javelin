// CompanyCode - 会社コード値オブジェクト

use crate::{error::DomainResult, value_object::ValueObject};

/// 会社コード
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompanyCode(String);

impl CompanyCode {
    pub fn new(code: impl Into<String>) -> DomainResult<Self> {
        let code = code.into();
        if code.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "会社コードは空にできません".to_string(),
            ));
        }
        Ok(Self(code))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for CompanyCode {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "会社コードは空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_code() {
        let code = CompanyCode::new("C001");
        assert!(code.is_ok());
        assert_eq!(code.unwrap().value(), "C001");

        assert!(CompanyCode::new("").is_err());
    }

    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            #[test]
            fn prop_valid_company_code(code in "[A-Z0-9]{1,20}") {
                let company_code = CompanyCode::new(code.clone());
                prop_assert!(company_code.is_ok());
                let company_code = company_code.unwrap();
                prop_assert_eq!(company_code.value(), code.as_str());
            }

            #[test]
            fn prop_empty_company_code_fails(_unit in Just(())) {
                let company_code = CompanyCode::new("");
                prop_assert!(company_code.is_err());
            }
        }
    }
}
