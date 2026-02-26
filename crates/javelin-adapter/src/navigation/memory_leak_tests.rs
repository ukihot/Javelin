// Memory leak tests for navigation system
// Task 12.2: Write memory leak tests
// Validates Properties 5, 22 and Requirements 11.1-11.5

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        PageState,
        navigation::{NavigationStack, PresenterRegistry},
        page_states::{HomePageState, JournalEntryPageState, SearchPageState},
    };

    #[test]
    fn test_repeated_navigation_no_memory_accumulation() {
        // Test that repeated navigation doesn't accumulate memory
        let registry = Arc::new(PresenterRegistry::new());

        // Perform 1000 navigation cycles
        for _ in 0..1000 {
            {
                let _page = SearchPageState::new(Arc::clone(&registry));
                // Page is created and immediately dropped
            }

            // Verify no presenters are left behind
            assert_eq!(registry.total_count(), 0);
        }

        // After 1000 cycles, memory should be clean
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_channel_cleanup_after_navigation() {
        // Test that channels are cleaned up after navigation
        let registry = Arc::new(PresenterRegistry::new());

        // Create a page with channels
        let page_id = {
            let page = SearchPageState::new(Arc::clone(&registry));
            let id = page.route(); // Get some identifier

            // Verify presenters are registered (SearchPresenter + AccountMasterPresenter)
            assert_eq!(registry.total_count(), 2);

            id
        }; // page is dropped here

        // Verify presenters are unregistered (channels cleaned up)
        assert_eq!(registry.total_count(), 0);

        // Verify we can still use the route (it's just an enum)
        assert_eq!(page_id, crate::navigation::Route::Search);
    }

    #[test]
    fn test_presenter_cleanup_after_navigation() {
        // Test that presenters are cleaned up after navigation
        let registry = Arc::new(PresenterRegistry::new());

        // Create multiple pages
        let ids = {
            let page1 = SearchPageState::new(Arc::clone(&registry));
            let page2 = JournalEntryPageState::new(Arc::clone(&registry));
            let page3 = SearchPageState::new(Arc::clone(&registry));

            // Should have 6 presenters (2 + 2 + 2)
            assert_eq!(registry.total_count(), 6);

            vec![page1.route(), page2.route(), page3.route()]
        }; // All pages dropped here

        // All presenters should be cleaned up
        assert_eq!(registry.total_count(), 0);

        // Routes should still be valid
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn test_stack_cleanup_on_pop() {
        // Test that popping from stack cleans up resources
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Push multiple pages
        stack.push(Box::new(HomePageState::new()));
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));

        // Should have 4 presenters (2 Search + 2 JournalEntry)
        assert_eq!(registry.total_count(), 4);

        // Pop JournalEntry (2 presenters)
        stack.pop();
        assert_eq!(registry.total_count(), 2); // Search presenters remain

        // Pop Search (2 presenters)
        stack.pop();
        assert_eq!(registry.total_count(), 0);

        // Pop Home (0 presenters)
        stack.pop();
        assert_eq!(registry.total_count(), 0);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_multiple_instances_independent_cleanup() {
        // Test that multiple instances clean up independently
        let registry = Arc::new(PresenterRegistry::new());

        let page1 = SearchPageState::new(Arc::clone(&registry));
        let page2 = SearchPageState::new(Arc::clone(&registry));
        let page3 = SearchPageState::new(Arc::clone(&registry));

        // Each SearchPageState registers 2 presenters (SearchPresenter + AccountMasterPresenter)
        assert_eq!(registry.total_count(), 6);

        // Drop page2
        drop(page2);
        assert_eq!(registry.total_count(), 4);

        // Drop page1
        drop(page1);
        assert_eq!(registry.total_count(), 2);

        // Drop page3
        drop(page3);
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_nested_navigation_cleanup() {
        // Test cleanup with deep navigation stack
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Build a deep stack (10 levels)
        stack.push(Box::new(HomePageState::new()));
        for i in 0..9 {
            if i % 2 == 0 {
                stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
            } else {
                stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
            }
        }

        // Count presenters: 5 Search (2 each) + 4 JournalEntry (2 each) = 18
        assert_eq!(registry.total_count(), 18);

        // Pop all pages
        for _ in 0..9 {
            stack.pop();
        }

        // All presenters should be cleaned up
        assert_eq!(registry.total_count(), 0);

        // Only Home remains
        assert!(!stack.is_empty());
        assert_eq!(stack.current().unwrap().route(), crate::navigation::Route::Home);
    }

    #[test]
    fn test_rapid_navigation_cycles() {
        // Test rapid creation and destruction cycles
        let registry = Arc::new(PresenterRegistry::new());

        for _ in 0..100 {
            // Create and immediately drop
            let _page1 = SearchPageState::new(Arc::clone(&registry));
            let _page2 = JournalEntryPageState::new(Arc::clone(&registry));
            let _page3 = SearchPageState::new(Arc::clone(&registry));

            // All should be registered (2 Search + 2 JournalEntry + 2 Search = 6)
            assert_eq!(registry.total_count(), 6);
        } // All dropped at end of loop

        // All should be cleaned up
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_partial_stack_cleanup() {
        // Test that partial stack cleanup works correctly
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Push 5 pages
        stack.push(Box::new(HomePageState::new()));
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        stack.push(Box::new(SearchPageState::new(Arc::clone(&registry))));
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));

        // Should have 8 presenters (2 Search + 2 JournalEntry + 2 Search + 2 JournalEntry)
        assert_eq!(registry.total_count(), 8);

        // Pop 2 pages
        stack.pop(); // JournalEntry (2 presenters)
        stack.pop(); // Search (2 presenters)

        // Should have 4 presenters left
        assert_eq!(registry.total_count(), 4);

        // Remaining pages should still work
        assert_eq!(stack.current().unwrap().route(), crate::navigation::Route::JournalEntry);
    }

    #[test]
    fn test_registry_clear_all() {
        // Test that clear_all properly cleans up all presenters
        let registry = Arc::new(PresenterRegistry::new());

        // Create multiple pages but don't drop them yet
        let _page1 = SearchPageState::new(Arc::clone(&registry));
        let _page2 = JournalEntryPageState::new(Arc::clone(&registry));
        let _page3 = SearchPageState::new(Arc::clone(&registry));

        assert_eq!(registry.total_count(), 6); // 2 + 2 + 2

        // Clear all presenters
        registry.clear_all();

        // All should be cleared
        assert_eq!(registry.total_count(), 0);

        // Note: The pages still exist, but their presenters are unregistered
        // This is useful for testing cleanup behavior
    }

    #[test]
    fn test_no_memory_leak_with_empty_stack_operations() {
        // Test that operations on empty stack don't leak memory
        let mut stack = NavigationStack::new();

        // Try to pop from empty stack
        let result = stack.pop();
        assert!(result.is_none());

        // Try to get current from empty stack
        let current = stack.current();
        assert!(current.is_none());

        // Stack should still be empty
        assert!(stack.is_empty());
    }

    #[test]
    fn test_memory_stability_with_mixed_page_types() {
        // Test memory stability with different page types
        let registry = Arc::new(PresenterRegistry::new());

        for _ in 0..50 {
            // Cycle through different page types
            {
                let _home = HomePageState::new(); // No presenters
                assert_eq!(registry.total_count(), 0);
            }

            {
                let _search = SearchPageState::new(Arc::clone(&registry)); // 2 presenters (SearchPresenter + AccountMasterPresenter)
                assert_eq!(registry.total_count(), 2);
            }
            assert_eq!(registry.total_count(), 0);

            {
                let _je = JournalEntryPageState::new(Arc::clone(&registry)); // 2 presenters
                assert_eq!(registry.total_count(), 2);
            }
            assert_eq!(registry.total_count(), 0);
        }

        // Memory should be stable
        assert_eq!(registry.total_count(), 0);
    }
}
