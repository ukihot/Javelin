// AdjustmentJournalListPageState - 補正仕訳一覧画面
// 責務: 補正仕訳の一覧表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// 補正仕訳項目ViewModel
#[derive(Debug, Clone)]
pub struct AdjustmentJournalItemViewModel {
    pub entry_number: String,
    pub date: String,
    pub account_name: String,
    pub debit: String,
    pub credit: String,
    pub description: String,
}

impl MasterListItem for AdjustmentJournalItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["仕訳番号", "日付", "勘定科目", "借方", "貸方", "摘要"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(15),
            Constraint::Length(12),
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Min(20),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.entry_number.clone(),
            self.date.clone(),
            self.account_name.clone(),
            self.debit.clone(),
            self.credit.clone(),
            self.description.clone(),
        ]
    }
}

/// 補正仕訳一覧画面
pub struct AdjustmentJournalListPageState {
    template: MasterListTemplate<AdjustmentJournalItemViewModel>,
}

impl AdjustmentJournalListPageState {
    pub fn new() -> Self {
        let template = MasterListTemplate::new("補正仕訳一覧");
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

impl PageState for AdjustmentJournalListPageState {
    fn route(&self) -> Route {
        Route::AdjustmentJournalList
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
                        // 詳細画面への遷移（将来実装）
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for AdjustmentJournalListPageState {
    fn default() -> Self {
        Self::new()
    }
}
