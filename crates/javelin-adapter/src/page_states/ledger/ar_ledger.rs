// ArLedgerPageState - 売掛金補助元帳画面
// 責務: 売掛金補助元帳の表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// 売掛金補助元帳項目ViewModel
#[derive(Debug, Clone)]
pub struct ArLedgerItemViewModel {
    pub date: String,
    pub customer: String,
    pub voucher_number: String,
    pub description: String,
    pub debit: String,
    pub credit: String,
    pub balance: String,
}

impl MasterListItem for ArLedgerItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["日付", "取引先", "伝票番号", "摘要", "借方", "貸方", "残高"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Min(20),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.customer.clone(),
            self.voucher_number.clone(),
            self.description.clone(),
            self.debit.clone(),
            self.credit.clone(),
            self.balance.clone(),
        ]
    }
}

/// 売掛金補助元帳画面
pub struct ArLedgerPageState {
    template: MasterListTemplate<ArLedgerItemViewModel>,
}

impl ArLedgerPageState {
    pub fn new() -> Self {
        let template = MasterListTemplate::new("売掛金補助元帳");
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

impl PageState for ArLedgerPageState {
    fn route(&self) -> Route {
        Route::ArLedger
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
                        return Ok(NavAction::Go(Route::ArDetail));
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for ArLedgerPageState {
    fn default() -> Self {
        Self::new()
    }
}
