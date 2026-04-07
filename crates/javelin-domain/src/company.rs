// Company Aggregate - 組織体集約
//
// 組織体制（会社・部署・役職・メンバー・権限）を管理する集約。
// 認証認可の基盤となる組織マスタデータを統合管理する。

pub mod domain_services;
pub mod entities;
pub mod repositories;
pub mod values;

// Re-exports
pub use domain_services::*;
pub use entities::*;
pub use repositories::*;
pub use values::*;
