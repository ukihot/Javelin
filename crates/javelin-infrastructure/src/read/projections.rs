// Projection definitions and ReadModels
// Projection定義とReadModel（イベントから派生する読み取り最適化構造）

pub mod batch_history_projection;
pub mod journal_entry_projection;
pub mod journal_entry_projection_worker;
pub mod journal_entry_search_projection;
pub mod journal_entry_search_read_model;
pub mod ledger_projection;
pub mod projection_builder_impl;
pub mod projection_db;
pub mod projection_trait;
pub mod projection_worker;

pub use batch_history_projection::{
    BatchHistoryProjection, BatchHistoryProjectionStrategy, BatchHistoryReadModel,
};
pub use journal_entry_projection::JournalEntryProjection;
pub use journal_entry_projection_worker::JournalEntryProjectionWorker;
pub use journal_entry_search_projection::JournalEntrySearchProjection;
pub use journal_entry_search_read_model::{JournalEntryLineReadModel, JournalEntrySearchReadModel};
pub use ledger_projection::{LedgerEntryReadModel, LedgerProjection};
pub use projection_builder_impl::ProjectionBuilderImpl;
pub use projection_db::ProjectionDb;
pub use projection_trait::{Apply, ProjectionStrategy, ToReadModel};
pub use projection_worker::ProjectionWorker;
