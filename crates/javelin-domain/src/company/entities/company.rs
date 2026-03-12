// Company Entity - 会社エンティティ

use crate::company::values::{CompanyCode, CompanyName};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_master() {
        let code = CompanyCode::new("C001").unwrap();
        let name = CompanyName::new("テスト株式会社").unwrap();
        let master = CompanyMaster::new(code, name, true);

        assert_eq!(master.code().value(), "C001");
        assert_eq!(master.name().value(), "テスト株式会社");
        assert!(master.is_active());
    }

    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            #[test]
            fn prop_company_master_state_changes(
                code in "[A-Z0-9]{1,20}",
                name in "[\\p{Hiragana}\\p{Katakana}\\p{Han}a-zA-Z ]{1,100}",
                is_active in any::<bool>()
            ) {
                let company_code = CompanyCode::new(code).unwrap();
                let company_name = CompanyName::new(name).unwrap();
                let mut master = CompanyMaster::new(company_code, company_name, is_active);

                master.activate();
                prop_assert!(master.is_active());

                master.deactivate();
                prop_assert!(!master.is_active());
            }
        }
    }
}
