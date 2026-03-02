// RouAssetListPageState - 使用権資産台帳画面
// 責務: 使用権資産（Right-of-Use Asset）の台帳表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// 使用権資産項目ViewModel
#[derive(Debug, Clone)]
pub struct RouAssetItemViewModel {
    pub asset_id: String,
    pub lease_contract_id: String,
    pub asset_name: String,
    pub initial_cost: String,
    pub accumulated_depreciation: String,
    pub carrying_amount: String,
    pub remaining_term: String,
}

impl MasterListItem for RouAssetItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec![
            "資産ID",
            "リース契約ID",
            "資産名",
            "初期測定額",
            "減価償却累計額",
            "帳簿価額",
            "残存期間",
        ]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Min(20),
            Constraint::Length(15),
            Constraint::Length(18),
            Constraint::Length(15),
            Constraint::Length(10),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.asset_id.clone(),
            self.lease_contract_id.clone(),
            self.asset_name.clone(),
            self.initial_cost.clone(),
            self.accumulated_depreciation.clone(),
            self.carrying_amount.clone(),
            self.remaining_term.clone(),
        ]
    }
}

/// 使用権資産台帳画面
pub struct RouAssetListPageState {
    template: MasterListTemplate<RouAssetItemViewModel>,
}

impl RouAssetListPageState {
    pub fn new() -> Self {
        let template = MasterListTemplate::new("使用権資産台帳");
        Self { template }
    }

    fn load_data(&mut self, _controllers: &Controllers) {
        self.template.set_data(vec![], 0, 0);
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for RouAssetListPageState {
    fn route(&self) -> Route {
        Route::RouAssetList
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        self.load_data(controllers);

        loop {
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

impl Default for RouAssetListPageState {
    fn default() -> Self {
        Self::new()
    }
}
