// InfoPanel - 情報パネルコンポーネント
// 責務: 情報表示パネル（Ratatui Paragraphウィジェット活用）

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

/// 情報パネル
pub struct InfoPanel {
    title: String,
    lines: Vec<Line<'static>>,
    border_color: Color,
    alignment: Alignment,
}

impl InfoPanel {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            lines: Vec::new(),
            border_color: Color::Cyan,
            alignment: Alignment::Left,
        }
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn add_line(&mut self, label: impl Into<String>, value: impl Into<String>) {
        let line = Line::from(vec![
            Span::styled(format!("{}: ", label.into()), Style::default().fg(Color::DarkGray)),
            Span::styled(value.into(), Style::default().fg(Color::Yellow)),
        ]);
        self.lines.push(line);
    }

    pub fn add_text(&mut self, text: impl Into<String>) {
        let line = Line::from(Span::styled(text.into(), Style::default().fg(Color::White)));
        self.lines.push(line);
    }

    pub fn add_warning(&mut self, text: impl Into<String>) {
        let line = Line::from(vec![
            Span::styled("⚠ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(text.into(), Style::default().fg(Color::Yellow)),
        ]);
        self.lines.push(line);
    }

    pub fn add_error(&mut self, text: impl Into<String>) {
        let line = Line::from(vec![
            Span::styled("✖ ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(text.into(), Style::default().fg(Color::Red)),
        ]);
        self.lines.push(line);
    }

    pub fn add_success(&mut self, text: impl Into<String>) {
        let line = Line::from(vec![
            Span::styled("✓ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(text.into(), Style::default().fg(Color::Green)),
        ]);
        self.lines.push(line);
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// 描画
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let paragraph = Paragraph::new(self.lines.clone())
            .block(
                Block::default()
                    .title(self.title.as_str())
                    .title_style(
                        Style::default().fg(self.border_color).add_modifier(Modifier::BOLD),
                    )
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(self.border_color)),
            )
            .alignment(self.alignment)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}
