// FixedAssetListPageState - 固定資産一覧画面
// 責務: 固定資産台帳のデータ管理とライフサイクル

use std::sync::Arc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::FixedAssetPresenter,
    views::pages::FixedAssetListPage,
};

pub struct FixedAssetListPageState {
    page_id: Uuid,
    page: FixedAssetListPage,
    presenter_registry: Arc<PresenterRegistry>,
}

impl FixedAssetListPageState {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        let page_id = Uuid::new_v4();

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

        let presenter = FixedAssetPresenter::new(
            fixed_asset_tx,
            depreciation_result_tx,
            lease_contract_tx,
            lease_schedule_tx,
            rou_asset_tx,
        );
        presenter_registry.register_fixed_asset_presenter(page_id, Arc::new(presenter));

        let page = FixedAssetListPage::new(page_id, fixed_asset_rx);

        Self { page_id, page, presenter_registry }
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
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            self.page.poll_assets();

            terminal
                .draw(|frame| {
                    self.page.render(frame);
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
                    KeyCode::Esc => return Ok(NavAction::Back),
                    KeyCode::Char('j') | KeyCode::Down => self.page.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.page.select_previous(),
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
