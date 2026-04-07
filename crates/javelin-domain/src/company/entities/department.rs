// Department Entity - 部署エンティティ
//
// 組織体制のツリー構造ノードとなる部署。
// 親部署への参照を持ち、階層構造を形成する。

use crate::{
    company::values::{DepartmentCode, DepartmentId, DepartmentName},
    entity::Entity,
};

/// 部署
#[derive(Debug, Clone)]
pub struct Department {
    id: DepartmentId,
    code: DepartmentCode,
    name: DepartmentName,
    parent_id: Option<DepartmentId>,
    is_active: bool,
}

impl Entity for Department {
    type Id = DepartmentId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Department {
    pub fn new(
        id: DepartmentId,
        code: DepartmentCode,
        name: DepartmentName,
        parent_id: Option<DepartmentId>,
    ) -> Self {
        Self { id, code, name, parent_id, is_active: true }
    }

    pub fn code(&self) -> &DepartmentCode {
        &self.code
    }

    pub fn name(&self) -> &DepartmentName {
        &self.name
    }

    pub fn parent_id(&self) -> Option<&DepartmentId> {
        self.parent_id.as_ref()
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn rename(&mut self, name: DepartmentName) {
        self.name = name;
    }

    pub fn move_to(&mut self, parent_id: Option<DepartmentId>) {
        self.parent_id = parent_id;
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
    use crate::entity::EntityId;

    #[test]
    fn test_department_creation() {
        let dept = Department::new(
            DepartmentId::generate(),
            DepartmentCode::new("FIN").unwrap(),
            DepartmentName::new("財務部").unwrap(),
            None,
        );
        assert_eq!(dept.code().value(), "FIN");
        assert_eq!(dept.name().value(), "財務部");
        assert!(dept.parent_id().is_none());
        assert!(dept.is_active());
    }

    #[test]
    fn test_department_hierarchy() {
        let parent_id = DepartmentId::generate();
        let child = Department::new(
            DepartmentId::generate(),
            DepartmentCode::new("FIN-ACC").unwrap(),
            DepartmentName::new("経理課").unwrap(),
            Some(parent_id.clone()),
        );
        assert_eq!(child.parent_id().unwrap().value(), parent_id.value());
    }

    #[test]
    fn test_department_move() {
        let mut dept = Department::new(
            DepartmentId::generate(),
            DepartmentCode::new("ACC").unwrap(),
            DepartmentName::new("経理課").unwrap(),
            None,
        );
        let new_parent = DepartmentId::generate();
        dept.move_to(Some(new_parent.clone()));
        assert_eq!(dept.parent_id().unwrap().value(), new_parent.value());
    }
}
