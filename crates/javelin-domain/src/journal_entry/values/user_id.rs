// UserId - ユーザーID値オブジェクト

use crate::entity::EntityId;

/// ユーザーID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(String);

impl EntityId for UserId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl UserId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}
