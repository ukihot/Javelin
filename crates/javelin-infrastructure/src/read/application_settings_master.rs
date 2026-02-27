// ApplicationSettingsMaster read-side (アプリケーション設定マスタ読み取り側)

pub mod projection;
pub mod query_service;

pub use projection::ApplicationSettingsMasterProjection;
pub use query_service::ApplicationSettingsMasterQueryServiceImpl;
