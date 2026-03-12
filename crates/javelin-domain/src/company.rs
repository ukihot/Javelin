// Company Aggregate - 会社集約
//
// 会社情報を管理する集約。
// 旧company_masterを統合。

pub mod entities;
pub mod repositories;
pub mod values;

// Re-exports
pub use entities::*;
pub use repositories::*;
pub use values::*;
