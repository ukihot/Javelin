// End-to-end integration tests for navigation system

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        navigation::{NavigationStack, PresenterRegistry, Route},
        page_states::{
            HomePageState, JournalEntryPageState, StubPageState,
            maintenance_home_page_state::MaintenanceHomePageState,
            maintenance_menu_page_state::MaintenanceMenuPageState,
        },
    };

    #[test]
    fn test_maintenance_mode_navigation() {
        // Test that maintenance mode stack begins at MaintenanceHome and can go to menu
        let mut stack = NavigationStack::new();
        stack.push(Box::new(MaintenanceHomePageState::new()));
        assert_eq!(stack.current().unwrap().route(), Route::MaintenanceHome);

        // simulate Enter -> MaintenanceMenu
        stack.push(Box::new(MaintenanceMenuPageState::new()));
        assert_eq!(stack.current().unwrap().route(), Route::MaintenanceMenu);

        // choose first item
        stack.push(Box::new(StubPageState::new(
            Route::MaintenanceRebuildProjections,
            "Maintenance: Rebuild Projections",
            "Trigger projection rebuild",
        )));
        assert_eq!(stack.current().unwrap().route(), Route::MaintenanceRebuildProjections);

        // back to menu
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::MaintenanceMenu);

        // back to home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::MaintenanceHome);
    }

    #[test]
    fn test_complete_navigation_flow_home_journal_entry_back() {
        // Test: Home → JournalEntry → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Start at Home
        stack.push(Box::new(HomePageState::new()));
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0);

        // Navigate to JournalEntry
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 2); // JournalEntry registers 2 presenters

        // Navigate back to Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0); // presenters cleaned up
    }

    #[test]
    fn test_nested_navigation_three_levels() {
        // Test: Home → JournalEntry → JournalEntry → Back → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Level 1: Home
        stack.push(Box::new(HomePageState::new()));
        assert_eq!(stack.current().unwrap().route(), Route::Home);

        // Level 2: JournalEntry
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 2); // JournalEntry registers 2 presenters

        // Level 3: another JournalEntry
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 4); // two JournalEntry instances

        // Back to first JournalEntry
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 2); // one JournalEntry cleaned up

        // Back to Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0); // JournalEntry presenter cleaned up
    }

    #[test]
    fn test_nested_navigation_four_levels() {
        // Test: Home → JournalEntry → JournalEntry → LedgerMenu → Back → Back → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Build up the stack
        stack.push(Box::new(HomePageState::new()));
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        stack.push(Box::new(StubPageState::new(Route::LedgerMenu, "Ledger Menu", "元帳メニュー")));

        assert_eq!(stack.current().unwrap().route(), Route::LedgerMenu);
        assert_eq!(registry.total_count(), 4); // two JournalEntry instances

        // Navigate back through all levels
        stack.pop(); // LedgerMenu → JournalEntry
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);

        stack.pop(); // JournalEntry → JournalEntry
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 2); // one JournalEntry remains

        stack.pop(); // JournalEntry → Home
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_complex_navigation_pattern() {
        // Test: Home → JournalEntry → Back → JournalEntry → Back → JournalEntry → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Start at Home
        stack.push(Box::new(HomePageState::new()));

        // Home → JournalEntry
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 2); // JournalEntry registers 2 presenters

        // Search → Back → Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0);

        // Home → JournalEntry
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 2);

        // JournalEntry → Back → Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0);

        // Home → JournalEntry (again)
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 2); // JournalEntry registers 2 presenters

        // Search → Back → Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_state_preservation_across_navigation() {
        // Test that state is preserved when navigating back (using journal entries)
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Create Home page
        let home = Box::new(HomePageState::new());
        stack.push(home);

        // Navigate to JournalEntry
        let je1 = Box::new(JournalEntryPageState::new(Arc::clone(&registry)));
        stack.push(je1);

        // Navigate to another JournalEntry
        let je = Box::new(JournalEntryPageState::new(Arc::clone(&registry)));
        stack.push(je);

        // Navigate back to previous JE
        stack.pop();

        // The JournalEntry page should still be there (state preserved)
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);

        // Navigate back to Home
        stack.pop();

        // The Home page should still be there (state preserved)
        assert_eq!(stack.current().unwrap().route(), Route::Home);
    }

    #[test]
    fn test_multiple_instances_of_same_screen_type() {
        // Test: Home → JE1 → JE2 → Back → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        stack.push(Box::new(HomePageState::new()));

        // First JournalEntry instance
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 2); // JournalEntry registers 2 presenters

        // Second JournalEntry instance
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 4); // Two separate instances

        // Back to first JournalEntry
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 2); // Second instance cleaned up

        // Back to Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0); // First instance cleaned up
    }

    #[test]
    fn test_deep_navigation_stack() {
        // Test navigation with 10+ levels
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Build a deep stack
        let routes = vec![
            Route::Home,
            Route::JournalEntry,
            Route::JournalEntry,
            Route::LedgerMenu,
            Route::JournalEntry,
            Route::JournalEntry,
            Route::JournalEntry,
            Route::LedgerMenu,
            Route::JournalEntry,
            Route::JournalEntry,
        ];

        for route in &routes {
            match route {
                Route::Home => stack.push(Box::new(HomePageState::new())),
                Route::JournalEntry => {
                    stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))))
                }
                Route::LedgerMenu => stack.push(Box::new(StubPageState::new(
                    Route::LedgerMenu,
                    "Ledger Menu",
                    "元帳メニュー",
                ))),
                _ => {}
            }
        }

        // Verify we're at the top
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);

        // Pop back through all levels
        for route in routes.iter().rev().skip(1) {
            stack.pop();
            if !stack.is_empty() {
                assert_eq!(stack.current().unwrap().route(), *route);
            }
        }

        // Should be back at Home
        assert!(!stack.is_empty());
        assert_eq!(stack.current().unwrap().route(), Route::Home);
    }

    #[test]
    fn test_navigation_with_mixed_presenter_counts() {
        // Test navigation with screens that have different numbers of presenters
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Home: 0 presenters
        stack.push(Box::new(HomePageState::new()));
        assert_eq!(registry.total_count(), 0);

        // JournalEntry: 2 presenters
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(registry.total_count(), 2);

        // JournalEntry: 2 presenters
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(registry.total_count(), 4); // 2 + 2

        // LedgerMenu: 0 presenters
        stack.push(Box::new(StubPageState::new(Route::LedgerMenu, "Ledger Menu", "元帳メニュー")));
        assert_eq!(registry.total_count(), 4); // Still 2 + 2

        // Back to JournalEntry
        stack.pop();
        assert_eq!(registry.total_count(), 4);

        // Back to JournalEntry
        stack.pop();
        assert_eq!(registry.total_count(), 2);

        // Back to Home
        stack.pop();
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_exit_application_flow() {
        // Test: Home → (user presses 'q') → empty stack → exit
        let mut stack = NavigationStack::new();

        // Start at Home
        stack.push(Box::new(HomePageState::new()));
        assert!(!stack.is_empty());

        // User presses 'q' (simulated by popping Home)
        stack.pop();

        // Stack is empty, application should exit
        assert!(stack.is_empty());
        assert!(stack.current().is_none());
    }

    #[test]
    fn test_repeated_navigation_cycles() {
        // Test: Repeat (Home → Search → Back) 100 times
        let registry = Arc::new(PresenterRegistry::new());

        for _ in 0..100 {
            let mut stack = NavigationStack::new();

            // Home → JournalEntry
            stack.push(Box::new(HomePageState::new()));
            stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
            assert_eq!(registry.total_count(), 2); // JournalEntry registers 2 presenters

            // Search → Back
            stack.pop();
            assert_eq!(registry.total_count(), 0);

            // Stack is dropped here
        }

        // Memory should be stable
        assert_eq!(registry.total_count(), 0);
    }
}
