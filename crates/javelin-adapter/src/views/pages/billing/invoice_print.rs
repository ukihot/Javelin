// 請求書印刷画面

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    page_states::billing::InvoicePrintPageState,
    presenter::invoice_print_presenter::{InvoicePrintViewModel, PrintStatus},
};

/// 請求書印刷画面
pub struct InvoicePrintPage {
    current_view_model: Option<InvoicePrintViewModel>,
}

impl InvoicePrintPage {
    pub fn new() -> Self {
        Self { current_view_model: None }
    }

    /// ビューモデルを更新
    pub fn update_view_model(&mut self, view_model: InvoicePrintViewModel) {
        self.current_view_model = Some(view_model);
    }

    /// 画面を描画
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // タイトル
                Constraint::Min(5),    // メイン
                Constraint::Length(3), // ステータス
            ])
            .split(area);

        // タイトル
        self.render_title(frame, chunks[0]);

        // メインコンテンツ
        self.render_main_content(frame, chunks[1]);

        // ステータス
        self.render_status(frame, chunks[2]);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new("請求書印刷")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, area);
    }

    fn render_main_content(&self, frame: &mut Frame, area: Rect) {
        let content = if let Some(ref vm) = self.current_view_model {
            match &vm.status {
                PrintStatus::Idle => {
                    vec![
                        Line::from(""),
                        Line::from(Span::styled(
                            "請求書印刷画面",
                            Style::default().add_modifier(Modifier::BOLD),
                        )),
                        Line::from(""),
                        Line::from("Enterキーを押すと、モック請求書を印刷します。"),
                        Line::from(""),
                        Line::from(Span::styled(
                            "※ 現在は開発用のモックデータを使用しています",
                            Style::default().fg(Color::Yellow),
                        )),
                    ]
                }
                PrintStatus::Printing => {
                    vec![
                        Line::from(""),
                        Line::from(Span::styled(
                            "印刷中...",
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                        )),
                        Line::from(""),
                        Line::from("請求書PDFを生成しています。"),
                    ]
                }
                PrintStatus::Success { file_path } => {
                    vec![
                        Line::from(""),
                        Line::from(Span::styled(
                            "✓ 印刷成功",
                            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                        )),
                        Line::from(""),
                        Line::from(format!("保存先: {}", file_path)),
                        Line::from(""),
                        Line::from("Enterキーで再度印刷できます。"),
                    ]
                }
                PrintStatus::Error => {
                    vec![
                        Line::from(""),
                        Line::from(Span::styled(
                            "✗ 印刷失敗",
                            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                        )),
                        Line::from(""),
                        Line::from(&vm.message),
                    ]
                }
            }
        } else {
            vec![Line::from("読み込み中...")]
        };

        let paragraph = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title("印刷情報"))
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    fn render_status(&self, frame: &mut Frame, area: Rect) {
        let status_text = if let Some(ref vm) = self.current_view_model {
            vm.message.clone()
        } else {
            "準備中...".to_string()
        };

        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("ステータス"));

        frame.render_widget(status, area);
    }
}

impl Default for InvoicePrintPage {
    fn default() -> Self {
        Self::new()
    }
}
