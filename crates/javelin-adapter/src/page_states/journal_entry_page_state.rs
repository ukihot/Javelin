// JournalEntryPageState - Page state for journal entry form screen
// Owns channels and manages journal entry page lifecycle

use std::sync::Arc;

use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::{AccountMasterPresenter, JournalEntryPresenter},
    views::pages::JournalEntryFormPage,
};

/// Journal entry page state with owned channels
pub struct JournalEntryPageState {
    /// Unique identifier for presenter registration
    id: Uuid,
    /// Reference to presenter registry
    registry: Arc<PresenterRegistry>,
    /// The journal entry form page view
    page: JournalEntryFormPage,
    /// Account master presenter for this page
    #[allow(dead_code)]
    account_master_presenter: Arc<AccountMasterPresenter>,
    /// Journal entry presenter for this page
    #[allow(dead_code)]
    journal_entry_presenter: Arc<JournalEntryPresenter>,
}

impl JournalEntryPageState {
    /// Create a new JournalEntryPageState with its own channels
    ///
    /// This method:
    /// 1. Creates 3 channels (account_master, result, progress)
    /// 2. Creates AccountMasterPresenter and JournalEntryPresenter with senders
    /// 3. Registers presenters in PresenterRegistry
    /// 4. Creates JournalEntryFormPage with receivers
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

        // Create 3 channels for journal entry communication
        let (account_master_tx, account_master_rx) = tokio::sync::mpsc::unbounded_channel();
        let (result_tx, result_rx) = tokio::sync::mpsc::unbounded_channel();
        let (progress_tx, progress_rx) = tokio::sync::mpsc::unbounded_channel();

        // Create AccountMasterPresenter with channel sender
        let account_master_presenter =
            Arc::new(AccountMasterPresenter::new(account_master_tx.clone()));

        // Create JournalEntryPresenter with channel senders
        // Note: JournalEntryPresenter needs 4 channels, but we only use 2 here
        // The other 2 (list and detail) are not used in the form page
        let (list_tx, _list_rx) = tokio::sync::mpsc::unbounded_channel();
        let (detail_tx, _detail_rx) = tokio::sync::mpsc::unbounded_channel();
        let journal_entry_presenter = Arc::new(JournalEntryPresenter::new(
            list_tx,
            detail_tx,
            result_tx.clone(),
            progress_tx.clone(),
        ));

        // Register presenters in PresenterRegistry with unique ID
        registry.register_account_master_presenter(id, Arc::clone(&account_master_presenter));
        registry.register_journal_entry_presenter(id, Arc::clone(&journal_entry_presenter));

        // Create JournalEntryFormPage and set receivers
        let mut page = JournalEntryFormPage::new();
        page.set_account_master_receiver(account_master_rx);
        page.set_result_receiver(result_rx);
        page.set_progress_receiver(progress_rx);

        Self { id, registry, page, account_master_presenter, journal_entry_presenter }
    }
}

impl PageState for JournalEntryPageState {
    fn route(&self) -> Route {
        Route::JournalEntry
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

        loop {
            // Poll for async data updates
            self.page.poll_account_master_data();
            self.page.poll_result_data();
            self.page.poll_progress_messages();

            // Check if account master data needs to be loaded
            if self.page.has_pending_account_load() {
                self.page.clear_pending_account_load();

                let controller = Arc::clone(&controllers.account_master);
                let page_id = self.id;

                tokio::spawn(async move {
                    use javelin_application::dtos::request::LoadAccountMasterRequest;

                    let request = LoadAccountMasterRequest { filter: None, active_only: true };

                    let _ = controller.handle_load_account_master(page_id, request).await;
                });
            }

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
                            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                // Submit journal entry (Ctrl+S)
                                if !self.page.is_submitting() {
                                    match self.page.to_register_request("system_user".to_string()) {
                                        Ok(request) => {
                                            self.page.start_submit();

                                            let page_id = self.id;
                                            let controller = Arc::clone(&controllers.journal_entry);

                                            tokio::spawn(async move {
                                                let _ = controller
                                                    .handle_register_journal_entry(page_id, request)
                                                    .await;
                                            });
                                        }
                                        Err(e) => {
                                            self.page.set_submit_failed(e);
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('j') | KeyCode::Down => {
                                self.page.focus_next();
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                self.page.focus_previous();
                            }
                            KeyCode::Tab => {
                                self.page.focus_next();
                            }
                            KeyCode::BackTab => {
                                self.page.focus_previous();
                            }
                            _ => {}
                        }
                    }
                    crate::input_mode::InputMode::Modify => {
                        // Check if overlay is visible
                        if self.page.is_overlay_visible() {
                            // Overlay-specific key handling
                            match key.code {
                                KeyCode::Esc => {
                                    // Cancel overlay
                                    self.page.overlay_cancel();
                                }
                                KeyCode::Char('j') | KeyCode::Down => {
                                    // Move selection down in overlay
                                    self.page.overlay_select_next();
                                }
                                KeyCode::Char('k') | KeyCode::Up => {
                                    // Move selection up in overlay
                                    self.page.overlay_select_previous();
                                }
                                KeyCode::Enter => {
                                    // Confirm selection
                                    self.page.overlay_confirm_selection();
                                }
                                _ => {}
                            }
                        } else {
                            // Normal modify mode key handling
                            match key.code {
                                KeyCode::Esc => {
                                    // Exit modify mode
                                    self.page.enter_normal_mode();
                                }
                                KeyCode::Char(ch) => {
                                    // Input character (jj detection handled inside)
                                    self.page.input_char(ch);
                                }
                                KeyCode::Backspace => {
                                    self.page.backspace();
                                }
                                KeyCode::Enter => {
                                    // Commit and exit modify mode
                                    self.page.enter_normal_mode();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Drop for JournalEntryPageState {
    fn drop(&mut self) {
        // Unregister both presenters from registry when page is destroyed
        self.registry.unregister_account_master_presenter(self.id);
        self.registry.unregister_journal_entry_presenter(self.id);
        // Channels are automatically cleaned up when dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_returns_journal_entry() {
        let registry = Arc::new(PresenterRegistry::new());
        let state = JournalEntryPageState::new(Arc::clone(&registry));
        assert_eq!(state.route(), Route::JournalEntry);
    }

    #[test]
    fn test_multiple_presenter_registration() {
        let registry = Arc::new(PresenterRegistry::new());
        let count_before = registry.total_count();

        let state = JournalEntryPageState::new(Arc::clone(&registry));

        // Verify both presenters were registered (AccountMaster + JournalEntry)
        assert_eq!(registry.total_count(), count_before + 2);
        assert!(registry.get_account_master_presenter(state.id).is_some());
        assert!(registry.get_journal_entry_presenter(state.id).is_some());
    }

    #[test]
    fn test_multiple_presenter_unregistration_on_drop() {
        let registry = Arc::new(PresenterRegistry::new());
        let count_before = registry.total_count();

        {
            let state = JournalEntryPageState::new(Arc::clone(&registry));
            let state_id = state.id;

            // Verify both presenters are registered
            assert_eq!(registry.total_count(), count_before + 2);
            assert!(registry.get_account_master_presenter(state_id).is_some());
            assert!(registry.get_journal_entry_presenter(state_id).is_some());

            // state goes out of scope here
        }

        // Verify both presenters were unregistered
        assert_eq!(registry.total_count(), count_before);
    }

    #[test]
    fn test_multiple_journal_entry_page_instances() {
        let registry = Arc::new(PresenterRegistry::new());

        let state1 = JournalEntryPageState::new(Arc::clone(&registry));
        let state2 = JournalEntryPageState::new(Arc::clone(&registry));

        // Both should have unique IDs
        assert_ne!(state1.id, state2.id);

        // Both should have 2 presenters each (4 total)
        assert_eq!(registry.total_count(), 4);

        // Verify state1's presenters
        assert!(registry.get_account_master_presenter(state1.id).is_some());
        assert!(registry.get_journal_entry_presenter(state1.id).is_some());

        // Verify state2's presenters
        assert!(registry.get_account_master_presenter(state2.id).is_some());
        assert!(registry.get_journal_entry_presenter(state2.id).is_some());
    }

    #[test]
    fn test_channel_cleanup_on_drop() {
        let registry = Arc::new(PresenterRegistry::new());

        let state1 = JournalEntryPageState::new(Arc::clone(&registry));
        let id1 = state1.id;
        let state2 = JournalEntryPageState::new(Arc::clone(&registry));
        let id2 = state2.id;

        assert_eq!(registry.total_count(), 4); // 2 presenters × 2 states

        // Drop state1
        drop(state1);
        assert_eq!(registry.total_count(), 2);
        assert!(registry.get_account_master_presenter(id1).is_none());
        assert!(registry.get_journal_entry_presenter(id1).is_none());
        assert!(registry.get_account_master_presenter(id2).is_some());
        assert!(registry.get_journal_entry_presenter(id2).is_some());

        // Drop state2
        drop(state2);
        assert_eq!(registry.total_count(), 0);
        assert!(registry.get_account_master_presenter(id2).is_none());
        assert!(registry.get_journal_entry_presenter(id2).is_none());
    }

    #[test]
    fn test_presenter_isolation_between_instances() {
        let registry = Arc::new(PresenterRegistry::new());

        let state1 = JournalEntryPageState::new(Arc::clone(&registry));
        let state2 = JournalEntryPageState::new(Arc::clone(&registry));

        // Get presenters for both states
        let am_presenter1 = registry.get_account_master_presenter(state1.id).unwrap();
        let am_presenter2 = registry.get_account_master_presenter(state2.id).unwrap();
        let je_presenter1 = registry.get_journal_entry_presenter(state1.id).unwrap();
        let je_presenter2 = registry.get_journal_entry_presenter(state2.id).unwrap();

        // Verify they are different instances (no sharing)
        assert!(!Arc::ptr_eq(&am_presenter1, &am_presenter2));
        assert!(!Arc::ptr_eq(&je_presenter1, &je_presenter2));
    }
}
