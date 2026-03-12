// JournalEntry Aggregate - 仕訳伝票集約
//
// 仕訳伝票のライフサイクルを管理する集約。
// Event Sourcingパターンを使用し、すべての状態変更をイベントとして記録する。

pub mod entities;
pub mod events;
pub mod repositories;
pub mod values;

// Re-exports
pub use entities::*;
pub use events::*;
pub use repositories::*;
pub use values::*;
