// MasterManagementMenuPageState - H-01: Master Management Menu

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::MenuPage,
};

pub struct MasterManagementMenuPageState {
    page: MenuPage,
}

impl MasterManagementMenuPageState {
    pub fn new() -> Self {
        let menu_items = vec![
            ("Chart of Accounts", "勘定科目マスタ"),
            ("Subsidiary Accounts", "補助科目マスタ"),
            ("Business Partners", "取引先マスタ"),
        ];

        Self { page: MenuPage::new("H. Master Management", &menu_items) }
    }
}

impl Default for MasterManagementMenuPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl PageState for MasterManagementMenuPageState {
    fn route(&self) -> Route {
        Route::MasterManagementMenu
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
                            0 => Route::ChartOfAccounts,
                            1 => Route::SubsidiaryAccounts,
                            2 => Route::BusinessPartners,
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
