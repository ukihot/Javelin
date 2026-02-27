// MaintenanceHomePageState - PageState for maintenance-mode top
// Minimal page that informs user the app is in maintenance mode and allows Back

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
};

pub struct MaintenanceHomePageState {
    title: String,
    message: String,
}

impl Default for MaintenanceHomePageState {
    fn default() -> Self {
        Self::new()
    }
}

impl MaintenanceHomePageState {
    pub fn new() -> Self {
        Self {
            title: "Maintenance Mode".to_string(),
            message: "This application is running in maintenance mode.".to_string(),
        }
    }

    fn render(&self, frame: &mut Frame) {
        use ratatui::{
            layout::{Constraint, Direction, Layout},
            style::{Color, Modifier, Style},
            text::Line,
            widgets::{Block, Borders, Paragraph},
        };

        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        let title = Paragraph::new(self.title.as_str())
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        let msg = Paragraph::new(vec![
            Line::from(""),
            Line::from(self.message.as_str()),
            Line::from(""),
            Line::from("Press Esc to exit."),
        ])
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(msg, chunks[1]);

        let help = Paragraph::new("Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(help, chunks[2]);
    }
}

impl PageState for MaintenanceHomePageState {
    fn route(&self) -> Route {
        Route::MaintenanceHome
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

                match key.code {
                    KeyCode::Esc => return Ok(NavAction::Back),
                    KeyCode::Enter => return Ok(NavAction::Go(Route::MaintenanceMenu)),
                    _ => {}
                }
            }
        }
    }
}
