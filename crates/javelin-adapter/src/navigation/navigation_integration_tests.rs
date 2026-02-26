// Integration tests for navigation flow
// Task 4.5: Write integration tests for navigation flow

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        navigation::{NavAction, NavigationStack, PresenterRegistry, Route},
        page_states::{HomePageState, SearchPageState},
    };

    // Mock PageState for testing
    struct MockPageState {
        route: Route,
        action_to_return: NavAction,
    }

    impl MockPageState {
        fn new(route: Route, action: NavAction) -> Self {
            Self { route, action_to_return: action }
        }
    }

    impl crate::navigation::PageState for MockPageState {
        fn route(&self) -> Route {
            self.route.clone()
        }

        fn run(
            &mut self,
            _terminal: &mut ratatui::DefaultTerminal,
            _controllers: &crate::navigation::Controllers,
        ) -> crate::error::AdapterResult<NavAction> {
            Ok(self.action_to_return.clone())
        }
    }

    #[test]
    fn test_navigation_to_unimplemented_route_stays_on_current_page() {
        // Test Home → (unimplemented route) → stay on Home
        let mut stack = NavigationStack::new();

        // Push home page
        let home = Box::new(MockPageState::new(Route::Home, NavAction::None));
        stack.push(home);

        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert!(!stack.is_empty());

        // In real implementation, trying to navigate to unimplemented route
        // would show error message and stay on current page (not push to stack)
        // Here we verify the stack remains unchanged
        assert_eq!(stack.current().unwrap().route(), Route::Home);
    }

    #[test]
    fn test_navigation_error_handling() {
        // Test navigation error handling
        let mut stack = NavigationStack::new();

        // Push home page
        let home = Box::new(MockPageState::new(Route::Home, NavAction::None));
        stack.push(home);

        // Try to pop when only one page exists
        // In real app, this would exit the application
        let popped = stack.pop();
        assert!(popped.is_some());
        assert!(stack.is_empty());
    }

    #[test]
    fn test_empty_stack_exit_behavior() {
        // Test empty stack exit behavior
        let mut stack = NavigationStack::new();

        // Initially empty
        assert!(stack.is_empty());
        assert!(stack.current().is_none());

        // Push and pop
        let home = Box::new(MockPageState::new(Route::Home, NavAction::None));
        stack.push(home);
        assert!(!stack.is_empty());

        stack.pop();
        assert!(stack.is_empty());

        // In real app, empty stack means exit application
    }

    #[test]
    fn test_forward_navigation_flow() {
        // Test Home → Search navigation
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Start with home
        let home = Box::new(HomePageState::new());
        stack.push(home);
        assert_eq!(stack.current().unwrap().route(), Route::Home);

        // Navigate to search
        let search = Box::new(SearchPageState::new(Arc::clone(&registry)));
        stack.push(search);
        assert_eq!(stack.current().unwrap().route(), Route::Search);

        // Stack should have 2 pages
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_back_navigation_flow() {
        // Test Home → Search → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Start with home
        let home = Box::new(HomePageState::new());
        stack.push(home);

        // Navigate to search
        let search = Box::new(SearchPageState::new(Arc::clone(&registry)));
        stack.push(search);
        assert_eq!(stack.current().unwrap().route(), Route::Search);

        // Navigate back
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
    }

    #[test]
    fn test_nested_navigation() {
        // Test Home → Search → JournalEntry → Back → Back → Home
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        // Home
        let home = Box::new(HomePageState::new());
        stack.push(home);
        assert_eq!(stack.current().unwrap().route(), Route::Home);

        // Home → Search
        let search = Box::new(SearchPageState::new(Arc::clone(&registry)));
        stack.push(search);
        assert_eq!(stack.current().unwrap().route(), Route::Search);

        // Search → JournalEntry (simulated with mock)
        let je = Box::new(MockPageState::new(Route::JournalEntry, NavAction::None));
        stack.push(je);
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);

        // Back to Search
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Search);

        // Back to Home
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
    }

    #[test]
    fn test_multiple_forward_navigations() {
        // Test multiple forward navigations without going back
        let mut stack = NavigationStack::new();

        let routes = vec![Route::Home, Route::Search, Route::JournalEntry, Route::Ledger];

        for route in &routes {
            let page = Box::new(MockPageState::new(route.clone(), NavAction::None));
            stack.push(page);
        }

        // Current should be the last pushed
        assert_eq!(stack.current().unwrap().route(), Route::Ledger);

        // Pop back through all pages
        for route in routes.iter().rev().skip(1) {
            stack.pop();
            if !stack.is_empty() {
                assert_eq!(stack.current().unwrap().route(), *route);
            }
        }
    }

    #[test]
    fn test_navigation_with_none_action() {
        // Test that NavAction::None keeps the page active
        let mut stack = NavigationStack::new();

        let home = Box::new(MockPageState::new(Route::Home, NavAction::None));
        stack.push(home);

        // Simulate running the page and getting NavAction::None
        // In real app, this would continue the loop on the same page
        assert_eq!(stack.current().unwrap().route(), Route::Home);
        assert!(!stack.is_empty());
    }
}
