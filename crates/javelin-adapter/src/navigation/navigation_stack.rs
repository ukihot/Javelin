// NavigationStack - Stack-based navigation manager
// Manages the navigation stack with push/pop operations and lifecycle hooks

use super::PageState;

/// Stack-based navigation manager
///
/// Maintains a stack of page states representing the navigation history.
/// Supports push (forward navigation) and pop (back navigation) operations.
///
/// # Lifecycle Management
///
/// - When pushing a new page, the current page's `on_pause()` is called
/// - When popping a page, the previous page's `on_resume()` is called
/// - This ensures proper state transitions during navigation
///
/// # Example
///
/// ```rust,ignore
/// let mut stack = NavigationStack::new();
/// stack.push(Box::new(HomePageState::new()));
///
/// // Navigate forward
/// stack.push(Box::new(SearchPageState::new()));
///
/// // Navigate back
/// stack.pop();
/// ```
pub struct NavigationStack {
    stack: Vec<Box<dyn PageState>>,
}

impl NavigationStack {
    /// Create a new empty navigation stack
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Push a new page onto the stack
    ///
    /// Calls `on_pause()` on the current page (if any) before pushing.
    /// The new page becomes the active page.
    pub fn push(&mut self, page: Box<dyn PageState>) {
        if let Some(current) = self.stack.last_mut() {
            current.on_pause();
        }
        self.stack.push(page);
    }

    /// Pop the current page and return to previous
    ///
    /// Calls `on_resume()` on the previous page (if any) after popping.
    /// Returns the popped page, or None if the stack is empty.
    pub fn pop(&mut self) -> Option<Box<dyn PageState>> {
        let popped = self.stack.pop();
        if let Some(current) = self.stack.last_mut() {
            current.on_resume();
        }
        popped
    }

    /// Get a mutable reference to the current page (top of stack)
    ///
    /// Returns None if the stack is empty.
    pub fn current(&mut self) -> Option<&mut Box<dyn PageState>> {
        self.stack.last_mut()
    }

    /// Check if the stack is empty
    ///
    /// An empty stack typically indicates the application should exit.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Get the current stack depth
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}

impl Default for NavigationStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::navigation::{Controllers, NavAction, Route};

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
    #[test]
    fn property_navigation_stack_maintains_history() {
        let mut stack = NavigationStack::new();

        stack.push(Box::new(MockPageState::new(Route::Home)));
        stack.push(Box::new(MockPageState::new(Route::Search)));
        stack.push(Box::new(MockPageState::new(Route::JournalEntry)));

        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);

        let popped3 = stack.pop().unwrap();
        assert_eq!(popped3.route(), Route::JournalEntry);

        let popped2 = stack.pop().unwrap();
        assert_eq!(popped2.route(), Route::Search);

        let popped1 = stack.pop().unwrap();
        assert_eq!(popped1.route(), Route::Home);

        assert!(stack.is_empty());
        assert!(stack.pop().is_none());
    }

    /// Property 2: Back Navigation Returns to Previous Screen
    #[test]
    fn property_back_navigation_returns_to_previous_screen() {
        let mut stack = NavigationStack::new();

        stack.push(Box::new(MockPageState::new(Route::Home)));
        stack.push(Box::new(MockPageState::new(Route::Search)));
        stack.push(Box::new(MockPageState::new(Route::JournalEntry)));

        assert_eq!(stack.current().unwrap().route(), Route::JournalEntry);
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Search);
        stack.pop();
        assert_eq!(stack.current().unwrap().route(), Route::Home);
    }

    /// Property 10: Pause Notification on Navigation Away
    #[test]
    fn property_pause_notification_on_navigation_away() {
        let mut stack = NavigationStack::new();

        let page1 = Box::new(MockPageState::new(Route::Home));
        let pause_count = page1.pause_count.clone();

        stack.push(page1);
        assert_eq!(*pause_count.lock().unwrap(), 0);

        stack.push(Box::new(MockPageState::new(Route::Search)));
        assert_eq!(*pause_count.lock().unwrap(), 1);

        stack.push(Box::new(MockPageState::new(Route::JournalEntry)));
        assert_eq!(*pause_count.lock().unwrap(), 1);
    }

    /// Property 11: Resume Notification on Navigation Back
    #[test]
    fn property_resume_notification_on_navigation_back() {
        let mut stack = NavigationStack::new();

        let page1 = Box::new(MockPageState::new(Route::Home));
        let resume_count = page1.resume_count.clone();

        stack.push(page1);
        assert_eq!(*resume_count.lock().unwrap(), 0);

        stack.push(Box::new(MockPageState::new(Route::Search)));
        assert_eq!(*resume_count.lock().unwrap(), 0);

        stack.pop();
        assert_eq!(*resume_count.lock().unwrap(), 1);
    }

    /// Property: Multiple Push/Pop Cycles
    #[test]
    fn property_multiple_push_pop_cycles() {
        let mut stack = NavigationStack::new();

        let page1 = Box::new(MockPageState::new(Route::Home));
        let pause_count = page1.pause_count.clone();
        let resume_count = page1.resume_count.clone();

        stack.push(page1);

        stack.push(Box::new(MockPageState::new(Route::Search)));
        assert_eq!(*pause_count.lock().unwrap(), 1);
        stack.pop();
        assert_eq!(*resume_count.lock().unwrap(), 1);

        stack.push(Box::new(MockPageState::new(Route::JournalEntry)));
        assert_eq!(*pause_count.lock().unwrap(), 2);
        stack.pop();
        assert_eq!(*resume_count.lock().unwrap(), 2);
    }

    /// Property: Empty Stack Behavior
    #[test]
    fn property_empty_stack_behavior() {
        let mut stack = NavigationStack::new();

        assert!(stack.is_empty());
        assert!(stack.current().is_none());
        assert!(stack.pop().is_none());

        stack.push(Box::new(MockPageState::new(Route::Home)));
        assert!(!stack.is_empty());

        stack.pop();
        assert!(stack.is_empty());
        assert!(stack.current().is_none());
    }
}
