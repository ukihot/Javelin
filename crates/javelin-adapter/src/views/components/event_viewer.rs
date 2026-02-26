// EventViewer - イベント表示コンポーネント
// 責務: INFO/ERRORイベントの表示（カレンダー統合）

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use super::Calendar;

#[derive(Debug, Clone)]
pub enum EventLevel {
    Info,
    Error,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub level: EventLevel,
    pub timestamp: String,
    pub message: String,
}

impl Event {
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            level: EventLevel::Info,
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            level: EventLevel::Error,
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            message: message.into(),
        }
    }
}

pub struct EventViewer {
    events: Vec<Event>,
    state: ListState,
    max_events: usize,
    calendar: Calendar,
}

impl EventViewer {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            state: ListState::default(),
            max_events: 100,
            calendar: Calendar::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);

        // 最大イベント数を超えたら古いものを削除
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }

        // 自動的に最新イベントにスクロール
        if !self.events.is_empty() {
            self.state.select(Some(self.events.len() - 1));
        }
    }

    pub fn add_info(&mut self, message: impl Into<String>) {
        self.add_event(Event::info(message));
    }

    pub fn add_error(&mut self, message: impl Into<String>) {
        self.add_event(Event::error(message));
    }

    pub fn scroll_up(&mut self) {
        if self.events.is_empty() {
            return;
        }

        let selected = self.state.selected().unwrap_or(self.events.len() - 1);
        if selected > 0 {
            self.state.select(Some(selected - 1));
        }
    }

    pub fn scroll_down(&mut self) {
        if self.events.is_empty() {
            return;
        }

        let selected = self.state.selected().unwrap_or(0);
        if selected < self.events.len() - 1 {
            self.state.select(Some(selected + 1));
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.state.select(None);
    }

    fn wrap_message(&self, message: &str, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![message.to_string()];
        }

        let mut lines = Vec::new();
        let mut current = String::new();

        for word in message.split_whitespace() {
            if current.is_empty() {
                current = word.to_string();
            } else if current.len() + 1 + word.len() <= width {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(current);
                current = word.to_string();
            }
        }

        if !current.is_empty() {
            lines.push(current);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // エリアを上下に分割: イベントログ62%、カレンダー38%
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(62), // イベントログ
                Constraint::Percentage(38), // カレンダー
            ])
            .split(area);

        // イベントログを描画
        let log_width = chunks[0].width.saturating_sub(2) as usize; // ボーダー分を引く

        let items: Vec<ListItem> = self
            .events
            .iter()
            .map(|event| {
                let (level_str, level_color) = match event.level {
                    EventLevel::Info => ("INFO ", Color::Cyan),
                    EventLevel::Error => ("ERROR", Color::Red),
                };

                let prefix = format!("[{}] {} ", event.timestamp, level_str);
                let prefix_len = prefix.len();
                let available_width = log_width.saturating_sub(prefix_len);

                let wrapped_lines = self.wrap_message(&event.message, available_width);
                let mut text_lines = Vec::new();

                for (i, line) in wrapped_lines.into_iter().enumerate() {
                    if i == 0 {
                        // 最初の行にはプレフィックスを付ける
                        text_lines.push(Line::from(vec![
                            Span::styled(
                                format!("[{}] ", event.timestamp),
                                Style::default().fg(Color::DarkGray),
                            ),
                            Span::styled(
                                level_str,
                                Style::default().fg(level_color).add_modifier(Modifier::BOLD),
                            ),
                            Span::raw(" "),
                            Span::styled(line, Style::default().fg(Color::White)),
                        ]));
                    } else {
                        // 2行目以降はインデント
                        text_lines.push(Line::from(vec![
                            Span::raw(" ".repeat(prefix_len)),
                            Span::styled(line, Style::default().fg(Color::White)),
                        ]));
                    }
                }

                ListItem::new(Text::from(text_lines))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("◆ イベントログ ◆")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray)),
            )
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

        frame.render_stateful_widget(list, chunks[0], &mut self.state);

        // カレンダーを描画（ボーダー付きブロックで囲んで領域全体を使用）
        let calendar_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Gray));
        let calendar_inner = calendar_block.inner(chunks[1]);
        frame.render_widget(calendar_block, chunks[1]);
        self.calendar.render(frame, calendar_inner);
    }
}

impl Default for EventViewer {
    fn default() -> Self {
        Self::new()
    }
}
