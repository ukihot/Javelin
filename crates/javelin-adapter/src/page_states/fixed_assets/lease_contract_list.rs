// LeaseContractListPageState - リース契約一覧画面
// 責務: リース契約の一覧表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// リース契約項目ViewModel
#[derive(Debug, Clone)]
pub struct LeaseContractItemViewModel {
    pub contract_id: String,
    pub lessor: String,
    pub asset_name: String,
    pub start_date: String,
    pub end_date: String,
    pub monthly_payment: String,
    pub total_liability: String,
}

impl MasterListItem for LeaseContractItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["契約ID", "貸手", "資産名", "開始日", "終了日", "月額支払額", "リース負債残高"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Length(20),
            Constraint::Min(20),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Length(18),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.contract_id.clone(),
            self.lessor.clone(),
            self.asset_name.clone(),
            self.start_date.clone(),
            self.end_date.clone(),
            self.monthly_payment.clone(),
            self.total_liability.clone(),
        ]
    }
}

/// リース契約一覧画面
pub struct LeaseContractListPageState {
    template: MasterListTemplate<LeaseContractItemViewModel>,
}

impl LeaseContractListPageState {
    pub fn new() -> Self {
        let template = MasterListTemplate::new("リース契約一覧");
        Self { template }
    }

    fn load_data(&mut self, _controllers: &Controllers) {
        self.template.set_data(vec![], 0, 0);
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for LeaseContractListPageState {
    fn route(&self) -> Route {
        Route::LeaseContractList
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

                match key.code {
                    KeyCode::Esc => {
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Enter => {
                        return Ok(NavAction::Go(Route::LeaseContractDetail));
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for LeaseContractListPageState {
    fn default() -> Self {
        Self::new()
    }
}
