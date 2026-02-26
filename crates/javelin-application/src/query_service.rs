// QueryService - Query処理
// 責務: Projection検索
// 禁止: Repository利用

pub mod batch_history_query_service;
pub mod journal_entry_finder;
pub mod journal_entry_search_query_service;
pub mod ledger_query_service;
pub mod master_data_loader;

use crate::error::ApplicationResult;

/// QueryServiceトレイト（async対応）
#[allow(async_fn_in_trait)]
pub trait QueryService: Send + Sync {
    type Query: Send;
    type Result: Send;

    async fn query(&self, query: Self::Query) -> ApplicationResult<Self::Result>;
}

// Re-export for convenience
pub use batch_history_query_service::*;
pub use journal_entry_finder::*;
pub use journal_entry_search_query_service::*;
pub use ledger_query_service::*;
pub use master_data_loader::*;
