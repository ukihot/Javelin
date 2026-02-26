// RepositoryTrait - イベントソーシングにおける基底リポジトリ抽象
// 全てのリポジトリはRepositoryBaseを実装し、イベントを永続化する
// 必須操作: append / loadStream
// 禁止: 詳細なQuery機能
pub mod account_master_repository;
pub mod application_settings_repository;
pub mod closing_repository;
pub mod company_master_repository;
pub mod journal_entry_repository;
pub mod repository_base;
pub mod subsidiary_account_master_repository;
pub mod system_master_repository;

pub use account_master_repository::*;
pub use application_settings_repository::*;
pub use closing_repository::*;
pub use company_master_repository::*;
pub use journal_entry_repository::*;
pub use repository_base::*;
// テスト用のモックを再エクスポート
#[cfg(test)]
pub use repository_base::{MockClosingRepository, MockJournalEntryRepository};
pub use subsidiary_account_master_repository::*;
pub use system_master_repository::*;
