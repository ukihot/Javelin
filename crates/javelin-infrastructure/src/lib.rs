// Infrastructure Layer - 永続化 / 外部技術実装
// 依存方向: → Domain
// Pure CQRS: write/ (Command) | read/ (Query) | shared/

pub mod read;
pub mod shared;
pub mod write;

// 内部のみ: read/write 配下および tests が crate::* を参照するため
pub(crate) use read::{
    projections::{projection_db, projection_trait},
    queries,
};
pub(crate) use shared::{error, storage_metrics, types};
pub(crate) use write::{
    event_store,
    event_store::{EventStore, event_stream},
    repositories,
};

// Test modules
#[cfg(test)]
#[path = "tests/event_store_property_tests.rs"]
mod event_store_property_tests;
#[cfg(test)]
#[path = "tests/event_store_unit_tests.rs"]
mod event_store_unit_tests;
#[cfg(test)]
#[path = "tests/ledger_query_service_property_tests.rs"]
mod ledger_query_service_property_tests;
#[cfg(test)]
#[path = "tests/projection_builder_property_tests.rs"]
mod projection_builder_property_tests;
