// RepositoryBase - イベントソーシングにおける基底リポジトリトレイト
// 全てのリポジトリはイベントを永続化する責務を持つ
// 必須操作: append / loadStream
// 禁止: 詳細なQuery機能

use crate::{error::DomainResult, event::DomainEvent};

/// RepositoryBase - 全リポジトリの基底トレイト
///
/// イベントソーシングアーキテクチャにおいて、全てのリポジトリは
/// イベントを永続化する責務を持つ。このトレイトはその共通インターフェースを定義する。
#[allow(async_fn_in_trait)]
pub trait RepositoryBase: Send + Sync {
    type Event: DomainEvent;

    /// イベントを追記
    async fn append(&self, event: Self::Event) -> DomainResult<()>;

    /// 複数イベントを一括追記
    ///
    /// # Arguments
    /// * `aggregate_id` - 集約ID
    /// * `events` - 保存するドメインイベントのリスト
    ///
    /// # Returns
    /// 最後に保存されたイベントのシーケンス番号
    async fn append_events<T>(&self, aggregate_id: &str, events: Vec<T>) -> DomainResult<u64>
    where
        T: serde::Serialize + Send + 'static;

    /// 指定された集約IDのイベントストリームを取得
    ///
    /// # Arguments
    /// * `aggregate_id` - 集約ID
    ///
    /// # Returns
    /// イベントのベクタ（シーケンス順）
    async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>>;

    /// 指定されたシーケンス番号以降の全イベントを取得
    ///
    /// # Arguments
    /// * `from_sequence` - 開始シーケンス番号
    ///
    /// # Returns
    /// イベントのベクタ（シーケンス順）
    async fn get_all_events(&self, from_sequence: u64) -> DomainResult<Vec<serde_json::Value>>;

    /// 最新シーケンス番号を取得
    async fn get_latest_sequence(&self) -> DomainResult<u64>;
}

// expose mocks for all crates; they are lightweight and useful in integration tests
pub use mock::{MockClosingRepository, MockJournalEntryRepository};

// テスト用のモック実装
// the mock module contains automock implementations used by tests and consumers
pub mod mock {
    use mockall::mock;

    use super::*;

    // JournalEntryEvent用のモック
    mock! {
        pub JournalEntryRepository {}
        #[allow(async_fn_in_trait)]
        impl crate::repositories::journal_entry_repository::JournalEntryRepository for JournalEntryRepository {}

        #[allow(async_fn_in_trait)]
        impl RepositoryBase for JournalEntryRepository {
            type Event = crate::financial_close::journal_entry::events::JournalEntryEvent;

            async fn append(&self, event: <Self as RepositoryBase>::Event) -> DomainResult<()>;
            async fn append_events<T>(&self, aggregate_id: &str, events: Vec<T>) -> DomainResult<u64>
            where
                T: serde::Serialize + Send + 'static;
            async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>>;
            async fn get_all_events(&self, from_sequence: u64) -> DomainResult<Vec<serde_json::Value>>;
            async fn get_latest_sequence(&self) -> DomainResult<u64>;
        }
    }

    // ClosingEvent用のモック
    mock! {
        pub ClosingRepository {}
        #[allow(async_fn_in_trait)]
        impl crate::repositories::closing_repository::ClosingRepository for ClosingRepository {}

        #[allow(async_fn_in_trait)]
        impl RepositoryBase for ClosingRepository {
            type Event = crate::financial_close::closing_events::ClosingEvent;

            async fn append(&self, event: <Self as RepositoryBase>::Event) -> DomainResult<()>;
            async fn append_events<T>(&self, aggregate_id: &str, events: Vec<T>) -> DomainResult<u64>
            where
                T: serde::Serialize + Send + 'static;
            async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>>;
            async fn get_all_events(&self, from_sequence: u64) -> DomainResult<Vec<serde_json::Value>>;
            async fn get_latest_sequence(&self) -> DomainResult<u64>;
        }
    }
}
