// JournalEntry Aggregate - 仕訳伝票集約
//
// 仕訳伝票のライフサイクルを管理する集約。
// Event Sourcingパターンを使用し、すべての状態変更をイベントとして記録する。

pub mod domain_events;
pub mod domain_services;
pub mod entities;
pub mod repositories;
pub mod values;

// Re-exports
pub use domain_events::*;
pub use entities::*;
pub use repositories::*;
pub use values::*;
