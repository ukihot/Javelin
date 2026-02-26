// CompanyMaster - 会社マスタドメイン

use crate::{error::DomainResult, value_object::ValueObject};

/// 会社マスタ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanyMaster {
    code: CompanyCode,
    name: CompanyName,
    is_active: bool,
}

impl CompanyMaster {
    pub fn new(code: CompanyCode, name: CompanyName, is_active: bool) -> Self {
        Self { code, name, is_active }
    }

    pub fn code(&self) -> &CompanyCode {
        &self.code
    }

    pub fn name(&self) -> &CompanyName {
        &self.name
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
    fn test_company_code() {
        let code = CompanyCode::new("C001");
        assert!(code.is_ok());
        assert_eq!(code.unwrap().value(), "C001");

        assert!(CompanyCode::new("").is_err());
    }

    #[test]
    fn test_company_name() {
        let name = CompanyName::new("テスト株式会社");
        assert!(name.is_ok());
        assert_eq!(name.unwrap().value(), "テスト株式会社");

        assert!(CompanyName::new("").is_err());
    }

    #[test]
    fn test_company_master() {
        let code = CompanyCode::new("C001").unwrap();
        let name = CompanyName::new("テスト株式会社").unwrap();
        let master = CompanyMaster::new(code, name, true);

        assert_eq!(master.code().value(), "C001");
        assert_eq!(master.name().value(), "テスト株式会社");
        assert!(master.is_active());
    }

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            // プロパティ1: 非空文字列の会社コードは常に作成可能
            #[test]
            fn prop_valid_company_code(code in "[A-Z0-9]{1,20}") {
                let company_code = CompanyCode::new(code.clone());
                prop_assert!(company_code.is_ok());
                let company_code = company_code.unwrap();
                prop_assert_eq!(company_code.value(), code.as_str());
            }

            // プロパティ2: 空文字列の会社コードは常にエラー
            #[test]
            fn prop_empty_company_code_fails(_unit in Just(())) {
                let company_code = CompanyCode::new("");
                prop_assert!(company_code.is_err());
            }

            // プロパティ3: 非空文字列の会社名は常に作成可能
            #[test]
            fn prop_valid_company_name(name in "[\\p{Hiragana}\\p{Katakana}\\p{Han}a-zA-Z ]{1,100}") {
                let company_name = CompanyName::new(name.clone());
                prop_assert!(company_name.is_ok());
                let company_name = company_name.unwrap();
                prop_assert_eq!(company_name.value(), name.as_str());
            }

            // プロパティ4: 空文字列の会社名は常にエラー
            #[test]
            fn prop_empty_company_name_fails(_unit in Just(())) {
                let company_name = CompanyName::new("");
                prop_assert!(company_name.is_err());
            }

            // プロパティ5: 会社マスタの作成と状態変更
            #[test]
            fn prop_company_master_state_changes(
                code in "[A-Z0-9]{1,20}",
                name in "[\\p{Hiragana}\\p{Katakana}\\p{Han}a-zA-Z ]{1,100}",
                is_active in any::<bool>()
            ) {
                let company_code = CompanyCode::new(code).unwrap();
                let company_name = CompanyName::new(name).unwrap();
                let mut master = CompanyMaster::new(company_code, company_name, is_active);

                // 状態変更のテスト
                master.activate();
                prop_assert!(master.is_active());

                master.deactivate();
                prop_assert!(!master.is_active());
            }
        }
    }
}
