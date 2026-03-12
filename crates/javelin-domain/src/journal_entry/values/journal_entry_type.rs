// JournalEntryType - 仕訳行為区分値オブジェクト

use crate::{error::DomainResult, value_object::ValueObject};

/// 仕訳行為区分
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JournalEntryType {
    /// 新規起票仕訳
    NewEntry,
    /// 取消仕訳
    Cancellation,
    /// 反対仕訳
    Reversal,
    /// 追加仕訳
    Additional,
    /// 再分類仕訳
    Reclassification,
    /// 洗替仕訳
    Replacement,
}

impl JournalEntryType {
    pub fn requires_reference(&self) -> bool {
        !matches!(self, JournalEntryType::NewEntry)
    }

    pub fn display_name(&self) -> &str {
        match self {
            JournalEntryType::NewEntry => "新規起票",
            JournalEntryType::Cancellation => "取消",
            JournalEntryType::Reversal => "反対",
            JournalEntryType::Additional => "追加",
            JournalEntryType::Reclassification => "再分類",
            JournalEntryType::Replacement => "洗替",
        }
    }
}

impl ValueObject for JournalEntryType {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}
