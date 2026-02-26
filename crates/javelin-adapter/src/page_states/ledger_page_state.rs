// LedgerPageState - Page state for ledger screen
// Simple page with minimal channels

use std::sync::{Arc, Mutex};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    presenter::LedgerEntryViewModel,
    views::pages::LedgerPage,
};

// Shared state for passing selected entry to detail view
lazy_static::lazy_static! {
    static ref SELECTED_LEDGER_ENTRY: Arc<Mutex<Option<(LedgerEntryViewModel, String, String)>>> =
        Arc::new(Mutex::new(None));
}

/// Ledger page state
pub struct LedgerPageState {
    /// The ledger page view
    page: LedgerPage,
}

impl LedgerPageState {
    /// Create a new LedgerPageState
    ///
    /// Creates a LedgerPage with a dummy channel receiver.
    /// In the future, this should be updated to use PresenterRegistry
    /// similar to SearchPageState and JournalEntryPageState.
    ///
    /// # Requirements
    ///
    /// Validates: Requirements 3.3, 7.1
    pub fn new() -> Self {
        // Create a dummy channel for now
        // TODO: Update to use PresenterRegistry pattern
        let (_tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let page = LedgerPage::new(rx);

        Self { page }
    }

    /// 選択されたエントリのインデックスを取得
    pub fn selected_entry_index(&self) -> Option<usize> {
        self.page.selected_index()
    }

    /// 選択されたエントリを共有状態に保存
    fn store_selected_entry(&self) {
        if let Some(entry) = self.page.get_selected_entry()
            && let (Some(code), Some(name)) =
                (self.page.get_account_code(), self.page.get_account_name())
            && let Ok(mut guard) = SELECTED_LEDGER_ENTRY.lock()
        {
            *guard = Some((entry.clone(), code, name));
        }
    }

    /// 共有状態から選択されたエントリを取得
    pub fn take_selected_entry() -> Option<(LedgerEntryViewModel, String, String)> {
        SELECTED_LEDGER_ENTRY.lock().ok()?.take()
    }
}

impl Default for LedgerPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl PageState for LedgerPageState {
    fn route(&self) -> Route {
        Route::Ledger
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            // Update page state
            self.page.update();

            // Tick animation
            self.page.tick();

            // Render the page
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // Handle events with timeout for animation
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
                        // Navigate back to home
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Enter => {
                        // Navigate to ledger detail view
                        if self.selected_entry_index().is_some() {
                            self.store_selected_entry();
                            return Ok(NavAction::Go(Route::LedgerDetail));
                        }
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
