// FinancialStatementsMenuPageState - E-01: Financial Statements Menu

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::MenuPage,
};

pub struct FinancialStatementsMenuPageState {
    page: MenuPage,
}

impl FinancialStatementsMenuPageState {
    pub fn new() -> Self {
        let menu_items = vec![
            ("Balance Sheet", "財政状態計算書（BS）"),
            ("P/L and OCI", "損益及びその他の包括利益計算書"),
            ("Cash Flow Statement", "キャッシュフロー計算書（SCF）"),
            ("Statement of Changes in Equity", "持分変動計算書（SCE）"),
            ("Notes", "注記"),
        ];

        Self { page: MenuPage::new("E. Financial Statements", &menu_items) }
    }
}

impl Default for FinancialStatementsMenuPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl PageState for FinancialStatementsMenuPageState {
    fn route(&self) -> Route {
        Route::FinancialStatementsMenu
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
                            0 => Route::BalanceSheet,
                            1 => Route::PlAndOci,
                            2 => Route::CashFlowStatement,
                            3 => Route::StatementOfChangesInEquity,
                            4 => Route::NotesMenu,
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
