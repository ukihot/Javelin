// PrimaryRecordsMenuPageState - A-01: Primary Records Menu
// Menu screen for primary records registration section

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::MenuPage,
};

/// A-01: Primary Records Menu
///
/// Menu screen providing access to:
/// - Journal Entry (A-02 to A-05)
/// - Cash Log (A-06 to A-07)
pub struct PrimaryRecordsMenuPageState {
    page: MenuPage,
}

impl PrimaryRecordsMenuPageState {
    pub fn new() -> Self {
        let menu_items = vec![
            ("Journal Entry Input", "仕訳入力画面"),
            ("Journal Entry Search", "仕訳検索・一覧"),
            ("Document Management", "証憑管理"),
            ("Cash Log Input", "キャッシュログ入力"),
            ("Cash Log List", "キャッシュログ一覧"),
        ];

        Self { page: MenuPage::new("A. Primary Records Registration", &menu_items) }
    }
}

impl Default for PrimaryRecordsMenuPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl PageState for PrimaryRecordsMenuPageState {
    fn route(&self) -> Route {
        Route::PrimaryRecordsMenu
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

            if let Event::Key(key) =
                event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => return Ok(NavAction::Back),
                    KeyCode::Char('j') | KeyCode::Down => self.page.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.page.select_previous(),
                    KeyCode::Enter => {
                        let route = match self.page.selected_index() {
                            0 => Route::JournalEntry,       // A-02: Journal Entry Input
                            1 => Route::JournalList,        // A-03: Journal Entry Search
                            2 => Route::DocumentManagement, // A-05: Document Management
                            3 => Route::CashLogInput,       // A-06: Cash Log Input
                            4 => Route::CashLogList,        // A-07: Cash Log List
                            _ => continue,
                        };
                        return Ok(NavAction::Go(route));
                    }
                    _ => {}
                }
            }
        }
    }
}
