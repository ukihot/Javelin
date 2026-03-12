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

// 具体的な型エイリアス
use crate::read::projectors::{
    AccountMasterProjector, JournalEntryProjector, LedgerProjector, TrialBalanceProjector,
};

/// 具体的なProjectionBuilder型
pub type ConcreteProjectionBuilder = ProjectionBuilderImpl<
    JournalEntryProjector,
    AccountMasterProjector,
    LedgerProjector,
    TrialBalanceProjector,
>;
