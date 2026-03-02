// 固定資産台帳モジュール
// IFRS準拠（IAS 16, IAS 38, IFRS 16）

pub mod entities;
pub mod events;
pub mod services;
pub mod values;

pub use entities::{Component, FixedAsset};
pub use events::{FixedAssetEvent, FixedAssetEventType};
pub use services::FixedAssetDomainService;
pub use values::{
    AssetCategory, AssetStatus, ComponentId, DepreciationMethod, FixedAssetId, MeasurementModel,
    UsefulLife,
};
