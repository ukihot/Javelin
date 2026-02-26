// NoteDraftPage - 注記草案生成画面
// 責務: 開示情報の整理

use javelin_application::dtos::GenerateNoteDraftResponse;
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

pub struct NoteDraftPage {
    note_table: DataTable,
    event_viewer: EventViewer,
    loading_spinner: LoadingSpinner,
    loading_state: LoadingState,
    animation_frame: usize,
}

impl NoteDraftPage {
    pub fn new() -> Self {
        let headers = vec!["注記項目".to_string(), "内容".to_string(), "詳細".to_string()];

        let note_table = DataTable::new("◆ 注記草案生成 - 開示情報整理 ◆", headers)
            .with_column_widths(vec![25, 35, 35]);

        let mut event_viewer = EventViewer::new();
        event_viewer.add_info("注記草案生成画面を開きました");
        event_viewer.add_info("データを読み込んでいます...");

        Self {
            note_table,
            event_viewer,
            loading_spinner: LoadingSpinner::new(),
            loading_state: LoadingState::Loading,
            animation_frame: 0,
        }
    }

    pub fn set_response(&mut self, response: GenerateNoteDraftResponse) {
        let mut data = Vec::new();

        // 会計方針
        for policy in &response.accounting_policies {
            data.push(vec!["会計方針".to_string(), policy.clone(), "-".to_string()]);
        }

        // 重要な見積り
        for estimate in &response.significant_estimates {
            data.push(vec!["重要な見積り".to_string(), estimate.clone(), "-".to_string()]);
        }

        // 勘定科目内訳
        for breakdown in &response.account_breakdowns {
            let components = breakdown.components.join(", ");
            data.push(vec!["勘定科目内訳".to_string(), breakdown.account_code.clone(), components]);
        }

        self.note_table.set_data(data);
        self.loading_state = LoadingState::Loaded;
        self.event_viewer.add_info(format!(
            "注記草案生成完了: 会計方針 {} 件、見積り {} 件、内訳 {} 件",
            response.accounting_policies.len(),
            response.significant_estimates.len(),
            response.account_breakdowns.len()
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
        self.note_table.select_next();
    }

    pub fn select_previous(&mut self) {
        self.note_table.select_previous();
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
                "注記草案データを読み込んでいます...",
            );
        } else {
            self.note_table.render(frame, left_chunks[0]);
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

impl Default for NoteDraftPage {
    fn default() -> Self {
        Self::new()
    }
}
