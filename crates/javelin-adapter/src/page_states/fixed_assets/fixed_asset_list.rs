// FixedAssetListPageState - 固定資産一覧画面
// 責務: 固定資産台帳の一覧表示

use std::sync::Arc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::FixedAssetPresenter,
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// 固定資産項目ViewModel
#[derive(Debug, Clone)]
pub struct FixedAssetItemViewModel {
    pub asset_id: String,
    pub asset_name: String,
    pub acquisition_date: String,
    pub acquisition_cost: String,
    pub accumulated_depreciation: String,
    pub carrying_amount: String,
    pub useful_life: String,
}

impl MasterListItem for FixedAssetItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec![
            "資産ID",
            "資産名",
            "取得日",
            "取得原価",
            "減価償却累計額",
            "帳簿価額",
            "耐用年数",
        ]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Min(20),
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Length(18),
            Constraint::Length(15),
            Constraint::Length(10),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.asset_id.clone(),
            self.asset_name.clone(),
            self.acquisition_date.clone(),
            self.acquisition_cost.clone(),
            self.accumulated_depreciation.clone(),
            self.carrying_amount.clone(),
            self.useful_life.clone(),
        ]
    }
}

/// 固定資産一覧画面
pub struct FixedAssetListPageState {
    page_id: Uuid,
    template: MasterListTemplate<FixedAssetItemViewModel>,
    presenter_registry: Arc<PresenterRegistry>,
    fixed_asset_rx:
        tokio::sync::mpsc::UnboundedReceiver<Vec<crate::presenter::FixedAssetViewModel>>,
}

impl FixedAssetListPageState {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        let page_id = Uuid::new_v4();
        let template = MasterListTemplate::new("固定資産一覧");

        let (
            fixed_asset_tx,
            fixed_asset_rx,
            depreciation_result_tx,
            _depreciation_result_rx,
            lease_contract_tx,
            _lease_contract_rx,
            lease_schedule_tx,
            _lease_schedule_rx,
            rou_asset_tx,
            _rou_asset_rx,
        ) = FixedAssetPresenter::create_channels();

        let presenter = Arc::new(FixedAssetPresenter::new(
            fixed_asset_tx,
            depreciation_result_tx,
            lease_contract_tx,
            lease_schedule_tx,
            rou_asset_tx,
        ));

        presenter_registry.register_fixed_asset_presenter(page_id, presenter);

        Self { page_id, template, presenter_registry, fixed_asset_rx }
    }

    fn load_data(&self, _controllers: &Controllers) {
        let page_id = self.page_id;
        let presenter_registry = self.presenter_registry.clone();

        tokio::spawn(async move {
            if let Some(_presenter) = presenter_registry.get_fixed_asset_presenter(page_id) {
                // 固定資産データを取得してプレゼンタに渡す
            }
        });
    }

    fn poll_data(&mut self) {
        while let Ok(assets) = self.fixed_asset_rx.try_recv() {
            let items: Vec<FixedAssetItemViewModel> = assets
                .into_iter()
                .map(|asset| FixedAssetItemViewModel {
                    asset_id: asset.asset_id,
                    asset_name: asset.asset_name,
                    acquisition_date: asset.acquisition_date,
                    acquisition_cost: format!("{:.2}", asset.acquisition_cost),
                    accumulated_depreciation: format!("{:.2}", asset.accumulated_depreciation),
                    carrying_amount: format!("{:.2}", asset.carrying_amount),
                    useful_life: format!("{}", asset.useful_life),
                })
                .collect();

            self.template.set_data(items, 0, 0);
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl Drop for FixedAssetListPageState {
    fn drop(&mut self) {
        self.presenter_registry.unregister_fixed_asset_presenter(self.page_id);
    }
}

impl PageState for FixedAssetListPageState {
    fn route(&self) -> Route {
        Route::FixedAssetList
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        self.load_data(controllers);

        loop {
            self.poll_data();

            terminal
                .draw(|frame| {
                    self.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            if event::poll(std::time::Duration::from_millis(100))
                .map_err(crate::error::AdapterError::EventReadFailed)?
                && let Event::Key(key) =
                    event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => {
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Enter => {
                        return Ok(NavAction::Go(Route::AssetDetail));
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for FixedAssetListPageState {
    fn default() -> Self {
        Self::new(Arc::new(PresenterRegistry::new()))
    }
}
