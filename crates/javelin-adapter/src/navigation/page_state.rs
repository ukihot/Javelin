// PageState trait - Per-screen state management
// Each screen implements this trait to manage its own state and event loop

use ratatui::DefaultTerminal;

use super::{Controllers, NavAction, Route};
use crate::error::AdapterResult;

/// Per-screen state management trait
///
/// Each screen implements this trait to:
/// - Own its state and resources (channels, data)
/// - Run its event loop and return navigation intent
/// - Handle lifecycle events (pause/resume)
///
/// # Lifecycle
///
/// 1. **Creation**: PageState is created when navigating to a screen
/// 2. **Active**: `run()` is called to execute the event loop
/// 3. **Pause**: `on_pause()` is called when navigating away
/// 4. **Resume**: `on_resume()` is called when navigating back
/// 5. **Destruction**: PageState is dropped when permanently removed from stack
///
/// # Example
///
/// ```rust,ignore
/// struct MyPageState {
///     data: String,
/// }
///
/// impl PageState for MyPageState {
///     fn route(&self) -> Route {
///         Route::MyScreen
///     }
///     
///     fn run(
///         &mut self,
///         terminal: &mut DefaultTerminal,
///         controllers: &Controllers,
///     ) -> AdapterResult<NavAction> {
///         // Event loop logic here
///         // Return NavAction::Back when user presses Esc
///         Ok(NavAction::None)
///     }
/// }
/// ```
pub trait PageState: Send {
    /// Get the route this state belongs to
    fn route(&self) -> Route;

    /// Run the page event loop and return navigation action
    ///
    /// This method contains the page's event loop logic.
    /// It should:
    /// - Render the page UI
    /// - Handle user input
    /// - Update page state
    /// - Return navigation intent (Go/Back/None)
    ///
    /// The method returns when navigation is requested or an error occurs.
    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction>;

    /// Called when navigating back to this screen
    ///
    /// Use this to resume operations that were paused,
    /// refresh data, or update UI state.
    fn on_resume(&mut self) {}

    /// Called when navigating away from this screen
    ///
    /// Use this to pause operations, save state,
    /// or clean up temporary resources.
    fn on_pause(&mut self) {}

    /// Called when a navigation error occurs
    ///
    /// Use this to display error messages in the page's event log
    /// instead of printing to stderr which breaks the TUI.
    fn on_navigation_error(&mut self, _error_message: &str) {}
}
