// Batch History read-side (バッチ履歴読み取り側)
// バッチ実行履歴に関する全ての読み取り機能

pub mod projection;
pub mod query_service;

pub use projection::{
    BatchHistoryProjection, BatchHistoryProjectionStrategy, BatchHistoryReadModel,
};
pub use query_service::BatchHistoryQueryServiceImpl;
