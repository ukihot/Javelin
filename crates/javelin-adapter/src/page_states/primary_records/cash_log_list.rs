// CashLogListPageState - キャッシュログ一覧画面
// 責務: キャッシュログの一覧表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// キャッシュログ項目ViewModel
#[derive(Debug, Clone)]
pub struct CashLogItemViewModel {
    pub log_id: String,
    pub date: String,
    pub amount: String,
    pub description: String,
    pub category: String,
}

impl MasterListItem for CashLogItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["ログID", "日付", "金額", "摘要", "カテゴリ"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Min(30),
            Constraint::Length(15),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.log_id.clone(),
            self.date.clone(),
            self.amount.clone(),
            self.description.clone(),
            self.category.clone(),
        ]
    }
}

/// キャッシュログ一覧画面
pub struct CashLogListPageState {
    template: MasterListTemplate<CashLogItemViewModel>,
}

impl CashLogListPageState {
    pub fn new() -> Self {
        let template = MasterListTemplate::new("キャッシュログ一覧");
        Self { template }
    }

    fn load_data(&mut self, _controllers: &Controllers) {
        // キャッシュログ用のコントローラは未実装
        // 将来的に CashLogController を実装して使用
        self.template.set_data(vec![], 0, 0);
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for CashLogListPageState {
    fn route(&self) -> Route {
        Route::CashLogList
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

impl Default for CashLogListPageState {
    fn default() -> Self {
        Self::new()
    }
}
