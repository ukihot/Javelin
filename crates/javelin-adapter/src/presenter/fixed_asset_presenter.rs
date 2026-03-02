// FixedAssetPresenter実装
// 固定資産・リース関連の出力を整形してビューに渡す

use tokio::sync::mpsc;

/// 固定資産ViewModel
#[derive(Debug, Clone)]
pub struct FixedAssetViewModel {
    pub asset_id: String,
    pub asset_name: String,
    pub acquisition_date: String,
    pub acquisition_cost: f64,
    pub accumulated_depreciation: f64,
    pub carrying_amount: f64,
    pub useful_life: u32,
}

/// 減価償却結果ViewModel
#[derive(Debug, Clone)]
pub struct DepreciationResultViewModel {
    pub asset_id: String,
    pub asset_name: String,
    pub depreciation_method: String,
    pub depreciation_amount: f64,
    pub accumulated_depreciation: f64,
    pub carrying_amount: f64,
}

/// リース契約ViewModel
#[derive(Debug, Clone)]
pub struct LeaseContractViewModel {
    pub contract_id: String,
    pub lessor: String,
    pub asset_name: String,
    pub start_date: String,
    pub end_date: String,
    pub monthly_payment: f64,
    pub total_liability: f64,
}

/// リーススケジュールViewModel
#[derive(Debug, Clone)]
pub struct LeaseScheduleViewModel {
    pub payment_date: String,
    pub payment_amount: f64,
    pub principal: f64,
    pub interest: f64,
    pub remaining_balance: f64,
}

/// 使用権資産ViewModel
#[derive(Debug, Clone)]
pub struct RouAssetViewModel {
    pub asset_id: String,
    pub lease_contract_id: String,
    pub asset_name: String,
    pub initial_cost: f64,
    pub accumulated_depreciation: f64,
    pub carrying_amount: f64,
    pub remaining_term: u32,
}

/// 固定資産Presenter
pub struct FixedAssetPresenter {
    fixed_asset_sender: mpsc::UnboundedSender<Vec<FixedAssetViewModel>>,
    depreciation_result_sender: mpsc::UnboundedSender<Vec<DepreciationResultViewModel>>,
    lease_contract_sender: mpsc::UnboundedSender<Vec<LeaseContractViewModel>>,
    lease_schedule_sender: mpsc::UnboundedSender<Vec<LeaseScheduleViewModel>>,
    rou_asset_sender: mpsc::UnboundedSender<Vec<RouAssetViewModel>>,
}

impl FixedAssetPresenter {
    pub fn new(
        fixed_asset_sender: mpsc::UnboundedSender<Vec<FixedAssetViewModel>>,
        depreciation_result_sender: mpsc::UnboundedSender<Vec<DepreciationResultViewModel>>,
        lease_contract_sender: mpsc::UnboundedSender<Vec<LeaseContractViewModel>>,
        lease_schedule_sender: mpsc::UnboundedSender<Vec<LeaseScheduleViewModel>>,
        rou_asset_sender: mpsc::UnboundedSender<Vec<RouAssetViewModel>>,
    ) -> Self {
        Self {
            fixed_asset_sender,
            depreciation_result_sender,
            lease_contract_sender,
            lease_schedule_sender,
            rou_asset_sender,
        }
    }

    /// チャネルを作成
    #[allow(clippy::type_complexity)]
    pub fn create_channels() -> (
        mpsc::UnboundedSender<Vec<FixedAssetViewModel>>,
        mpsc::UnboundedReceiver<Vec<FixedAssetViewModel>>,
        mpsc::UnboundedSender<Vec<DepreciationResultViewModel>>,
        mpsc::UnboundedReceiver<Vec<DepreciationResultViewModel>>,
        mpsc::UnboundedSender<Vec<LeaseContractViewModel>>,
        mpsc::UnboundedReceiver<Vec<LeaseContractViewModel>>,
        mpsc::UnboundedSender<Vec<LeaseScheduleViewModel>>,
        mpsc::UnboundedReceiver<Vec<LeaseScheduleViewModel>>,
        mpsc::UnboundedSender<Vec<RouAssetViewModel>>,
        mpsc::UnboundedReceiver<Vec<RouAssetViewModel>>,
    ) {
        let (fixed_asset_tx, fixed_asset_rx) = mpsc::unbounded_channel();
        let (depreciation_result_tx, depreciation_result_rx) = mpsc::unbounded_channel();
        let (lease_contract_tx, lease_contract_rx) = mpsc::unbounded_channel();
        let (lease_schedule_tx, lease_schedule_rx) = mpsc::unbounded_channel();
        let (rou_asset_tx, rou_asset_rx) = mpsc::unbounded_channel();

        (
            fixed_asset_tx,
            fixed_asset_rx,
            depreciation_result_tx,
            depreciation_result_rx,
            lease_contract_tx,
            lease_contract_rx,
            lease_schedule_tx,
            lease_schedule_rx,
            rou_asset_tx,
            rou_asset_rx,
        )
    }

    pub fn present_fixed_assets(&self, assets: Vec<FixedAssetViewModel>) {
        let _ = self.fixed_asset_sender.send(assets);
    }

    pub fn present_depreciation_results(&self, results: Vec<DepreciationResultViewModel>) {
        let _ = self.depreciation_result_sender.send(results);
    }

    pub fn present_lease_contracts(&self, contracts: Vec<LeaseContractViewModel>) {
        let _ = self.lease_contract_sender.send(contracts);
    }

    pub fn present_lease_schedule(&self, schedule: Vec<LeaseScheduleViewModel>) {
        let _ = self.lease_schedule_sender.send(schedule);
    }

    pub fn present_rou_assets(&self, assets: Vec<RouAssetViewModel>) {
        let _ = self.rou_asset_sender.send(assets);
    }
}
