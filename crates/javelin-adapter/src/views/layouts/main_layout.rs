// MainLayout - メインレイアウト
// 責務: ヘッダー、コンテンツ、フッターの配置

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// キーバインド情報
#[derive(Clone)]
pub struct KeyBinding {
    pub key: String,
    pub description: String,
}

impl KeyBinding {
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self { key: key.into(), description: description.into() }
    }
}

/// パンくずリストの項目
#[derive(Clone)]
pub struct Breadcrumb {
    pub label: String,
}

impl Breadcrumb {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into() }
    }
}

/// メインレイアウトの構造
pub struct MainLayout {
    title: String,
    breadcrumbs: Vec<Breadcrumb>,
    key_bindings: Vec<KeyBinding>,
}

impl MainLayout {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            breadcrumbs: Vec::new(),
            key_bindings: vec![
                KeyBinding::new("e", "イベント表示切替"),
                KeyBinding::new("hjkl", "移動"),
            ],
        }
    }

    /// パンくずリストを設定
    pub fn with_breadcrumbs(mut self, breadcrumbs: Vec<Breadcrumb>) -> Self {
        self.breadcrumbs = breadcrumbs;
        self
    }

    /// キーバインドを設定
    pub fn with_key_bindings(mut self, bindings: Vec<KeyBinding>) -> Self {
        self.key_bindings = bindings;
        self
    }

    /// レイアウトを描画（イベントビューア対応）
    pub fn render<F>(&self, frame: &mut Frame, show_events: bool, render_content: F)
    where
        F: FnOnce(&mut Frame, Rect, Option<Rect>),
    {
        let size = frame.area();

        // 3分割レイアウト: ヘッダー、コンテンツ、フッター
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // ヘッダー
                Constraint::Min(10),   // コンテンツ
                Constraint::Length(3), // フッター
            ])
            .split(size);

        // ヘッダー
        self.render_header(frame, main_chunks[0]);

        // コンテンツエリア（イベントビューアの有無で分割）
        let content_chunks = if show_events {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(62), // メインビュー
                    Constraint::Percentage(38), // イベントビューア+カレンダー
                ])
                .split(main_chunks[1])
        } else {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)])
                .split(main_chunks[1])
        };

        let main_area = content_chunks[0];
        let event_area = if show_events {
            Some(content_chunks[1])
        } else {
            None
        };

        // コンテンツ（呼び出し元が描画）
        render_content(frame, main_area, event_area);

        // フッター
        self.render_footer(frame, main_chunks[2]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let mut header_spans = vec![
            Span::styled("◆ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(
                &self.title,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ];

        // パンくずリストがある場合は追加
        if !self.breadcrumbs.is_empty() {
            header_spans.push(Span::styled(" » ", Style::default().fg(Color::DarkGray)));

            for (i, crumb) in self.breadcrumbs.iter().enumerate() {
                if i > 0 {
                    header_spans.push(Span::styled(" › ", Style::default().fg(Color::DarkGray)));
                }

                let style = if i == self.breadcrumbs.len() - 1 {
                    // 最後の項目（現在地）は強調
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    // それ以外は控えめに
                    Style::default().fg(Color::Gray)
                };

                header_spans.push(Span::styled(&crumb.label, style));
            }
        }

        let header = Paragraph::new(Line::from(header_spans))
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(header, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        // キーバインド情報を整形（レトロスタイル）
        let mut key_hints: Vec<Span> = Vec::new();
        for (i, kb) in self.key_bindings.iter().enumerate() {
            if i > 0 {
                key_hints.push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));
            }
            key_hints.push(Span::styled(
                format!("[{}]", kb.key),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ));
            key_hints.push(Span::styled(
                format!(" {}", kb.description),
                Style::default().fg(Color::Gray),
            ));
        }

        let footer = Paragraph::new(Line::from(key_hints))
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(footer, area);
    }
}

/// レイアウトヘルパー - 2カラムレイアウト
pub fn two_column_layout(area: Rect, left_width: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(left_width), Constraint::Min(0)])
        .split(area)
        .to_vec()
}

/// レイアウトヘルパー - 3カラムレイアウト
pub fn three_column_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area)
        .to_vec()
}
