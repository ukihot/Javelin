// ClosingLockPage - 締日固定画面
// 責務: 取引データのロック処理

use javelin_application::dtos::LockClosingPeriodResponse;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::views::components::{DataTable, EventViewer, LoadingSpinner};

#[derive(Debug, Clone, PartialEq)]
enum LoadingState {
    Loading,
    Loaded,
    Error(String),
}

pub struct ClosingLockPage {
    lock_table: DataTable,
    event_viewer: EventViewer,
    loading_spinner: LoadingSpinner,
    loading_state: LoadingState,
    animation_frame: usize,
}

impl ClosingLockPage {
    pub fn new() -> Self {
        let headers = vec![
            "期間".to_string(),
            "ロック状態".to_string(),
            "ロック日時".to_string(),
            "ロック件数".to_string(),
            "監査ログID".to_string(),
        ];

        let lock_table = DataTable::new("◆ 締日固定 - データロック管理 ◆", headers)
            .with_column_widths(vec![15, 12, 20, 12, 35]);

        let mut event_viewer = EventViewer::new();
        event_viewer.add_info("締日固定画面を開きました");
        event_viewer.add_info("データを読み込んでいます...");

        Self {
            lock_table,
            event_viewer,
            loading_spinner: LoadingSpinner::new(),
            loading_state: LoadingState::Loading,
            animation_frame: 0,
        }
    }

    pub fn set_response(&mut self, response: LockClosingPeriodResponse) {
        let data = vec![vec![
            "2024-12".to_string(),
            "ロック済".to_string(),
            response.locked_at.clone(),
            response.locked_entries_count.to_string(),
            response.audit_log_id.clone(),
        ]];

        self.lock_table.set_data(data);
        self.loading_state = LoadingState::Loaded;
        self.event_viewer.add_info(format!(
            "締日固定完了: {} 件のエントリをロック",
            response.locked_entries_count
        ));
    }

    pub fn set_error(&mut self, error: String) {
        self.loading_state = LoadingState::Error(error.clone());
        self.event_viewer.add_error(format!("エラー: {}", error));
    }

    pub fn is_loading(&self) -> bool {
        self.loading_state == LoadingState::Loading
    }

    pub fn tick(&mut self) {
        self.animation_frame = (self.animation_frame + 1) % 60;
        if self.loading_state == LoadingState::Loading {
            self.loading_spinner.tick();
        }
    }

    pub fn add_info(&mut self, message: impl Into<String>) {
        self.event_viewer.add_info(message);
    }

    pub fn add_error(&mut self, message: impl Into<String>) {
        self.event_viewer.add_error(message);
    }

    pub fn select_next(&mut self) {
        self.lock_table.select_next();
    }

    pub fn select_previous(&mut self) {
        self.lock_table.select_previous();
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
            .split(area);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(chunks[0]);

        if self.loading_state == LoadingState::Loading {
            self.loading_spinner.render(
                frame,
                left_chunks[0],
                "締日固定データを読み込んでいます...",
            );
        } else {
            self.lock_table.render(frame, left_chunks[0]);
        }

        self.render_status_bar(frame, left_chunks[1]);
        self.event_viewer.render(frame, chunks[1]);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let cursor = if self.animation_frame < 30 {
            "▮"
        } else {
            " "
        };

        let status_text = vec![Line::from(vec![
            Span::styled(" [↑↓] ", Style::default().fg(Color::DarkGray)),
            Span::styled("選択", Style::default().fg(Color::Gray)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[Esc] ", Style::default().fg(Color::DarkGray)),
            Span::styled("戻る", Style::default().fg(Color::Gray)),
            Span::styled(
                format!(" {}", cursor),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ])];

        let paragraph = Paragraph::new(status_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        frame.render_widget(paragraph, area);
    }
}

impl Default for ClosingLockPage {
    fn default() -> Self {
        Self::new()
    }
}
