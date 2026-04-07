// PositionId - 役職ID値オブジェクト

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::entity::EntityId;

/// 役職ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PositionId(String);

impl PositionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl EntityId for PositionId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PositionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
