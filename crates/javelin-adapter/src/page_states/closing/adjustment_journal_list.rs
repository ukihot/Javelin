// AdjustmentJournalListPageState - 補正仕訳一覧画面
// 責務: 補正仕訳のデータ管理とライフサイクル

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::AdjustmentJournalListPage,
};

pub struct AdjustmentJournalListPageState {
    page: AdjustmentJournalListPage,
}

impl AdjustmentJournalListPageState {
    pub fn new() -> Self {
        Self { page: AdjustmentJournalListPage::new() }
    }
}

impl PageState for AdjustmentJournalListPageState {
    fn route(&self) -> Route {
        Route::AdjustmentJournalList
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
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

impl Default for AdjustmentJournalListPageState {
    fn default() -> Self {
        Self::new()
    }
}
