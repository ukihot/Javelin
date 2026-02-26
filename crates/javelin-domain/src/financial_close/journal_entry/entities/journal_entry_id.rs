// 仕訳ID

use crate::entity::EntityId;

/// 仕訳ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntryId(String);

impl EntityId for JournalEntryId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl JournalEntryId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}
