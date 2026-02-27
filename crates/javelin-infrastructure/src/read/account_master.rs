// AccountMaster read-side (勘定科目マスタ読み取り側)

pub mod projection;
pub mod query_service;

pub use projection::AccountMasterProjection;
pub use query_service::AccountMasterQueryServiceImpl;
