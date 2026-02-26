// LoadingSpinner - ローディングアニメーション
// 責務: データ取得中の表示

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub struct LoadingSpinner {
    frame_count: usize,
}

impl LoadingSpinner {
    pub fn new() -> Self {
        Self { frame_count: 0 }
    }

    pub fn tick(&mut self) {
        self.frame_count = (self.frame_count + 1) % 8;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, message: &str) {
        let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"];
        let spinner = spinner_chars[self.frame_count];

        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    spinner,
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ", Style::default()),
                Span::styled(message, Style::default().fg(Color::Gray)),
            ]),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title("読込中"),
        );

        frame.render_widget(paragraph, area);
    }
}

impl Default for LoadingSpinner {
    fn default() -> Self {
        Self::new()
    }
}
