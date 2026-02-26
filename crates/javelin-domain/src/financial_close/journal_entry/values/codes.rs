// コード関連の値オブジェクト

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 補助科目コード
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubAccountCode(String);

impl ValueObject for SubAccountCode {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }
        Ok(())
    }
}

impl SubAccountCode {
    pub fn new(code: String) -> DomainResult<Self> {
        let sub_account_code = Self(code);
        sub_account_code.validate()?;
        Ok(sub_account_code)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 部門コード
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepartmentCode(String);

impl ValueObject for DepartmentCode {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount("Department code cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl DepartmentCode {
    pub fn new(code: String) -> DomainResult<Self> {
        let department_code = Self(code);
        department_code.validate()?;
        Ok(department_code)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_account_code() {
        let code = SubAccountCode::new("SUB001".to_string());
        assert!(code.is_ok());
        assert_eq!(code.unwrap().value(), "SUB001");

        assert!(SubAccountCode::new("".to_string()).is_err());
    }

    #[test]
    fn test_department_code() {
        let code = DepartmentCode::new("DEPT001".to_string());
        assert!(code.is_ok());
        assert_eq!(code.unwrap().value(), "DEPT001");

        assert!(DepartmentCode::new("".to_string()).is_err());
    }

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            // プロパティ1: 非空文字列の補助科目コードは常に作成可能
            #[test]
            fn prop_valid_sub_account_code(code in "[A-Z0-9]{1,20}") {
                let sub_account_code = SubAccountCode::new(code.clone());
                prop_assert!(sub_account_code.is_ok());
                let sub_account_code = sub_account_code.unwrap();
                prop_assert_eq!(sub_account_code.value(), code.as_str());
            }

            // プロパティ2: 空文字列の補助科目コードは常にエラー
            #[test]
            fn prop_empty_sub_account_code_fails(_unit in Just(())) {
                let sub_account_code = SubAccountCode::new("".to_string());
                prop_assert!(sub_account_code.is_err());
            }

            // プロパティ3: 非空文字列の部門コードは常に作成可能
            #[test]
            fn prop_valid_department_code(code in "[A-Z0-9]{1,20}") {
                let department_code = DepartmentCode::new(code.clone());
                prop_assert!(department_code.is_ok());
                let department_code = department_code.unwrap();
                prop_assert_eq!(department_code.value(), code.as_str());
            }

            // プロパティ4: 空文字列の部門コードは常にエラー
            #[test]
            fn prop_empty_department_code_fails(_unit in Just(())) {
                let department_code = DepartmentCode::new("".to_string());
                prop_assert!(department_code.is_err());
            }
        }
    }
}
