// Read-side infrastructure
// Projection基盤（DB、trait、builder、worker）

pub mod builder;
pub mod db;
pub mod traits;
pub mod worker;

pub use builder::ProjectionBuilderImpl;
pub use db::ProjectionDb;
pub use traits::{Apply, ProjectionStrategy, ToReadModel};
pub use worker::ProjectionWorker;
