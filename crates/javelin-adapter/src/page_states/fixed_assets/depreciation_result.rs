// DepreciationResultPageState - 償却計算結果一覧画面
// 責務: 減価償却計算結果の一覧表示

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

/// 償却計算結果項目ViewModel
#[derive(Debug, Clone)]
pub struct DepreciationResultItemViewModel {
    pub asset_id: String,
    pub asset_name: String,
    pub depreciation_method: String,
    pub depreciation_amount: String,
    pub accumulated_depreciation: String,
    pub carrying_amount: String,
}

impl MasterListItem for DepreciationResultItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["資産ID", "資産名", "償却方法", "当期償却額", "減価償却累計額", "帳簿価額"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Min(20),
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Length(18),
            Constraint::Length(15),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.asset_id.clone(),
            self.asset_name.clone(),
            self.depreciation_method.clone(),
            self.depreciation_amount.clone(),
            self.accumulated_depreciation.clone(),
            self.carrying_amount.clone(),
        ]
    }
}

/// 償却計算結果一覧画面
pub struct DepreciationResultPageState {
    page_id: Uuid,
    template: MasterListTemplate<DepreciationResultItemViewModel>,
    presenter_registry: Arc<PresenterRegistry>,
    depreciation_result_rx:
        tokio::sync::mpsc::UnboundedReceiver<Vec<crate::presenter::DepreciationResultViewModel>>,
}

impl DepreciationResultPageState {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        let page_id = Uuid::new_v4();
        let template = MasterListTemplate::new("償却計算結果一覧");

        let (
            fixed_asset_tx,
            _fixed_asset_rx,
            depreciation_result_tx,
            depreciation_result_rx,
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

        Self { page_id, template, presenter_registry, depreciation_result_rx }
    }

    fn load_data(&self, _controllers: &Controllers) {
        let page_id = self.page_id;
        let presenter_registry = self.presenter_registry.clone();

        tokio::spawn(async move {
            if let Some(_presenter) = presenter_registry.get_fixed_asset_presenter(page_id) {
                // 減価償却結果データを取得してプレゼンタに渡す
            }
        });
    }

    fn poll_data(&mut self) {
        while let Ok(results) = self.depreciation_result_rx.try_recv() {
            let items: Vec<DepreciationResultItemViewModel> = results
                .into_iter()
                .map(|result| DepreciationResultItemViewModel {
                    asset_id: result.asset_id,
                    asset_name: result.asset_name,
                    depreciation_method: result.depreciation_method,
                    depreciation_amount: format!("{:.2}", result.depreciation_amount),
                    accumulated_depreciation: format!("{:.2}", result.accumulated_depreciation),
                    carrying_amount: format!("{:.2}", result.carrying_amount),
                })
                .collect();

            self.template.set_data(items, 0, 0);
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl Drop for DepreciationResultPageState {
    fn drop(&mut self) {
        self.presenter_registry.unregister_fixed_asset_presenter(self.page_id);
    }
}

impl PageState for DepreciationResultPageState {
    fn route(&self) -> Route {
        Route::DepreciationResult
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

                if key.code == KeyCode::Esc {
                    return Ok(NavAction::Back);
                }
            }
        }
    }
}

impl Default for DepreciationResultPageState {
    fn default() -> Self {
        Self::new(Arc::new(PresenterRegistry::new()))
    }
}
