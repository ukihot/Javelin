// ApDetailPageState - 買掛金明細画面
// 責務: 買掛金の詳細明細表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// 買掛金明細項目ViewModel
#[derive(Debug, Clone)]
pub struct ApDetailItemViewModel {
    pub date: String,
    pub vendor: String,
    pub invoice_number: String,
    pub description: String,
    pub amount: String,
    pub payment_status: String,
    pub due_date: String,
}

impl MasterListItem for ApDetailItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["日付", "取引先", "請求番号", "摘要", "金額", "支払状況", "期日"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Min(20),
            Constraint::Length(15),
            Constraint::Length(12),
            Constraint::Length(12),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.vendor.clone(),
            self.invoice_number.clone(),
            self.description.clone(),
            self.amount.clone(),
            self.payment_status.clone(),
            self.due_date.clone(),
        ]
    }
}

/// 買掛金明細画面
pub struct ApDetailPageState {
    template: MasterListTemplate<ApDetailItemViewModel>,
}

impl ApDetailPageState {
    pub fn new() -> Self {
        let template = MasterListTemplate::new("買掛金明細");
        Self { template }
    }

    fn load_data(&mut self, _controllers: &Controllers) {
        // TODO: 実際のコントローラを使ってデータを取得
        self.template.set_data(vec![], 0, 0);
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for ApDetailPageState {
    fn route(&self) -> Route {
        Route::ApDetail
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

impl Default for ApDetailPageState {
    fn default() -> Self {
        Self::new()
    }
}
