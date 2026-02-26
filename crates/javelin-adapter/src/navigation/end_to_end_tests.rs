// End-to-end integration tests for navigation system
// Task 12.1: Write end-to-end integration tests
// Tests complete navigation flows and state preservation

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        navigation::{NavigationStack, PresenterRegistry, Route},
        page_states::{HomePageState, JournalEntryPageState, LedgerPageState, SearchPageState},
    };

    #[test]
    fn test_complete_navigation_flow_home_search_back() {
        // Test: Home → Search → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Start at Home
        stack.push(Box::new(HomePageState::new()));
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0);

        // Navigate to Search
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::Search);
        assert_eq!(registry.total_count(), 2); // SearchPageState registers 2 presenters

        // Navigate back to Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0); // Search presenters cleaned up
    }

    #[test]
    fn test_complete_navigation_flow_home_journal_entry_back() {
        // Test: Home → JournalEntry → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Start at Home
        stack.push(Box::new(HomePageState::new()));
        assert_eq!(stack.current().unwrap().route(), Route::Home);

        // Navigate to JournalEntry
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 2); // AccountMaster + JournalEntry presenters

        // Navigate back to Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0); // Both presenters cleaned up
    }

    #[test]
    fn test_nested_navigation_three_levels() {
        // Test: Home → Search → JournalEntry → Back → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Level 1: Home
        stack.push(Box::new(HomePageState::new()));
        assert_eq!(stack.current().unwrap().route(), Route::Home);

        // Level 2: Search
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::Search);
        assert_eq!(registry.total_count(), 2); // SearchPageState registers 2 presenters

        // Level 3: JournalEntry
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        assert_eq!(registry.total_count(), 4); // 2 Search + 2 JournalEntry

        // Back to Search
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Search);
        assert_eq!(registry.total_count(), 2); // JournalEntry presenters cleaned up

        // Back to Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0); // Search presenter cleaned up
    }

    #[test]
    fn test_nested_navigation_four_levels() {
        // Test: Home → Search → JournalEntry → Ledger → Back → Back → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Build up the stack
        stack.push(Box::new(HomePageState::new()));
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        stack.push(Box::new(LedgerPageState::new()));

        assert_eq!(stack.current().unwrap().route(), Route::Ledger);
        assert_eq!(registry.total_count(), 4); // Search (2) + JournalEntry (2)

        // Navigate back through all levels
        stack.pop(); // Ledger → JournalEntry
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);

        stack.pop(); // JournalEntry → Search
        assert_eq!(stack.current().unwrap().route(), Route::Search);
        assert_eq!(registry.total_count(), 2); // SearchPageState registers 2 presenters

        stack.pop(); // Search → Home
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_complex_navigation_pattern() {
        // Test: Home → Search → Back → JournalEntry → Back → Search → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Start at Home
        stack.push(Box::new(HomePageState::new()));

        // Home → Search
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::Search);
        assert_eq!(registry.total_count(), 2); // SearchPageState registers 2 presenters

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

        // Home → Search (again)
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::Search);
        assert_eq!(registry.total_count(), 2); // SearchPageState registers 2 presenters

        // Search → Back → Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_state_preservation_across_navigation() {
        // Test that state is preserved when navigating back
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Create Home page
        let home = Box::new(HomePageState::new());
        stack.push(home);

        // Navigate to Search
        let search = Box::new(SearchPageState::new(Arc::clone(&registry)));
        stack.push(search);

        // Navigate to JournalEntry
        let je = Box::new(JournalEntryPageState::new(Arc::clone(&registry)));
        stack.push(je);

        // Navigate back to Search
        stack.pop();

        // The Search page should still be there (state preserved)
        assert_eq!(stack.current().unwrap().route(), Route::Search);

        // Navigate back to Home
        stack.pop();

        // The Home page should still be there (state preserved)
        assert_eq!(stack.current().unwrap().route(), Route::Home);
    }

    #[test]
    fn test_multiple_instances_of_same_screen_type() {
        // Test: Home → Search1 → Search2 → Back → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        stack.push(Box::new(HomePageState::new()));

        // First Search instance
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::Search);
        assert_eq!(registry.total_count(), 2); // SearchPageState registers 2 presenters

        // Second Search instance
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        assert_eq!(stack.current().unwrap().route(), Route::Search);
        assert_eq!(registry.total_count(), 4); // Two separate instances (2 + 2)

        // Back to first Search
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Search);
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
            Route::Search,
            Route::JournalEntry,
            Route::Ledger,
            Route::Search,
            Route::JournalEntry,
            Route::Search,
            Route::Ledger,
            Route::Search,
            Route::JournalEntry,
        ];

        for route in &routes {
            match route {
                Route::Home => stack.push(Box::new(HomePageState::new())),
                Route::Search => stack.push(Box::new(SearchPageState::new(Arc::clone(&registry)))),
                Route::JournalEntry => {
                    stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))))
                }
                Route::Ledger => stack.push(Box::new(LedgerPageState::new())),
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

        // Search: 2 presenters (SearchPresenter + AccountMasterPresenter)
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        assert_eq!(registry.total_count(), 2);

        // JournalEntry: 2 presenters
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        assert_eq!(registry.total_count(), 4); // 2 + 2

        // Ledger: 0 presenters
        stack.push(Box::new(LedgerPageState::new()));
        assert_eq!(registry.total_count(), 4); // Still 2 + 2

        // Back to JournalEntry
        stack.pop();
        assert_eq!(registry.total_count(), 4);

        // Back to Search
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

            // Home → Search
            stack.push(Box::new(HomePageState::new()));
            stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
            assert_eq!(registry.total_count(), 2); // SearchPageState registers 2 presenters

            // Search → Back
            stack.pop();
            assert_eq!(registry.total_count(), 0);

            // Stack is dropped here
        }

        // Memory should be stable
        assert_eq!(registry.total_count(), 0);
    }
}
