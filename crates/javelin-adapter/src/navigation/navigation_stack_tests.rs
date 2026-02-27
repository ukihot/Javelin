// Property tests for NavigationStack
// Validates: Requirements 1.1, 1.2, 5.3, 5.4

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::sync::{Arc, Mutex};

    // Mock PageState for testing
    struct MockPageState {
        route: Route,
        pause_count: Arc<Mutex<usize>>,
        resume_count: Arc<Mutex<usize>>,
    }

    impl MockPageState {
        fn new(route: Route) -> Self {
            Self {
                route,
                pause_count: Arc::new(Mutex::new(0)),
                resume_count: Arc::new(Mutex::new(0)),
            }
        }

        fn pause_count(&self) -> usize {
            *self.pause_count.lock().unwrap()
        }

        fn resume_count(&self) -> usize {
            *self.resume_count.lock().unwrap()
        }
    }

    impl PageState for MockPageState {
        fn route(&self) -> Route {
            self.route.clone()
        }

        fn run(
            &mut self,
            _terminal: &mut ratatui::DefaultTerminal,
            _controllers: &Controllers,
        ) -> crate::error::AdapterResult<NavAction> {
            Ok(NavAction::None)
        }

        fn on_pause(&mut self) {
            *self.pause_count.lock().unwrap() += 1;
        }

        fn on_resume(&mut self) {
            *self.resume_count.lock().unwrap() += 1;
        }
    }

    /// Property 1: Navigation Stack Maintains History
    /// 
    /// When pages are pushed onto the stack, they should be maintained
    /// in the correct order (LIFO - Last In, First Out).
    #[test]
    fn property_navigation_stack_maintains_history() {
        let mut stack = NavigationStack::new();

        // Push three pages
        let page1 = Box::new(MockPageState::new(Route::Home));
        let page2 = Box::new(MockPageState::new(Route::PrimaryRecordsMenu));
        let page3 = Box::new(MockPageState::new(Route::JournalEntry));

        stack.push(page1);
        stack.push(page2);
        stack.push(page3);

        // Current should be the last pushed (JournalEntry)
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);

        // Pop should return pages in reverse order
        let popped3 = stack.pop().unwrap();
        assert_eq!(popped3.route(), Route::JournalEntry);

        let popped2 = stack.pop().unwrap();
        assert_eq!(popped2.route(), Route::PrimaryRecordsMenu);

        let popped1 = stack.pop().unwrap();
        assert_eq!(popped1.route(), Route::Home);

        // Stack should be empty
        assert!(stack.is_empty());
        assert!(stack.pop().is_none());
    }

    /// Property 2: Back Navigation Returns to Previous Screen
    /// 
    /// When navigating back (pop), the current screen should be
    /// the screen that was active before the last push.
    #[test]
    fn property_back_navigation_returns_to_previous_screen() {
        let mut stack = NavigationStack::new();

        // Push Home -> PrimaryRecordsMenu -> JournalEntry
        stack.push(Box::new(MockPageState::new(Route::Home)));
        stack.push(Box::new(MockPageState::new(Route::PrimaryRecordsMenu)));
        stack.push(Box::new(MockPageState::new(Route::JournalEntry)));

        // Current is JournalEntry
        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);

        // Pop JournalEntry
        stack.pop();

        // Current should now be PrimaryRecordsMenu (previous screen)
        assert_eq!(stack.current().unwrap().route(), Route::PrimaryRecordsMenu);

        // Pop PrimaryRecordsMenu
        stack.pop();

        // Current should now be Home (original screen)
        assert_eq!(stack.current().unwrap().route(), Route::Home);
    }

    /// Property 10: Pause Notification on Navigation Away
    /// 
    /// When navigating away from a screen (pushing a new screen),
    /// the current screen's on_pause() should be called.
    #[test]
    fn property_pause_notification_on_navigation_away() {
        let mut stack = NavigationStack::new();

        let page1 = Box::new(MockPageState::new(Route::Home));
        let pause_count = page1.pause_count.clone();

        stack.push(page1);

        // Initially, pause should not have been called
        assert_eq!(*pause_count.lock().unwrap(), 0);

        // Push another page (navigate away)
        stack.push(Box::new(MockPageState::new(Route::PrimaryRecordsMenu)));

        // on_pause() should have been called on the first page
        assert_eq!(*pause_count.lock().unwrap(), 1);

        // Push another page
        stack.push(Box::new(MockPageState::new(Route::JournalEntry)));

        // First page's pause count should still be 1 (only called once)
        assert_eq!(*pause_count.lock().unwrap(), 1);
    }

    /// Property 11: Resume Notification on Navigation Back
    /// 
    /// When navigating back to a screen (popping the current screen),
    /// the previous screen's on_resume() should be called.
    #[test]
    fn property_resume_notification_on_navigation_back() {
        let mut stack = NavigationStack::new();

        let page1 = Box::new(MockPageState::new(Route::Home));
        let resume_count = page1.resume_count.clone();

        stack.push(page1);

        // Initially, resume should not have been called
        assert_eq!(*resume_count.lock().unwrap(), 0);

        // Push another page (JournalEntry)
        stack.push(Box::new(MockPageState::new(Route::JournalEntry)));

        // Resume should still not have been called
        assert_eq!(*resume_count.lock().unwrap(), 0);

        // Pop the second page (navigate back)
        stack.pop();

        // on_resume() should have been called on the first page
        assert_eq!(*resume_count.lock().unwrap(), 1);
    }

    /// Property: Multiple Push/Pop Cycles
    /// 
    /// Verifies that pause and resume are called correctly
    /// across multiple navigation cycles.
    #[test]
    fn property_multiple_push_pop_cycles() {
        let mut stack = NavigationStack::new();

        let page1 = Box::new(MockPageState::new(Route::Home));
        let pause_count = page1.pause_count.clone();
        let resume_count = page1.resume_count.clone();

        stack.push(page1);

        // Cycle 1: Push and pop a JournalEntry
        stack.push(Box::new(MockPageState::new(Route::JournalEntry)));
        assert_eq!(*pause_count.lock().unwrap(), 1);
        stack.pop();
        assert_eq!(*resume_count.lock().unwrap(), 1);

        // Cycle 2: Push and pop another JournalEntry
        stack.push(Box::new(MockPageState::new(Route::JournalEntry)));
        assert_eq!(*pause_count.lock().unwrap(), 2);
        stack.pop();
        assert_eq!(*resume_count.lock().unwrap(), 2);

        // Cycle 3: push home to ensure transition works
        stack.push(Box::new(MockPageState::new(Route::Home)));
        assert_eq!(*pause_count.lock().unwrap(), 3);
        stack.pop();
        assert_eq!(*resume_count.lock().unwrap(), 3);
    }

    /// Property: Empty Stack Behavior
    /// 
    /// Verifies that operations on an empty stack behave correctly.
    #[test]
    fn property_empty_stack_behavior() {
        let mut stack = NavigationStack::new();

        // Empty stack
        assert!(stack.is_empty());
        assert!(stack.current().is_none());
        assert!(stack.pop().is_none());

        // Push one page
        stack.push(Box::new(MockPageState::new(Route::Home)));
        assert!(!stack.is_empty());

        // Pop the page
        stack.pop();
        assert!(stack.is_empty());
        assert!(stack.current().is_none());
    }
}
