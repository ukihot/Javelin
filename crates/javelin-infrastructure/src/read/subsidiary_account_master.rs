// SubsidiaryAccountMaster read-side (補助科目マスタ読み取り側)

pub mod projection;
pub mod query_service;

pub use projection::SubsidiaryAccountMasterProjection;
pub use query_service::SubsidiaryAccountMasterQueryServiceImpl;
