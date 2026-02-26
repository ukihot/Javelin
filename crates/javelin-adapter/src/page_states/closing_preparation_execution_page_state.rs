// ClosingPreparationExecutionPageState - 締準備実行画面の状態管理
// 責務: 締準備実行画面の状態とイベント処理

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, Route, page_state::PageState},
    views::pages::ClosingPreparationExecutionPage,
};

pub struct ClosingPreparationExecutionPageState {
    page: ClosingPreparationExecutionPage,
}

impl ClosingPreparationExecutionPageState {
    pub fn new() -> Self {
        Self { page: ClosingPreparationExecutionPage::new() }
    }
}

impl PageState for ClosingPreparationExecutionPageState {
    fn route(&self) -> Route {
        Route::ClosingPreparationExecution
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            self.page.tick();

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
                    KeyCode::Char('s') => self.page.start_execution(),
                    KeyCode::Char('j') | KeyCode::Down => self.page.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.page.select_previous(),
                    _ => {}
                }
            }
        }
    }

    fn on_navigation_error(&mut self, error_message: &str) {
        self.page.add_error(error_message);
    }
}

impl Default for ClosingPreparationExecutionPageState {
    fn default() -> Self {
        Self::new()
    }
}
