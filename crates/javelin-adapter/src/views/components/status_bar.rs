// StatusBar - ステータスバーコンポーネント
// 責務: ステータス情報の表示

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct StatusBar {
    message: String,
    status_type: StatusType,
}

pub enum StatusType {
    Info,
    Success,
    Warning,
    Error,
}

impl StatusBar {
    pub fn new(message: impl Into<String>, status_type: StatusType) -> Self {
        Self { message: message.into(), status_type }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message, StatusType::Info)
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message, StatusType::Success)
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message, StatusType::Warning)
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message, StatusType::Error)
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let color = match self.status_type {
            StatusType::Info => Color::Cyan,
            StatusType::Success => Color::Green,
            StatusType::Warning => Color::Yellow,
            StatusType::Error => Color::Red,
        };

        let paragraph = Paragraph::new(self.message.as_str())
            .style(Style::default().fg(color))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(paragraph, area);
    }
}
