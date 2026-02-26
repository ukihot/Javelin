// LedgerMenuPageState - B-01: Ledger Management Menu

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::MenuPage,
};

pub struct LedgerMenuPageState {
    page: MenuPage,
}

impl LedgerMenuPageState {
    pub fn new() -> Self {
        let menu_items = vec![
            ("Ledger Aggregation Execution", "元帳集約処理実行"),
            ("Ledger Aggregation History", "元帳集約実行履歴"),
            ("General Ledger", "総勘定元帳"),
            ("AR Ledger", "売掛金補助元帳"),
            ("AP Ledger", "買掛金補助元帳"),
        ];

        Self { page: MenuPage::new("B. Ledger Management", &menu_items) }
    }
}

impl Default for LedgerMenuPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl PageState for LedgerMenuPageState {
    fn route(&self) -> Route {
        Route::LedgerMenu
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
                            0 => Route::LedgerAggregationExecution, // B-02: Execution
                            1 => Route::LedgerConsolidation,        // Legacy: History view
                            2 => Route::GeneralLedger,              // B-03
                            3 => Route::ArLedger,                   // B-05
                            4 => Route::ApLedger,                   // B-07
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
