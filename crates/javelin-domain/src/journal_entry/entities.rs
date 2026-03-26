// JournalEntry Entities - 仕訳伝票エンティティ
//
// 公開範囲：
// - journal_entry::JournalEntry（ルート集約、公開）
// - journal_entry_id::JournalEntryId（ID、公開）
// - journal_entry_line::JournalEntryLine（子エンティティ、公開だが new は pub(super)）

pub mod journal_entry;
pub mod journal_entry_id;
pub mod journal_entry_line;

// Re-exports: すべて公開（ただしコンストラクタは集約内でのみ構築可能）
pub use journal_entry::*;
pub use journal_entry_id::*;
pub use journal_entry_line::*;
