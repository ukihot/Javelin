// Organization Entity - 組織体エンティティ（ルート集約）
//
// 組織体制全体を管理するルート集約。
// 部署ツリー・役職・メンバー・ロールを統合管理する。

use super::{CompanyMaster, Department, Member, Position, Role};
use crate::{
    company::values::{
        CompanyCode, CompanyName, DepartmentId, MemberId, OrganizationId, PositionId, RoleId,
    },
    entity::Entity,
    error::{DomainError, DomainResult},
};

/// 組織体（ルート集約）
///
/// 1つの組織体に紐づく全エンティティのライフサイクルを管理する。
/// - 法人情報（CompanyMaster）
/// - 部署ツリー
/// - 役職定義
/// - メンバー
/// - ロール（権限セット）
#[derive(Debug, Clone)]
pub struct Organization {
    id: OrganizationId,
    company: CompanyMaster,
    departments: Vec<Department>,
    positions: Vec<Position>,
    members: Vec<Member>,
    roles: Vec<Role>,
}

impl Entity for Organization {
    type Id = OrganizationId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Organization {
    /// 新しい組織体を作成
    pub fn new(id: OrganizationId, code: CompanyCode, name: CompanyName) -> Self {
        let company = CompanyMaster::new(id.clone(), code, name, true);
        Self {
            id,
            company,
            departments: Vec::new(),
            positions: Vec::new(),
            members: Vec::new(),
            roles: Vec::new(),
        }
    }

    // --- 法人情報 ---

    pub fn company(&self) -> &CompanyMaster {
        &self.company
    }

    pub fn company_mut(&mut self) -> &mut CompanyMaster {
        &mut self.company
    }

    // --- 部署管理 ---

    pub fn departments(&self) -> &[Department] {
        &self.departments
    }

    /// 部署を追加（コード重複チェック付き）
    pub fn add_department(&mut self, department: Department) -> DomainResult<()> {
        if self.departments.iter().any(|d| d.code() == department.code()) {
            return Err(DomainError::ValidationError(format!(
                "部署コード '{}' は既に存在します",
                department.code()
            )));
        }
        if let Some(parent_id) = department.parent_id() {
            if !self.departments.iter().any(|d| d.id() == parent_id) {
                return Err(DomainError::ValidationError(format!(
                    "親部署 '{}' が存在しません",
                    parent_id
                )));
            }
        }
        self.departments.push(department);
        Ok(())
    }

    /// 部署をIDで検索
    pub fn find_department(&self, id: &DepartmentId) -> Option<&Department> {
        self.departments.iter().find(|d| d.id() == id)
    }

    /// 部署をIDで検索（可変参照）
    pub fn find_department_mut(&mut self, id: &DepartmentId) -> Option<&mut Department> {
        self.departments.iter_mut().find(|d| d.id() == id)
    }

    /// 部署を削除（子部署やメンバーが存在する場合はエラー）
    pub fn remove_department(&mut self, id: &DepartmentId) -> DomainResult<()> {
        if self.departments.iter().any(|d| d.parent_id() == Some(id)) {
            return Err(DomainError::ValidationError(
                "子部署が存在するため削除できません".to_string(),
            ));
        }
        if self.members.iter().any(|m| m.department_id() == id) {
            return Err(DomainError::ValidationError(
                "所属メンバーが存在するため削除できません".to_string(),
            ));
        }
        self.departments.retain(|d| d.id() != id);
        Ok(())
    }

    /// ルート部署（親部署なし）を取得
    pub fn root_departments(&self) -> Vec<&Department> {
        self.departments.iter().filter(|d| d.parent_id().is_none()).collect()
    }

    /// 指定部署の子部署を取得
    pub fn child_departments(&self, parent_id: &DepartmentId) -> Vec<&Department> {
        self.departments.iter().filter(|d| d.parent_id() == Some(parent_id)).collect()
    }

    // --- 役職管理 ---

    pub fn positions(&self) -> &[Position] {
        &self.positions
    }

    /// 役職を追加（名前重複チェック付き）
    pub fn add_position(&mut self, position: Position) -> DomainResult<()> {
        if self.positions.iter().any(|p| p.name() == position.name()) {
            return Err(DomainError::ValidationError(format!(
                "役職名 '{}' は既に存在します",
                position.name()
            )));
        }
        self.positions.push(position);
        Ok(())
    }

    /// 役職をIDで検索
    pub fn find_position(&self, id: &PositionId) -> Option<&Position> {
        self.positions.iter().find(|p| p.id() == id)
    }

    /// 役職を削除（メンバーが使用中の場合はエラー）
    pub fn remove_position(&mut self, id: &PositionId) -> DomainResult<()> {
        if self.members.iter().any(|m| m.position_id() == id) {
            return Err(DomainError::ValidationError("使用中の役職は削除できません".to_string()));
        }
        self.positions.retain(|p| p.id() != id);
        Ok(())
    }

    // --- メンバー管理 ---

    pub fn members(&self) -> &[Member] {
        &self.members
    }

    /// メンバーを追加（部署・役職の存在確認付き）
    pub fn add_member(&mut self, member: Member) -> DomainResult<()> {
        if self.members.iter().any(|m| m.email() == member.email()) {
            return Err(DomainError::ValidationError(format!(
                "メールアドレス '{}' は既に使用されています",
                member.email()
            )));
        }
        if self.find_department(member.department_id()).is_none() {
            return Err(DomainError::ValidationError(format!(
                "部署 '{}' が存在しません",
                member.department_id()
            )));
        }
        if self.find_position(member.position_id()).is_none() {
            return Err(DomainError::ValidationError(format!(
                "役職 '{}' が存在しません",
                member.position_id()
            )));
        }
        for role_id in member.role_ids() {
            if self.find_role(role_id).is_none() {
                return Err(DomainError::ValidationError(format!(
                    "ロール '{}' が存在しません",
                    role_id
                )));
            }
        }
        self.members.push(member);
        Ok(())
    }

    /// メンバーをIDで検索
    pub fn find_member(&self, id: &MemberId) -> Option<&Member> {
        self.members.iter().find(|m| m.id() == id)
    }

    /// メンバーをIDで検索（可変参照）
    pub fn find_member_mut(&mut self, id: &MemberId) -> Option<&mut Member> {
        self.members.iter_mut().find(|m| m.id() == id)
    }

    /// 部署に所属するメンバーを取得
    pub fn members_in_department(&self, department_id: &DepartmentId) -> Vec<&Member> {
        self.members.iter().filter(|m| m.department_id() == department_id).collect()
    }

    /// メンバーを無効化
    pub fn deactivate_member(&mut self, id: &MemberId) -> DomainResult<()> {
        let Some(member) = self.find_member_mut(id) else {
            return Err(DomainError::EntityNotFound(format!("メンバー '{}'", id)));
        };
        member.deactivate();
        Ok(())
    }

    // --- ロール管理 ---

    pub fn roles(&self) -> &[Role] {
        &self.roles
    }

    /// ロールを追加（名前重複チェック付き）
    pub fn add_role(&mut self, role: Role) -> DomainResult<()> {
        if self.roles.iter().any(|r| r.name() == role.name()) {
            return Err(DomainError::ValidationError(format!(
                "ロール名 '{}' は既に存在します",
                role.name()
            )));
        }
        self.roles.push(role);
        Ok(())
    }

    /// ロールをIDで検索
    pub fn find_role(&self, id: &RoleId) -> Option<&Role> {
        self.roles.iter().find(|r| r.id() == id)
    }

    /// ロールを削除（メンバーが使用中の場合はエラー）
    pub fn remove_role(&mut self, id: &RoleId) -> DomainResult<()> {
        if self.members.iter().any(|m| m.role_ids().contains(id)) {
            return Err(DomainError::ValidationError("使用中のロールは削除できません".to_string()));
        }
        self.roles.retain(|r| r.id() != id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{company::values::*, entity::EntityId};

    fn create_test_org() -> Organization {
        let id = OrganizationId::generate();
        let code = CompanyCode::new("C001").unwrap();
        let name = CompanyName::new("テスト株式会社").unwrap();
        Organization::new(id, code, name)
    }

    fn create_test_department(parent_id: Option<&DepartmentId>) -> Department {
        Department::new(
            DepartmentId::generate(),
            DepartmentCode::new("D001").unwrap(),
            DepartmentName::new("経理部").unwrap(),
            parent_id.cloned(),
        )
    }

    fn create_test_position() -> Position {
        Position::new(
            PositionId::generate(),
            PositionName::new("課長").unwrap(),
            PositionLevel::new(4).unwrap(),
        )
    }

    fn create_test_role() -> Role {
        let mut role = Role::new(RoleId::generate(), RoleName::new("経理担当").unwrap());
        role.grant(Permission::new("journal_entry:create").unwrap());
        role
    }

    #[test]
    fn test_organization_creation() {
        let org = create_test_org();
        assert_eq!(org.company().code().value(), "C001");
        assert!(org.company().is_active());
        assert!(org.departments().is_empty());
        assert!(org.positions().is_empty());
        assert!(org.members().is_empty());
        assert!(org.roles().is_empty());
    }

    #[test]
    fn test_add_department() {
        let mut org = create_test_org();
        let dept = create_test_department(None);
        assert!(org.add_department(dept).is_ok());
        assert_eq!(org.departments().len(), 1);
    }

    #[test]
    fn test_add_duplicate_department_code_fails() {
        let mut org = create_test_org();
        let dept1 = create_test_department(None);
        org.add_department(dept1).unwrap();

        let dept2 = Department::new(
            DepartmentId::generate(),
            DepartmentCode::new("D001").unwrap(),
            DepartmentName::new("別部署").unwrap(),
            None,
        );
        assert!(org.add_department(dept2).is_err());
    }

    #[test]
    fn test_department_hierarchy() {
        let mut org = create_test_org();
        let parent = create_test_department(None);
        let parent_id = parent.id().clone();
        org.add_department(parent).unwrap();

        let child = Department::new(
            DepartmentId::generate(),
            DepartmentCode::new("D002").unwrap(),
            DepartmentName::new("経理課").unwrap(),
            Some(parent_id.clone()),
        );
        org.add_department(child).unwrap();

        assert_eq!(org.root_departments().len(), 1);
        assert_eq!(org.child_departments(&parent_id).len(), 1);
    }

    #[test]
    fn test_add_member_with_validation() {
        let mut org = create_test_org();
        let dept = create_test_department(None);
        let dept_id = dept.id().clone();
        org.add_department(dept).unwrap();

        let pos = create_test_position();
        let pos_id = pos.id().clone();
        org.add_position(pos).unwrap();

        let role = create_test_role();
        let role_id = role.id().clone();
        org.add_role(role).unwrap();

        let member = Member::new(
            MemberId::generate(),
            MemberName::new("山田太郎").unwrap(),
            Email::new("yamada@example.com").unwrap(),
            dept_id,
            pos_id,
            vec![role_id],
        );
        assert!(org.add_member(member).is_ok());
        assert_eq!(org.members().len(), 1);
    }

    #[test]
    fn test_add_member_with_invalid_department_fails() {
        let mut org = create_test_org();
        let pos = create_test_position();
        let pos_id = pos.id().clone();
        org.add_position(pos).unwrap();

        let role = create_test_role();
        let role_id = role.id().clone();
        org.add_role(role).unwrap();

        let member = Member::new(
            MemberId::generate(),
            MemberName::new("山田太郎").unwrap(),
            Email::new("yamada@example.com").unwrap(),
            DepartmentId::new("nonexistent"),
            pos_id,
            vec![role_id],
        );
        assert!(org.add_member(member).is_err());
    }

    #[test]
    fn test_remove_department_with_members_fails() {
        let mut org = create_test_org();
        let dept = create_test_department(None);
        let dept_id = dept.id().clone();
        org.add_department(dept).unwrap();

        let pos = create_test_position();
        let pos_id = pos.id().clone();
        org.add_position(pos).unwrap();

        let role = create_test_role();
        let role_id = role.id().clone();
        org.add_role(role).unwrap();

        let member = Member::new(
            MemberId::generate(),
            MemberName::new("山田太郎").unwrap(),
            Email::new("yamada@example.com").unwrap(),
            dept_id.clone(),
            pos_id,
            vec![role_id],
        );
        org.add_member(member).unwrap();

        assert!(org.remove_department(&dept_id).is_err());
    }

    #[test]
    fn test_remove_role_in_use_fails() {
        let mut org = create_test_org();
        let dept = create_test_department(None);
        let dept_id = dept.id().clone();
        org.add_department(dept).unwrap();

        let pos = create_test_position();
        let pos_id = pos.id().clone();
        org.add_position(pos).unwrap();

        let role = create_test_role();
        let role_id = role.id().clone();
        org.add_role(role).unwrap();

        let member = Member::new(
            MemberId::generate(),
            MemberName::new("山田太郎").unwrap(),
            Email::new("yamada@example.com").unwrap(),
            dept_id,
            pos_id,
            vec![role_id.clone()],
        );
        org.add_member(member).unwrap();

        assert!(org.remove_role(&role_id).is_err());
    }
}
