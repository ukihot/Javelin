// HomePageState - PageState implementation for home screen
// Wraps HomePage and implements navigation logic

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::{HomePage, home_page::ViewType},
};

/// PageState implementation for the home screen
///
/// The home screen displays the main menu and allows users to
/// navigate to other screens. It has no channels or async communication.
pub struct HomePageState {
    page: HomePage,
}

impl HomePageState {
    /// Create a new HomePageState
    pub fn new() -> Self {
        Self { page: HomePage::new() }
    }
}

impl PageState for HomePageState {
    fn route(&self) -> Route {
        Route::Home
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

                match key.code {
                    KeyCode::Char('q') => {
                        // Exit application
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Char('h') | KeyCode::Char('l') => {
                        self.page.switch_menu();
                    }
                    KeyCode::Char('k') => {
                        self.page.select_previous();
                    }
                    KeyCode::Char('j') => {
                        self.page.select_next();
                    }
                    KeyCode::Enter => {
                        // Navigate to selected screen
                        if let Some(view_type) = self.page.get_selected_view() {
                            let route = view_type_to_route(view_type);
                            return Ok(NavAction::Go(route));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn on_navigation_error(&mut self, error_message: &str) {
        self.page.add_error(error_message);
    }
}

impl Default for HomePageState {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert ViewType to Route
///
/// Maps the old ViewType enum to the new hierarchical Route structure.
/// Now routes to section menus instead of direct screens.
fn view_type_to_route(view_type: ViewType) -> Route {
    match view_type {
        ViewType::Home => Route::Home,
        // A. Primary Records
        ViewType::JournalEntry => Route::PrimaryRecordsMenu,
        ViewType::Search => Route::JournalList,
        // B. Ledger Management
        ViewType::Ledger => Route::LedgerMenu,
        ViewType::LedgerConsolidation => Route::FixedAssetsMenu, // C. Fixed Assets
        // D. Monthly Closing
        ViewType::ClosingPreparation => Route::ClosingMenu,
        ViewType::ClosingLock => Route::ClosingLockExecution,
        ViewType::TrialBalance => Route::TrialBalance,
        ViewType::NoteDraft => Route::NotesDraft,
        ViewType::AccountAdjustment => Route::ManagementAccountingMenu, // F. Management
        // Accounting
        // (placeholder)
        ViewType::IfrsValuation => Route::ManagementAccountingMenu, // F. Management Accounting
        ViewType::FinancialStatement => Route::FinancialStatementsMenu,
        // H. Master Management
        ViewType::AccountMasterManagement => Route::MasterManagementMenu,
        ViewType::SubsidiaryAccountMasterManagement => Route::SubsidiaryAccounts,
        ViewType::UserSettingsManagement => Route::PeriodManagement,
        // Legacy - no longer used
        ViewType::DataImport => Route::Home,
        ViewType::DataExport => Route::Home,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_returns_home() {
        let state = HomePageState::new();
        assert_eq!(state.route(), Route::Home);
    }

    #[test]
    fn test_default_implementation() {
        let state1 = HomePageState::new();
        let state2 = HomePageState::default();

        assert_eq!(state1.route(), state2.route());
    }
}
