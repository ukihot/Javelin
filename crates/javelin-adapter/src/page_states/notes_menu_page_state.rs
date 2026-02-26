// NotesMenuPageState - E-06: Notes Menu

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::MenuPage,
};

pub struct NotesMenuPageState {
    page: MenuPage,
}

impl NotesMenuPageState {
    pub fn new() -> Self {
        let menu_items = vec![
            ("Accounting Policies", "会計方針"),
            ("Revenue Breakdown", "収益分解（IFRS 15）"),
            ("Fixed Assets Notes", "固定資産・使用権資産"),
            ("Lease Notes", "リース（IFRS 16）"),
            ("Financial Instruments Notes", "金融商品・ECL（IFRS 9）"),
        ];

        Self { page: MenuPage::new("E-06. Notes", &menu_items) }
    }
}

impl Default for NotesMenuPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl PageState for NotesMenuPageState {
    fn route(&self) -> Route {
        Route::NotesMenu
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
                            0 => Route::AccountingPolicies,
                            1 => Route::RevenueBreakdown,
                            2 => Route::FixedAssetsNotes,
                            3 => Route::LeaseNotes,
                            4 => Route::FinancialInstrumentsNotes,
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
