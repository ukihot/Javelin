// Role Entity - ロールエンティティ
//
// 権限セットを定義するロール。メンバーに複数割り当て可能。
// ロールベースアクセス制御（RBAC）の基盤。

use crate::{
    company::values::{Permission, RoleId, RoleName},
    entity::Entity,
};

/// ロール（権限セット）
#[derive(Debug, Clone)]
pub struct Role {
    id: RoleId,
    name: RoleName,
    permissions: Vec<Permission>,
}

impl Entity for Role {
    type Id = RoleId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Role {
    pub fn new(id: RoleId, name: RoleName) -> Self {
        Self { id, name, permissions: Vec::new() }
    }

    pub fn name(&self) -> &RoleName {
        &self.name
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    /// 権限を付与
    pub fn grant(&mut self, permission: Permission) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    /// 権限を剥奪
    pub fn revoke(&mut self, permission: &Permission) {
        self.permissions.retain(|p| p != permission);
    }

    /// 指定された権限を保有しているか
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.iter().any(|p| p.covers(permission))
    }

    pub fn rename(&mut self, name: RoleName) {
        self.name = name;
    }
}

#[cfg(test)]
mod tests {
    use crate::company::values::*;

    use super::*;

    fn create_test_role() -> Role {
        Role::new(
            RoleId::generate(),
            RoleName::new("経理担当").unwrap(),
        )
    }

    #[test]
    fn test_role_creation() {
        let role = create_test_role();
        assert_eq!(role.name().value(), "経理担当");
        assert!(role.permissions().is_empty());
    }

    #[test]
    fn test_role_permission_management() {
        let mut role = create_test_role();
        let perm = Permission::new("journal_entry:create").unwrap();

        role.grant(perm.clone());
        assert_eq!(role.permissions().len(), 1);
        assert!(role.has_permission(&perm));

        // 重複付与は無視
        role.grant(perm.clone());
        assert_eq!(role.permissions().len(), 1);

        role.revoke(&perm);
        assert!(role.permissions().is_empty());
        assert!(!role.has_permission(&perm));
    }

    #[test]
    fn test_wildcard_permission() {
        let mut role = create_test_role();
        let wildcard = Permission::new("journal_entry:*").unwrap();
        role.grant(wildcard);

        let create = Permission::new("journal_entry:create").unwrap();
        let approve = Permission::new("journal_entry:approve").unwrap();
        let other = Permission::new("closing:execute").unwrap();

        assert!(role.has_permission(&create));
        assert!(role.has_permission(&approve));
        assert!(!role.has_permission(&other));
    }
}
