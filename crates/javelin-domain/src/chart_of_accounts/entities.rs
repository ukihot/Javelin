// ChartOfAccounts Entities - 勘定科目表エンティティ

pub mod account_master;
pub mod subsidiary_account_master;

// Re-exports
pub use account_master::*;
pub use subsidiary_account_master::*;

// 将来的には以下のような構造に移行：
// - ChartOfAccounts (集約ルート)
// - Account (勘定科目エンティティ)
// - SubAccount (補助科目エンティティ)
