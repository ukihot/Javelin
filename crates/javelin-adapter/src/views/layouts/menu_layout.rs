// MenuLayout - メニュー選択レイアウト
// 責務: 業務メニューの表示とナビゲーション
// デザイン: レトロな基幹システム風

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

use crate::views::components::EventViewer;

/// メニュー項目
#[derive(Clone)]
pub struct MenuItem {
    pub code: String,
    pub label: String,
    pub description: String,
}

impl MenuItem {
    pub fn new(
        code: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self { code: code.into(), label: label.into(), description: description.into() }
    }
}

/// メニューレイアウト
pub struct MenuLayout {
    system_name: String,
    department: String,
    user_name: String,
    event_viewer: EventViewer,
}

impl MenuLayout {
    pub fn new(
        system_name: impl Into<String>,
        department: impl Into<String>,
        user_name: impl Into<String>,
    ) -> Self {
        Self {
            system_name: system_name.into(),
            department: department.into(),
            user_name: user_name.into(),
            event_viewer: EventViewer::new(),
        }
    }

    pub fn event_viewer_mut(&mut self) -> &mut EventViewer {
        &mut self.event_viewer
    }

    /// レイアウトを描画（左62%コンテンツ、右38%イベントビューア）
    pub fn render<F>(&mut self, frame: &mut Frame, render_content: F)
    where
        F: FnOnce(&mut Frame, Rect),
    {
        let size = frame.area();

        // 全体レイアウト: ヘッダー、メインエリア、フッター
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // ヘッダー
                Constraint::Min(10),   // メインエリア
                Constraint::Length(3), // フッター
            ])
            .split(size);

        self.render_header(frame, chunks[0]);

        // メインエリアを左右分割: コンテンツ(62%) + イベントビューア(38%)
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
            .split(chunks[1]);

        render_content(frame, main_chunks[0]);
        self.event_viewer.render(frame, main_chunks[1]);

        self.render_footer(frame, chunks[2]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(2)])
            .split(area);

        // システム名
        let title_line = Line::from(vec![
            Span::styled("=== ", Style::default().fg(Color::Cyan)),
            Span::styled(
                &self.system_name,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ===", Style::default().fg(Color::Cyan)),
        ]);

        let title = Paragraph::new(title_line).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Cyan)),
        );

        frame.render_widget(title, header_chunks[0]);

        // 部門・ユーザー情報
        let info_line = Line::from(vec![
            Span::styled("部門: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&self.department, Style::default().fg(Color::Yellow)),
            Span::styled("  |  ", Style::default().fg(Color::DarkGray)),
            Span::styled("担当: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&self.user_name, Style::default().fg(Color::Yellow)),
        ]);

        let info = Paragraph::new(info_line).alignment(Alignment::Center);

        frame.render_widget(info, header_chunks[1]);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let footer_text = Line::from(vec![
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("↑↓", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("]選択 ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("Enter", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("]決定 ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("]終了", Style::default().fg(Color::DarkGray)),
        ]);

        let footer = Paragraph::new(footer_text).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        frame.render_widget(footer, area);
    }

    /// メニュー項目リストを描画
    pub fn render_menu_items(
        &self,
        frame: &mut Frame,
        area: Rect,
        items: &[MenuItem],
        selected: usize,
    ) {
        let list_items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_selected = i == selected;
                let style = if is_selected {
                    Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if is_selected { "> " } else { "  " };

                let line = Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(
                        format!("[{}] ", item.code),
                        if is_selected {
                            style
                        } else {
                            Style::default().fg(Color::Yellow)
                        },
                    ),
                    Span::styled(&item.label, style),
                    Span::styled(
                        format!("  - {}", item.description),
                        if is_selected {
                            style
                        } else {
                            Style::default().fg(Color::DarkGray)
                        },
                    ),
                ]);

                ListItem::new(line)
            })
            .collect();

        let list = List::new(list_items).block(
            Block::default()
                .title("業務メニュー")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(Color::Cyan)),
        );

        frame.render_widget(list, area);
    }
}
