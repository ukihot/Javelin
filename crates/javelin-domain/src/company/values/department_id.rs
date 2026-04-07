// DepartmentId - 部署ID値オブジェクト

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::entity::EntityId;

/// 部署ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DepartmentId(String);

impl DepartmentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl EntityId for DepartmentId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DepartmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
