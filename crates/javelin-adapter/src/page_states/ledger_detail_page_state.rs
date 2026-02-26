// LedgerDetailPageState - PageState implementation for ledger detail view screen

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    page_states::LedgerPageState,
    views::pages::LedgerDetailPage,
};

#[derive(Default)]
pub struct LedgerDetailPageState {
    page: LedgerDetailPage,
}

impl LedgerDetailPageState {
    pub fn new() -> Self {
        // Try to get the selected entry from shared state
        if let Some((entry, account_code, account_name)) = LedgerPageState::take_selected_entry() {
            Self { page: LedgerDetailPage::new(entry, account_code, account_name) }
        } else {
            // Fallback to default if no data available
            Self { page: LedgerDetailPage::default() }
        }
    }
}

impl PageState for LedgerDetailPageState {
    fn route(&self) -> Route {
        Route::LedgerDetail
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            // Render the page
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // Handle events
            if let Event::Key(key) =
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

    fn on_navigation_error(&mut self, error_message: &str) {
        self.page.add_error(error_message);
    }
}
