// Company Entity - 会社エンティティ
//
// 組織体の法人情報を表現する。OrganizationIdで組織体と紐付く。

use crate::company::values::{CompanyCode, CompanyName, OrganizationId};

/// 会社マスタ
///
/// 法人としての基本属性（コード・名前・有効性）を保持する。
/// 組織体（Organization）に所属し、OrganizationIdで紐付く。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanyMaster {
    organization_id: OrganizationId,
    code: CompanyCode,
    name: CompanyName,
    is_active: bool,
}

impl CompanyMaster {
    pub fn new(
        organization_id: OrganizationId,
        code: CompanyCode,
        name: CompanyName,
        is_active: bool,
    ) -> Self {
        Self { organization_id, code, name, is_active }
    }

    pub fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
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

    pub fn rename(&mut self, name: CompanyName) {
        self.name = name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_master() {
        let org_id = OrganizationId::generate();
        let code = CompanyCode::new("C001").unwrap();
        let name = CompanyName::new("テスト株式会社").unwrap();
        let master = CompanyMaster::new(org_id.clone(), code, name, true);

        assert_eq!(master.code().value(), "C001");
        assert_eq!(master.name().value(), "テスト株式会社");
        assert!(master.is_active());
        assert_eq!(master.organization_id(), &org_id);
    }

    #[test]
    fn test_company_master_rename() {
        let org_id = OrganizationId::generate();
        let code = CompanyCode::new("C001").unwrap();
        let name = CompanyName::new("旧会社名").unwrap();
        let mut master = CompanyMaster::new(org_id, code, name, true);

        let new_name = CompanyName::new("新会社名").unwrap();
        master.rename(new_name);
        assert_eq!(master.name().value(), "新会社名");
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
                let org_id = OrganizationId::generate();
                let company_code = CompanyCode::new(code).unwrap();
                let company_name = CompanyName::new(name).unwrap();
                let mut master = CompanyMaster::new(org_id, company_code, company_name, is_active);

                master.activate();
                prop_assert!(master.is_active());

                master.deactivate();
                prop_assert!(!master.is_active());
            }
        }
    }
}
