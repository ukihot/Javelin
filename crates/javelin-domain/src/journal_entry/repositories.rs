// JournalEntry Repository - 仕訳伝票リポジトリ

use super::entities::JournalEntry;
use crate::common::RepositoryBase;

/// 仕訳エントリリポジトリトレイト
///
/// JournalEntry集約を扱う専用リポジトリ。
/// RepositoryBaseを継承し、集約のロード/保存機能を提供。
///
/// # 責務
/// - JournalEntry集約の保存
/// - JournalEntry集約のロード
///
/// # インフラ層での実装
/// - save: 集約からイベントを生成し、EventStoreに保存
/// - load: EventStoreからイベントを取得し、集約を復元
#[allow(async_fn_in_trait)]
pub trait JournalEntryRepository: RepositoryBase<JournalEntry> + Send + Sync {
    // 必要に応じて仕訳エントリ固有のメソッドを追加可能
}
