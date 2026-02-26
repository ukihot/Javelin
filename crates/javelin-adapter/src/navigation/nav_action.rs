// NavAction - Navigation intent returned by pages
// Pages express navigation intent without implementing navigation logic

use super::Route;

/// Navigation action returned by page event loops
///
/// Pages return navigation intent instead of directly transitioning.
/// The application layer interprets these actions and performs navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavAction {
    /// Navigate to a specific route
    ///
    /// Pushes a new page onto the navigation stack.
    /// The previous page is paused and the new page becomes active.
    Go(Route),

    /// Navigate back to the previous screen
    ///
    /// Pops the current page from the navigation stack.
    /// The previous page is resumed. If the stack becomes empty,
    /// the application exits.
    Back,

    /// Stay on the current screen
    ///
    /// No navigation occurs. The current page continues running.
    /// This is the default action when no navigation is requested.
    None,
}
