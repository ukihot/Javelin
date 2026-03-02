// AccountDetailPageState - 勘定科目明細画面
// 責務: 勘定科目の詳細明細表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// 勘定科目明細項目ViewModel
#[derive(Debug, Clone)]
pub struct AccountDetailItemViewModel {
    pub date: String,
    pub voucher_number: String,
    pub description: String,
    pub debit: String,
    pub credit: String,
    pub balance: String,
}

impl MasterListItem for AccountDetailItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["日付", "伝票番号", "摘要", "借方", "貸方", "残高"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Min(25),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.voucher_number.clone(),
            self.description.clone(),
            self.debit.clone(),
            self.credit.clone(),
            self.balance.clone(),
        ]
    }
}

/// 勘定科目明細画面
pub struct AccountDetailPageState {
    template: MasterListTemplate<AccountDetailItemViewModel>,
}

impl AccountDetailPageState {
    pub fn new() -> Self {
        let template = MasterListTemplate::new("勘定科目明細");
        Self { template }
    }

    fn load_data(&mut self, _controllers: &Controllers) {
        // TODO: 実際のコントローラを使ってデータを取得
        // 現在は空のデータを表示
        self.template.set_data(vec![], 0, 0);
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for AccountDetailPageState {
    fn route(&self) -> Route {
        Route::AccountDetail
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        // 初回データロード
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
                        // 仕訳詳細への遷移（将来実装）
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for AccountDetailPageState {
    fn default() -> Self {
        Self::new()
    }
}
