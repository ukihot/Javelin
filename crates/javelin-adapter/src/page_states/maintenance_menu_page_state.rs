// MaintenanceMenuPageState - menu for maintenance-mode operations

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::MenuPage,
};

/// Maintenance menu with a couple of placeholder actions
pub struct MaintenanceMenuPageState {
    page: MenuPage,
}

impl MaintenanceMenuPageState {
    pub fn new() -> Self {
        let menu_items = vec![
            ("Rebuild projections", "再構築プロジェクション"),
            ("Clean event store", "イベントストアクリーニング"),
        ];
        Self { page: MenuPage::new("Maintenance Menu", &menu_items) }
    }
}

impl Default for MaintenanceMenuPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl PageState for MaintenanceMenuPageState {
    fn route(&self) -> Route {
        Route::MaintenanceMenu
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
                            0 => Route::MaintenanceRebuildProjections,
                            1 => Route::MaintenanceCleanEventStore,
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
