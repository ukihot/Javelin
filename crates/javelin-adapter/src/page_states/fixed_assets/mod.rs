// Fixed Assets Menu and related pages

pub mod asset_detail;
pub mod asset_registration;
pub mod depreciation_execution;
pub mod depreciation_result;
pub mod fixed_asset_list;
pub mod ifrs_valuation_execution;
pub mod lease_contract_detail;
pub mod lease_contract_list;
pub mod lease_schedule;
pub mod menu;
pub mod rou_asset_list;

pub use asset_detail::AssetDetailPageState;
pub use asset_registration::AssetRegistrationPageState;
pub use depreciation_execution::DepreciationExecutionPageState;
pub use depreciation_result::DepreciationResultPageState;
pub use fixed_asset_list::FixedAssetListPageState;
pub use ifrs_valuation_execution::IfrsValuationExecutionPageState;
pub use lease_contract_detail::LeaseContractDetailPageState;
pub use lease_contract_list::LeaseContractListPageState;
pub use lease_schedule::LeaseSchedulePageState;
pub use menu::FixedAssetsMenuPageState;
pub use rou_asset_list::RouAssetListPageState;
