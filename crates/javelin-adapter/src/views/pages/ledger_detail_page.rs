// LedgerDetailPage - 元帳詳細閲覧画面
// 責務: 選択された元帳エントリの詳細表示

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::{format_amount, format_balance, presenter::LedgerEntryViewModel};

/// 元帳詳細閲覧画面
pub struct LedgerDetailPage {
    /// 表示中のエントリ
    entry: LedgerEntryViewModel,
    /// 勘定科目情報
    account_code: String,
    account_name: String,
}

impl LedgerDetailPage {
    /// 新しいLedgerDetailPageを作成
    pub fn new(entry: LedgerEntryViewModel, account_code: String, account_name: String) -> Self {
        Self { entry, account_code, account_name }
    }

    /// エラーメッセージをイベントログに追加（互換性のため）
    pub fn add_error(&mut self, _message: impl Into<String>) {
        // LedgerDetailPageにはイベントログがないため、何もしない
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // 画面を上下に分割（ヘッダー + メイン + ステータスバー）
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // ヘッダー
                Constraint::Min(10),   // メイン
                Constraint::Length(3), // ステータスバー
            ])
            .split(area);

        // ヘッダー
        self.render_header(frame, chunks[0]);

        // メインエリア
        self.render_main(frame, chunks[1]);

        // ステータスバー
        self.render_status_bar(frame, chunks[2]);
    }

    /// ヘッダーを描画
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_text = vec![Line::from(vec![
            Span::styled(
                "◆ 元帳詳細 ◆ ",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{} - {}", self.account_code, self.account_name),
                Style::default().fg(Color::Yellow),
            ),
        ])];

        let paragraph = Paragraph::new(header_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        );

        frame.render_widget(paragraph, area);
    }

    /// メインエリアを描画
    fn render_main(&self, frame: &mut Frame, area: Rect) {
        let content = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("取引日付: ", Style::default().fg(Color::Gray)),
                Span::styled(&self.entry.transaction_date, Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("伝票番号: ", Style::default().fg(Color::Gray)),
                Span::styled(&self.entry.entry_number, Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("仕訳ID: ", Style::default().fg(Color::Gray)),
                Span::styled(&self.entry.entry_id, Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("借方金額: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format_amount!(self.entry.debit_amount),
                    if self.entry.debit_amount > 0.0 {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("貸方金額: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format_amount!(self.entry.credit_amount),
                    if self.entry.credit_amount > 0.0 {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("残高: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format_balance!(self.entry.balance),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(vec![Span::styled("摘要:", Style::default().fg(Color::Gray))]),
            Line::from(Span::styled(&self.entry.description, Style::default().fg(Color::White))),
        ];

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::White)),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    /// ステータスバーを描画
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let status_text = vec![Line::from(vec![
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::styled("]戻る", Style::default().fg(Color::DarkGray)),
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

impl Default for LedgerDetailPage {
    fn default() -> Self {
        Self {
            entry: LedgerEntryViewModel {
                transaction_date: "2024-01-01".to_string(),
                entry_number: "JE-001".to_string(),
                entry_id: "entry-001".to_string(),
                description: "サンプルエントリ".to_string(),
                debit_amount: 0.0,
                credit_amount: 0.0,
                balance: 0.0,
            },
            account_code: "1001".to_string(),
            account_name: "現金".to_string(),
        }
    }
}
