// Organization Domain Services - 組織体ドメインサービス
//
// 複数エンティティにまたがるドメインロジック。

use crate::{
    company::{
        entities::{Member, Organization},
        values::{MemberId, Permission},
    },
    entity::Entity,
    error::{DomainError, DomainResult},
};

/// 組織体ドメインサービス
pub struct OrganizationDomainService;

impl OrganizationDomainService {
    /// メンバーが指定権限を保有しているか検証する
    ///
    /// メンバーに割り当てられた全ロールの権限を集約して判定する。
    pub fn has_permission(
        organization: &Organization,
        member_id: &MemberId,
        permission: &Permission,
    ) -> DomainResult<bool> {
        let Some(member) = organization.find_member(member_id) else {
            return Err(DomainError::EntityNotFound(format!("メンバー '{}'", member_id)));
        };

        if !member.is_active() {
            return Ok(false);
        }

        let has = member.role_ids().iter().any(|role_id| {
            organization
                .find_role(role_id)
                .is_some_and(|role| role.has_permission(permission))
        });

        Ok(has)
    }

    /// メンバーが保有する全権限を収集する
    pub fn collect_permissions(
        organization: &Organization,
        member_id: &MemberId,
    ) -> DomainResult<Vec<Permission>> {
        let Some(member) = organization.find_member(member_id) else {
            return Err(DomainError::EntityNotFound(format!("メンバー '{}'", member_id)));
        };

        let mut permissions = Vec::new();
        for role_id in member.role_ids() {
            if let Some(role) = organization.find_role(role_id) {
                for perm in role.permissions() {
                    if !permissions.contains(perm) {
                        permissions.push(perm.clone());
                    }
                }
            }
        }
        Ok(permissions)
    }

    /// 指定権限を持つメンバーを検索する
    pub fn find_members_with_permission<'a>(
        organization: &'a Organization,
        permission: &Permission,
    ) -> Vec<&'a Member> {
        organization
            .members()
            .iter()
            .filter(|member| {
                member.is_active()
                    && member.role_ids().iter().any(|role_id| {
                        organization
                            .find_role(role_id)
                            .is_some_and(|role| role.has_permission(permission))
                    })
            })
            .collect()
    }

    /// 部署のメンバー数を取得（再帰的に子部署を含む）
    pub fn count_members_recursive(
        organization: &Organization,
        department_id: &crate::company::values::DepartmentId,
    ) -> usize {
        let direct = organization.members_in_department(department_id).len();
        let children: usize = organization
            .child_departments(department_id)
            .iter()
            .map(|child| Self::count_members_recursive(organization, child.id()))
            .sum();
        direct + children
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::company::{entities::*, values::*};

    fn setup_org() -> (Organization, MemberId, RoleId) {
        let mut org = Organization::new(
            OrganizationId::generate(),
            CompanyCode::new("C001").unwrap(),
            CompanyName::new("テスト株式会社").unwrap(),
        );

        let dept = Department::new(
            DepartmentId::generate(),
            DepartmentCode::new("FIN").unwrap(),
            DepartmentName::new("財務部").unwrap(),
            None,
        );
        let dept_id = dept.id().clone();
        org.add_department(dept).unwrap();

        let pos = Position::new(
            PositionId::generate(),
            PositionName::new("課長").unwrap(),
            PositionLevel::new(4).unwrap(),
        );
        let pos_id = pos.id().clone();
        org.add_position(pos).unwrap();

        let mut role = Role::new(RoleId::generate(), RoleName::new("経理担当").unwrap());
        role.grant(Permission::new("journal_entry:create").unwrap());
        role.grant(Permission::new("journal_entry:approve").unwrap());
        let role_id = role.id().clone();
        org.add_role(role).unwrap();

        let member_id = MemberId::generate();
        let member = Member::new(
            member_id.clone(),
            MemberName::new("山田太郎").unwrap(),
            Email::new("yamada@example.com").unwrap(),
            dept_id,
            pos_id,
            vec![role_id.clone()],
        );
        org.add_member(member).unwrap();

        (org, member_id, role_id)
    }

    #[test]
    fn test_has_permission() {
        let (org, member_id, _) = setup_org();

        let create_perm = Permission::new("journal_entry:create").unwrap();
        assert!(OrganizationDomainService::has_permission(&org, &member_id, &create_perm).unwrap());

        let closing_perm = Permission::new("closing:execute").unwrap();
        assert!(
            !OrganizationDomainService::has_permission(&org, &member_id, &closing_perm).unwrap()
        );
    }

    #[test]
    fn test_inactive_member_has_no_permission() {
        let (mut org, member_id, _) = setup_org();
        org.deactivate_member(&member_id).unwrap();

        let perm = Permission::new("journal_entry:create").unwrap();
        assert!(!OrganizationDomainService::has_permission(&org, &member_id, &perm).unwrap());
    }

    #[test]
    fn test_collect_permissions() {
        let (org, member_id, _) = setup_org();
        let permissions = OrganizationDomainService::collect_permissions(&org, &member_id).unwrap();
        assert_eq!(permissions.len(), 2);
    }

    #[test]
    fn test_find_members_with_permission() {
        let (org, _, _) = setup_org();
        let perm = Permission::new("journal_entry:create").unwrap();
        let members = OrganizationDomainService::find_members_with_permission(&org, &perm);
        assert_eq!(members.len(), 1);
    }
}
