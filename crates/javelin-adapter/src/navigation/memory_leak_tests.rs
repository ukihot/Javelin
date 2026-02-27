// Memory leak tests for navigation system
// Simplified after removal of SearchPageState

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        navigation::{NavigationStack, PresenterRegistry},
        page_states::{HomePageState, JournalEntryPageState},
    };

    #[test]
    fn repeated_journal_entry_navigation_does_not_leak() {
        let registry = Arc::new(PresenterRegistry::new());

        for _ in 0..1000 {
            {
                let _page = JournalEntryPageState::new(Arc::clone(&registry));
            }
            assert_eq!(registry.total_count(), 0);
        }

        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn stack_cleanup_on_pop_removes_presenters() {
        let mut stack = NavigationStack::new();
        let registry = Arc::new(PresenterRegistry::new());

        stack.push(Box::new(HomePageState::new()));
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));
        stack.push(Box::new(JournalEntryPageState::new(Arc::clone(&registry))));

        assert_eq!(registry.total_count(), 4);

        stack.pop();
        assert_eq!(registry.total_count(), 2);

        stack.pop();
        assert_eq!(registry.total_count(), 0);

        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn presenter_registry_clears_all() {
        let registry = Arc::new(PresenterRegistry::new());
        let _page1 = JournalEntryPageState::new(Arc::clone(&registry));
        let _page2 = JournalEntryPageState::new(Arc::clone(&registry));

        assert_eq!(registry.total_count(), 4);
        registry.clear_all();
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn empty_stack_operations_are_safe() {
        let mut stack = NavigationStack::new();
        assert!(stack.pop().is_none());
        assert!(stack.current().is_none());
        assert!(stack.is_empty());
    }
}
