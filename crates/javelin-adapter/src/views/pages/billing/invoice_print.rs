// 請求書印刷画面

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::views::layouts::{Breadcrumb, KeyBinding, MainLayout};

pub struct InvoicePrintPage {
    layout: MainLayout,
    status_message: String,
}

impl InvoicePrintPage {
    pub fn new() -> Self {
        let layout = MainLayout::new("請求書発行")
            .with_breadcrumbs(vec![Breadcrumb::new("販売管理"), Breadcrumb::new("請求書発行")])
            .with_key_bindings(vec![
                KeyBinding::new("Enter", "印刷実行"),
                KeyBinding::new("q/Esc", "戻る"),
            ]);

        Self { layout, status_message: "Enterキーで印刷を実行します".to_string() }
    }

    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = message.into();
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.layout.render(frame, false, |frame, area, _| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(10), // 説明エリア
                    Constraint::Min(5),     // ステータスエリア
                ])
                .split(area);

            // 説明エリア
            let info_text = vec![
                Line::from(vec![Span::styled(
                    "請求書印刷機能",
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from("この画面では請求書をPDF形式で生成します。"),
                Line::from(""),
                Line::from(vec![Span::styled("操作方法:", Style::default().fg(Color::Yellow))]),
                Line::from("  • Enterキー: モックデータで請求書を印刷"),
                Line::from("  • q/Esc: 前の画面に戻る"),
            ];

            let info_block = Paragraph::new(info_text)
                .block(Block::default().borders(Borders::ALL).title("説明"))
                .wrap(Wrap { trim: true });

            frame.render_widget(info_block, chunks[0]);

            // ステータスエリア
            let status_text = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("ステータス: ", Style::default().fg(Color::Gray)),
                    Span::styled(&self.status_message, Style::default().fg(Color::Green)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("出力先: ", Style::default().fg(Color::Gray)),
                    Span::styled("カレントディレクトリ", Style::default().fg(Color::White)),
                ]),
            ];

            let status_block = Paragraph::new(status_text)
                .block(Block::default().borders(Borders::ALL).title("実行状態"))
                .alignment(Alignment::Left);

            frame.render_widget(status_block, chunks[1]);
        });
    }
}

impl Default for InvoicePrintPage {
    fn default() -> Self {
        Self::new()
    }
}
