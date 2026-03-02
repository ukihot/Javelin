// LedgerConsistencyVerificationPageState - 元帳整合性検証画面
// 責務: 元帳整合性検証処理の実行と結果表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::LedgerConsistencyVerificationPage,
};

pub struct LedgerConsistencyVerificationPageState {
    page: LedgerConsistencyVerificationPage,
}

impl LedgerConsistencyVerificationPageState {
    pub fn new() -> Self {
        Self { page: LedgerConsistencyVerificationPage::new() }
    }
}

impl PageState for LedgerConsistencyVerificationPageState {
    fn route(&self) -> Route {
        Route::LedgerConsistencyVerification
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            // Tick animation
            self.page.tick();

            // Update page data
            self.page.update(controllers);

            // Render the page
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // Handle events with timeout for animation updates
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
                    KeyCode::Char('s') => {
                        self.page.start_verification(controllers);
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.page.select_next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.page.select_previous();
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for LedgerConsistencyVerificationPageState {
    fn default() -> Self {
        Self::new()
    }
}
