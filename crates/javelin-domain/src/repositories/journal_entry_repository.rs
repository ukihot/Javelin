// JournalEntryRepository - 仕訳エントリ専用リポジトリ

use super::RepositoryBase;
use crate::financial_close::journal_entry::events::JournalEntryEvent;

/// 仕訳エントリリポジトリトレイト
///
/// JournalEntryEventを扱う専用リポジトリ。
/// RepositoryBaseを継承し、イベントソーシングの基本機能を提供。
#[allow(async_fn_in_trait)]
pub trait JournalEntryRepository: RepositoryBase<Event = JournalEntryEvent> + Send + Sync {
    // 必要に応じて仕訳エントリ固有のメソッドを追加可能
}
