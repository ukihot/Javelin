// Member Entity - メンバーエンティティ
//
// 組織に所属するユーザ（メンバー）を表現する。
// 部署・役職に紐付き、複数のロールを持つことで権限が決定される。

use crate::{
    company::values::{DepartmentId, Email, MemberId, MemberName, PositionId, RoleId},
    entity::Entity,
};

/// メンバー
#[derive(Debug, Clone)]
pub struct Member {
    id: MemberId,
    name: MemberName,
    email: Email,
    department_id: DepartmentId,
    position_id: PositionId,
    role_ids: Vec<RoleId>,
    is_active: bool,
}

impl Entity for Member {
    type Id = MemberId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Member {
    pub fn new(
        id: MemberId,
        name: MemberName,
        email: Email,
        department_id: DepartmentId,
        position_id: PositionId,
        role_ids: Vec<RoleId>,
    ) -> Self {
        Self { id, name, email, department_id, position_id, role_ids, is_active: true }
    }

    pub fn name(&self) -> &MemberName {
        &self.name
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn department_id(&self) -> &DepartmentId {
        &self.department_id
    }

    pub fn position_id(&self) -> &PositionId {
        &self.position_id
    }

    pub fn role_ids(&self) -> &[RoleId] {
        &self.role_ids
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn rename(&mut self, name: MemberName) {
        self.name = name;
    }

    pub fn change_email(&mut self, email: Email) {
        self.email = email;
    }

    pub fn transfer(&mut self, department_id: DepartmentId, position_id: PositionId) {
        self.department_id = department_id;
        self.position_id = position_id;
    }

    pub fn assign_role(&mut self, role_id: RoleId) {
        if !self.role_ids.contains(&role_id) {
            self.role_ids.push(role_id);
        }
    }

    pub fn revoke_role(&mut self, role_id: &RoleId) {
        self.role_ids.retain(|r| r != role_id);
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
    use crate::company::values::*;

    fn create_test_member() -> Member {
        Member::new(
            MemberId::generate(),
            MemberName::new("山田太郎").unwrap(),
            Email::new("yamada@example.com").unwrap(),
            DepartmentId::generate(),
            PositionId::generate(),
            vec![],
        )
    }

    #[test]
    fn test_member_creation() {
        let member = create_test_member();
        assert_eq!(member.name().value(), "山田太郎");
        assert_eq!(member.email().value(), "yamada@example.com");
        assert!(member.is_active());
        assert!(member.role_ids().is_empty());
    }

    #[test]
    fn test_member_role_management() {
        let mut member = create_test_member();
        let role_id = RoleId::generate();

        member.assign_role(role_id.clone());
        assert_eq!(member.role_ids().len(), 1);

        // 重複追加は無視
        member.assign_role(role_id.clone());
        assert_eq!(member.role_ids().len(), 1);

        member.revoke_role(&role_id);
        assert!(member.role_ids().is_empty());
    }

    #[test]
    fn test_member_transfer() {
        let mut member = create_test_member();
        let new_dept = DepartmentId::generate();
        let new_pos = PositionId::generate();

        member.transfer(new_dept.clone(), new_pos.clone());
        assert_eq!(member.department_id(), &new_dept);
        assert_eq!(member.position_id(), &new_pos);
    }

    #[test]
    fn test_member_deactivate() {
        let mut member = create_test_member();
        assert!(member.is_active());

        member.deactivate();
        assert!(!member.is_active());

        member.activate();
        assert!(member.is_active());
    }
}
