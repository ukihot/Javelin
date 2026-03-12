// ChartOfAccounts Aggregate - 勘定科目表集約
//
// 勘定科目表（Chart of Accounts）を管理する集約。
// 旧account_master, subsidiary_account_masterを統合。

pub mod entities;
pub mod repositories;
pub mod values;

// Re-exports
pub use entities::*;
pub use repositories::*;
pub use values::*;
