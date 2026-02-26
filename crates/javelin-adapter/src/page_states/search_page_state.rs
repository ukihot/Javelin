// SearchPageState - Page state for search screen
// Owns channels and manages search page lifecycle

use std::sync::Arc;

use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::{AccountMasterPresenter, SearchPresenter},
    views::pages::SearchPage,
};

/// Search page state with owned channels
pub struct SearchPageState {
    /// Unique identifier for presenter registration
    id: Uuid,
    /// Reference to presenter registry
    registry: Arc<PresenterRegistry>,
    /// The search page view
    page: SearchPage,
    /// Account master presenter for this page
    #[allow(dead_code)]
    account_master_presenter: Arc<AccountMasterPresenter>,
}

impl SearchPageState {
    /// Create a new SearchPageState with its own channels
    ///
    /// This method:
    /// 1. Creates 5 channels (result, error, progress, execution_time, account_master)
    /// 2. Creates SearchPresenter and AccountMasterPresenter with channel senders
    /// 3. Registers presenters in PresenterRegistry with unique ID
    /// 4. Creates SearchPage with channel receivers
    ///
    /// # Arguments
    ///
    /// * `registry` - Shared presenter registry for controller access
    ///
    /// # Requirements
    ///
    /// Validates: Requirements 7.1, 7.3, 10.1
    pub fn new(registry: Arc<PresenterRegistry>) -> Self {
        // Generate unique ID for this page instance
        let id = Uuid::new_v4();

        // Create 4 channels for search communication
        let (result_tx, result_rx) = tokio::sync::mpsc::channel(100);
        let (error_tx, error_rx) = tokio::sync::mpsc::channel(100);
        let (progress_tx, progress_rx) = tokio::sync::mpsc::channel(100);
        let (execution_time_tx, execution_time_rx) = tokio::sync::mpsc::channel(100);

        // Create channel for account master (unbounded to match presenter)
        let (account_master_tx, account_master_rx) = tokio::sync::mpsc::unbounded_channel();

        // Create SearchPresenter with channel senders
        let presenter =
            Arc::new(SearchPresenter::new(result_tx, error_tx, progress_tx, execution_time_tx));

        // Create AccountMasterPresenter with channel sender
        let account_master_presenter = Arc::new(AccountMasterPresenter::new(account_master_tx));

        // Register presenters in PresenterRegistry with unique ID
        registry.register_search_presenter(id, presenter);
        registry.register_account_master_presenter(id, Arc::clone(&account_master_presenter));

        // Create SearchPage with channel receivers
        let mut page = SearchPage::new(result_rx, error_rx, progress_rx, execution_time_rx);
        page.set_account_master_receiver(account_master_rx);

        Self { id, registry, page, account_master_presenter }
    }
}

impl PageState for SearchPageState {
    fn route(&self) -> Route {
        Route::Search
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        use crossterm::event::{self, Event, KeyCode, KeyEventKind};

        loop {
            // 科目マスター読み込み待機中の場合、読み込みを開始
            if self.page.is_pending_account_load() {
                self.page.clear_pending_account_load();

                let controller = Arc::clone(&controllers.account_master);
                let page_id = self.id;

                tokio::spawn(async move {
                    use javelin_application::dtos::request::LoadAccountMasterRequest;

                    let request = LoadAccountMasterRequest { filter: None, active_only: true };

                    let _ = controller.handle_load_account_master(page_id, request).await;
                });
            }

            // Update page state (check for async messages)
            self.page.update();

            // Tick animation
            self.page.tick();

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

                match self.page.input_mode() {
                    crate::input_mode::InputMode::Normal => {
                        match key.code {
                            KeyCode::Esc => {
                                // Navigate back to home
                                return Ok(NavAction::Back);
                            }
                            KeyCode::Char('i') => {
                                // Enter modify mode
                                self.page.enter_modify_mode();
                            }
                            KeyCode::Char('h') => {
                                self.page.focus_left();
                            }
                            KeyCode::Char('j') => match self.page.focus_area() {
                                crate::views::pages::search_page::FocusArea::Criteria => {
                                    self.page.focus_down();
                                }
                                crate::views::pages::search_page::FocusArea::Results => {
                                    self.page.select_next();
                                }
                            },
                            KeyCode::Char('k') => match self.page.focus_area() {
                                crate::views::pages::search_page::FocusArea::Criteria => {
                                    self.page.focus_up();
                                }
                                crate::views::pages::search_page::FocusArea::Results => {
                                    self.page.select_previous();
                                }
                            },
                            KeyCode::Char('l') => {
                                self.page.focus_right();
                            }
                            KeyCode::Tab => {
                                self.page.toggle_focus_area();
                            }
                            KeyCode::Enter => {
                                // Execute search
                                let criteria = self.page.to_search_criteria_dto();
                                let page_id = self.id;
                                let controller = Arc::clone(&controllers.search);

                                // Spawn async task to execute search
                                tokio::spawn(async move {
                                    let _ = controller.handle_search(page_id, criteria).await;
                                });
                            }
                            KeyCode::Char('c') => {
                                // Clear search criteria
                                self.page.clear_criteria();
                            }
                            _ => {}
                        }
                    }
                    crate::input_mode::InputMode::Modify => {
                        match key.code {
                            KeyCode::Esc => {
                                // オーバーレイが表示されている場合はキャンセル
                                if self.page.is_overlay_visible() {
                                    self.page.overlay_cancel();
                                } else {
                                    // Exit modify mode
                                    self.page.enter_normal_mode();
                                }
                            }
                            KeyCode::Char('j') if self.page.is_overlay_visible() => {
                                // オーバーレイで次の項目を選択
                                self.page.overlay_select_next();
                            }
                            KeyCode::Char('k') if self.page.is_overlay_visible() => {
                                // オーバーレイで前の項目を選択
                                self.page.overlay_select_previous();
                            }
                            KeyCode::Char(ch) => {
                                // Input character (jj detection handled inside)
                                self.page.input_char(ch);
                            }
                            KeyCode::Backspace => {
                                self.page.backspace();
                            }
                            KeyCode::Enter => {
                                // オーバーレイが表示されている場合は選択を確定
                                if self.page.is_overlay_visible() {
                                    self.page.overlay_confirm_selection();
                                } else {
                                    // Commit and exit modify mode
                                    self.page.enter_normal_mode();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

impl Drop for SearchPageState {
    fn drop(&mut self) {
        // Unregister presenters from registry when page is destroyed
        self.registry.unregister_search_presenter(self.id);
        self.registry.unregister_account_master_presenter(self.id);
        // Channels are automatically cleaned up when dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Unit Tests (Task 6.8)

    #[test]
    fn test_route_returns_search() {
        let registry = Arc::new(PresenterRegistry::new());
        let state = SearchPageState::new(Arc::clone(&registry));
        assert_eq!(state.route(), Route::Search);
    }

    #[test]
    fn test_channel_creation_and_presenter_registration() {
        let registry = Arc::new(PresenterRegistry::new());
        let id_before = registry.total_count();

        let state = SearchPageState::new(Arc::clone(&registry));

        // Verify presenter was registered
        assert_eq!(registry.total_count(), id_before + 2);
        assert!(registry.get_search_presenter(state.id).is_some());
    }

    #[test]
    fn test_presenter_unregistration_on_drop() {
        let registry = Arc::new(PresenterRegistry::new());
        let id_before = registry.total_count();

        {
            let state = SearchPageState::new(Arc::clone(&registry));
            let state_id = state.id;

            // Verify presenter is registered
            assert_eq!(registry.total_count(), id_before + 2);
            assert!(registry.get_search_presenter(state_id).is_some());

            // state goes out of scope here
        }

        // Verify presenter was unregistered
        assert_eq!(registry.total_count(), id_before);
    }

    #[test]
    fn test_multiple_search_page_instances() {
        let registry = Arc::new(PresenterRegistry::new());

        let state1 = SearchPageState::new(Arc::clone(&registry));
        let state2 = SearchPageState::new(Arc::clone(&registry));
        let state3 = SearchPageState::new(Arc::clone(&registry));

        // All three should have unique IDs
        assert_ne!(state1.id, state2.id);
        assert_ne!(state1.id, state3.id);
        assert_ne!(state2.id, state3.id);

        // All three should be registered
        assert_eq!(registry.total_count(), 6);
        assert!(registry.get_search_presenter(state1.id).is_some());
        assert!(registry.get_search_presenter(state2.id).is_some());
        assert!(registry.get_search_presenter(state3.id).is_some());
    }

    // Property Tests (Task 6.7)
    // These validate Properties 4, 5, 7, 15 from the design document

    #[test]
    fn property_4_inactive_screens_have_no_resources() {
        // Property 4: Inactive Screens Have No Resources
        // For any screen that is not in the current navigation stack,
        // the system should not have allocated communication channels for that screen.

        let registry = Arc::new(PresenterRegistry::new());

        // Initially, no screens are active
        assert_eq!(registry.total_count(), 0);

        // Create a search page (simulating navigation to search)
        let state = SearchPageState::new(Arc::clone(&registry));
        assert_eq!(registry.total_count(), 2);

        // Drop the search page (simulating navigation away)
        drop(state);

        // Resources should be cleaned up
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn property_5_resource_cleanup_on_navigation() {
        // Property 5: Resource Cleanup on Navigation
        // For any screen that is removed from the navigation stack,
        // all resources (channels, presenters, memory) associated with that screen
        // should be released.

        let registry = Arc::new(PresenterRegistry::new());

        // Create multiple search pages
        let state1 = SearchPageState::new(Arc::clone(&registry));
        let id1 = state1.id;
        let state2 = SearchPageState::new(Arc::clone(&registry));
        let id2 = state2.id;
        let state3 = SearchPageState::new(Arc::clone(&registry));
        let id3 = state3.id;

        assert_eq!(registry.total_count(), 6);

        // Drop state2 (simulating back navigation)
        drop(state2);
        assert_eq!(registry.total_count(), 4);
        assert!(registry.get_search_presenter(id1).is_some());
        assert!(registry.get_search_presenter(id2).is_none()); // Cleaned up
        assert!(registry.get_search_presenter(id3).is_some());

        // Drop state1
        drop(state1);
        assert_eq!(registry.total_count(), 2);
        assert!(registry.get_search_presenter(id1).is_none()); // Cleaned up
        assert!(registry.get_search_presenter(id3).is_some());

        // Drop state3
        drop(state3);
        assert_eq!(registry.total_count(), 0);
        assert!(registry.get_search_presenter(id3).is_none()); // Cleaned up
    }

    #[test]
    fn property_7_independent_channel_management() {
        // Property 7: Independent Channel Management
        // For any screen instance, that screen should own and manage its own
        // communication channels without sharing them with other screens.

        let registry = Arc::new(PresenterRegistry::new());

        let state1 = SearchPageState::new(Arc::clone(&registry));
        let state2 = SearchPageState::new(Arc::clone(&registry));

        // Each state should have its own unique presenter
        let presenter1 = registry.get_search_presenter(state1.id).unwrap();
        let presenter2 = registry.get_search_presenter(state2.id).unwrap();

        // Presenters should be different instances
        assert!(!Arc::ptr_eq(&presenter1, &presenter2));

        // Each state manages its own channels independently
        // (verified by having separate presenter instances)
    }

    #[test]
    fn property_15_screen_specific_channel_creation() {
        // Property 15: Screen-Specific Channel Creation
        // For any screen that requires communication channels,
        // those channels should be created when the screen is instantiated
        // and should be specific to that screen instance.

        let registry = Arc::new(PresenterRegistry::new());

        // Before creating the page, no presenters exist
        let count_before = registry.total_count();

        // Create a search page
        let state = SearchPageState::new(Arc::clone(&registry));

        // After creating the page, exactly one presenter should exist
        assert_eq!(registry.total_count(), count_before + 2);

        // The presenter should be retrievable by the page's ID
        assert!(registry.get_search_presenter(state.id).is_some());

        // Create another search page
        let state2 = SearchPageState::new(Arc::clone(&registry));

        // Now two presenters should exist
        assert_eq!(registry.total_count(), count_before + 4);

        // Both presenters should be retrievable
        assert!(registry.get_search_presenter(state.id).is_some());
        assert!(registry.get_search_presenter(state2.id).is_some());
    }

    #[test]
    fn property_17_channel_isolation_between_instances() {
        // Property 17: Channel Isolation Between Screen Instances
        // For any two screen instances (even of the same type),
        // each should have completely separate communication channels with no sharing.

        let registry = Arc::new(PresenterRegistry::new());

        // Create two search page instances
        let state1 = SearchPageState::new(Arc::clone(&registry));
        let state2 = SearchPageState::new(Arc::clone(&registry));

        // Get their presenters
        let presenter1 = registry.get_search_presenter(state1.id).unwrap();
        let presenter2 = registry.get_search_presenter(state2.id).unwrap();

        // Verify they are different instances (no sharing)
        assert!(!Arc::ptr_eq(&presenter1, &presenter2));

        // Verify they have different IDs
        assert_ne!(state1.id, state2.id);

        // Drop one instance
        drop(state1);

        // The other instance should still have its presenter
        assert!(registry.get_search_presenter(state2.id).is_some());

        // The dropped instance's presenter should be gone
        // (This verifies complete isolation - dropping one doesn't affect the other)
    }
}
