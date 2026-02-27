// QueryService - Query処理
// 責務: Projection検索
// 禁止: Repository利用

pub mod account_master_query_service;
pub mod application_settings_master_query_service;
pub mod batch_history_query_service;
pub mod company_master_query_service;
pub mod journal_entry_finder;
pub mod journal_entry_search_query_service;
pub mod ledger_query_service;
pub mod subsidiary_account_master_query_service;

use crate::error::ApplicationResult;

/// QueryServiceトレイト（async対応）
#[allow(async_fn_in_trait)]
pub trait QueryService: Send + Sync {
    type Query: Send;
    type Result: Send;

    async fn query(&self, query: Self::Query) -> ApplicationResult<Self::Result>;
}

// Re-export for convenience
pub use account_master_query_service::*;
pub use application_settings_master_query_service::*;
pub use batch_history_query_service::*;
pub use company_master_query_service::*;
pub use journal_entry_finder::*;
pub use journal_entry_search_query_service::*;
pub use ledger_query_service::*;
pub use subsidiary_account_master_query_service::*;
