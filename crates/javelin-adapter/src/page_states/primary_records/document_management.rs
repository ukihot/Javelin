// DocumentManagementPageState - 証憑管理画面
// 責務: 証憑のデータ管理とライフサイクル

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::DocumentManagementPage,
};

pub struct DocumentManagementPageState {
    page: DocumentManagementPage,
}

impl DocumentManagementPageState {
    pub fn new() -> Self {
        Self { page: DocumentManagementPage::new() }
    }
}

impl PageState for DocumentManagementPageState {
    fn route(&self) -> Route {
        Route::DocumentManagement
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

impl Default for DocumentManagementPageState {
    fn default() -> Self {
        Self::new()
    }
}
