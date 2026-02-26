// FormLayout - データ登録レイアウト
// 責務: フォーム入力画面の構造定義
// デザイン: レトロな基幹システム風の入力フォーム

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{input_mode::InputMode, views::components::EventViewer};

/// フォームレイアウト
pub struct FormLayout {
    title: String,
    form_code: String,
    status: InputMode,
    event_viewer: EventViewer,
}

impl FormLayout {
    pub fn new(title: impl Into<String>, form_code: impl Into<String>, status: InputMode) -> Self {
        Self {
            title: title.into(),
            form_code: form_code.into(),
            status,
            event_viewer: EventViewer::new(),
        }
    }

    pub fn event_viewer_mut(&mut self) -> &mut EventViewer {
        &mut self.event_viewer
    }

    /// タイトルを設定
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// ステータスを設定
    pub fn set_status(&mut self, status: InputMode) {
        self.status = status;
    }

    /// レイアウトを描画（左62%フォーム、右38%イベントビューア+カレンダー）
    pub fn render<F>(
        &mut self,
        frame: &mut Frame,
        input_mode: InputMode,
        footer_text: Option<Line<'static>>,
        render_form: F,
    ) where
        F: FnOnce(&mut Frame, Rect),
    {
        let size = frame.area();

        // 全体レイアウト: ヘッダー、メインエリア、フッター
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(4), // ヘッダー
                Constraint::Min(10),   // メインエリア
                Constraint::Length(3), // フッター
            ])
            .split(size);

        self.render_header(frame, chunks[0]);

        // メインエリアを左右分割: フォーム(62%) + イベントビューア+カレンダー(38%)
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
            .split(chunks[1]);

        render_form(frame, main_chunks[0]);

        // EventViewerが自動的にカレンダーも表示
        self.event_viewer.render(frame, main_chunks[1]);

        self.render_footer(frame, chunks[2], input_mode, footer_text);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Length(2)])
            .split(area);

        // タイトル行
        let title_line = Line::from(vec![
            Span::styled("■ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(
                &self.title,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("  [{}]", self.form_code), Style::default().fg(Color::Yellow)),
        ]);

        let title = Paragraph::new(title_line)
            .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT));

        frame.render_widget(title, header_chunks[0]);

        // ステータス行
        let status_line = Line::from(vec![
            Span::styled("状態: ", Style::default().fg(Color::DarkGray)),
            Span::styled(self.status.name(), Style::default().fg(Color::Green)),
        ]);

        let status = Paragraph::new(status_line)
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM));

        frame.render_widget(status, header_chunks[1]);
    }

    fn render_footer(
        &self,
        frame: &mut Frame,
        area: Rect,
        input_mode: InputMode,
        custom_text: Option<Line<'static>>,
    ) {
        let footer_text = if let Some(text) = custom_text {
            text
        } else {
            match input_mode {
                InputMode::Normal => Line::from(vec![
                    Span::styled("[", Style::default().fg(Color::DarkGray)),
                    Span::styled("hjkl", Style::default().fg(Color::Cyan)),
                    Span::styled("]移動 [", Style::default().fg(Color::DarkGray)),
                    Span::styled("i", Style::default().fg(Color::Cyan)),
                    Span::styled("]入力 [", Style::default().fg(Color::DarkGray)),
                    Span::styled("Esc", Style::default().fg(Color::Cyan)),
                    Span::styled("]戻る", Style::default().fg(Color::DarkGray)),
                ]),
                InputMode::Modify => Line::from(vec![
                    Span::styled("[", Style::default().fg(Color::DarkGray)),
                    Span::styled("jj", Style::default().fg(Color::Yellow)),
                    Span::styled("]確定 [", Style::default().fg(Color::DarkGray)),
                    Span::styled("Esc", Style::default().fg(Color::Yellow)),
                    Span::styled("]キャンセル", Style::default().fg(Color::DarkGray)),
                ]),
            }
        };

        let footer = Paragraph::new(footer_text).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        frame.render_widget(footer, area);
    }
}

/// フォーム用の2カラムレイアウト（ラベル + 入力欄）
pub fn form_field_layout(area: Rect, label_width: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(label_width), Constraint::Min(0)])
        .split(area)
        .to_vec()
}

/// フォーム用の行レイアウト
pub fn form_rows_layout(area: Rect, row_count: usize) -> Vec<Rect> {
    let constraints: Vec<Constraint> = (0..row_count).map(|_| Constraint::Length(3)).collect();

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area)
        .to_vec()
}
