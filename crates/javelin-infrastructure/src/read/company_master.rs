// CompanyMaster read-side (会社マスタ読み取り側)

pub mod projection;
pub mod query_service;

pub use projection::CompanyMasterProjection;
pub use query_service::CompanyMasterQueryServiceImpl;
