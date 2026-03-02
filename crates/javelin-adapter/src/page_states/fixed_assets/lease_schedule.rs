// LeaseSchedulePageState - リース負債スケジュール画面
// 責務: リース負債の支払スケジュール表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// リーススケジュール項目ViewModel
#[derive(Debug, Clone)]
pub struct LeaseScheduleItemViewModel {
    pub payment_date: String,
    pub payment_amount: String,
    pub principal: String,
    pub interest: String,
    pub remaining_balance: String,
}

impl MasterListItem for LeaseScheduleItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["支払日", "支払額", "元本", "利息", "残高"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(18),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.payment_date.clone(),
            self.payment_amount.clone(),
            self.principal.clone(),
            self.interest.clone(),
            self.remaining_balance.clone(),
        ]
    }
}

/// リース負債スケジュール画面
pub struct LeaseSchedulePageState {
    template: MasterListTemplate<LeaseScheduleItemViewModel>,
}

impl LeaseSchedulePageState {
    pub fn new() -> Self {
        let template = MasterListTemplate::new("リース負債スケジュール");
        Self { template }
    }

    fn load_data(&mut self, _controllers: &Controllers) {
        self.template.set_data(vec![], 0, 0);
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for LeaseSchedulePageState {
    fn route(&self) -> Route {
        Route::LeaseSchedule
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

impl Default for LeaseSchedulePageState {
    fn default() -> Self {
        Self::new()
    }
}
