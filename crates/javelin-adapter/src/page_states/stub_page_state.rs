// StubPageState - Placeholder page state for not-yet-implemented screens
// Displays a simple message and allows navigation back

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
};

/// Stub page state for not-yet-implemented screens
///
/// Displays a message indicating the screen is not yet implemented
/// and allows the user to navigate back.
pub struct StubPageState {
    route: Route,
    title: String,
    message: String,
}

impl StubPageState {
    /// Create a new stub page state
    pub fn new(route: Route, title: &str, message: &str) -> Self {
        Self { route, title: title.to_string(), message: message.to_string() }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(0),    // Message
                Constraint::Length(3), // Help
            ])
            .split(area);

        // Title
        let title = Paragraph::new(self.title.as_str())
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        // Message
        let message = Paragraph::new(vec![
            Line::from(""),
            Line::from(self.message.as_str()),
            Line::from(""),
            Line::from("This screen is not yet implemented."),
            Line::from("Press Esc to go back."),
        ])
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(message, chunks[1]);

        // Help
        let help = Paragraph::new("Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(help, chunks[2]);
    }
}

impl PageState for StubPageState {
    fn route(&self) -> Route {
        self.route.clone()
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            terminal
                .draw(|frame| {
                    self.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            if let Event::Key(key) =
                event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.code == KeyCode::Esc {
                    return Ok(NavAction::Back);
                }
            }
        }
    }
}
