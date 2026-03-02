// JournalDetailPage - 仕訳詳細画面のビュー
// 仕訳の詳細情報を表示する

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
};
use tokio::sync::mpsc;

use crate::{
    presenter::JournalEntryDetailViewModel, views::components::loading_spinner::LoadingSpinner,
};

/// 仕訳詳細ページ
pub struct JournalDetailPage {
    /// 詳細データ受信チャネル
    detail_rx: Option<mpsc::UnboundedReceiver<JournalEntryDetailViewModel>>,
    /// 進捗メッセージ受信チャネル
    progress_rx: Option<mpsc::UnboundedReceiver<String>>,
    /// 詳細データ
    detail: Option<JournalEntryDetailViewModel>,
    /// 進捗メッセージ
    progress_message: Option<String>,
    /// ローディングスピナー
    loading_spinner: LoadingSpinner,
    /// 初回ロード済みフラグ
    loaded: bool,
}

impl JournalDetailPage {
    /// 新しい仕訳詳細ページを作成
    pub fn new() -> Self {
        Self {
            detail_rx: None,
            progress_rx: None,
            detail: None,
            progress_message: None,
            loading_spinner: LoadingSpinner::new(),
            loaded: false,
        }
    }

    /// 詳細データ受信チャネルを設定
    pub fn set_detail_receiver(
        &mut self,
        rx: mpsc::UnboundedReceiver<JournalEntryDetailViewModel>,
    ) {
        self.detail_rx = Some(rx);
    }

    /// 進捗メッセージ受信チャネルを設定
    pub fn set_progress_receiver(&mut self, rx: mpsc::UnboundedReceiver<String>) {
        self.progress_rx = Some(rx);
    }

    /// 初回ロードが必要かどうか
    pub fn needs_initial_load(&self) -> bool {
        !self.loaded
    }

    /// ロード済みとしてマーク
    pub fn mark_loaded(&mut self) {
        self.loaded = true;
    }

    /// 詳細データをポーリング
    pub fn poll_detail_data(&mut self) {
        if let Some(ref mut rx) = self.detail_rx {
            while let Ok(detail) = rx.try_recv() {
                self.detail = Some(detail);
                self.progress_message = None;
            }
        }
    }

    /// 進捗メッセージをポーリング
    pub fn poll_progress_messages(&mut self) {
        if let Some(ref mut rx) = self.progress_rx {
            while let Ok(message) = rx.try_recv() {
                self.progress_message = Some(message);
            }
        }
    }

    /// 画面を描画
    pub fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(3), // タイトル
            Constraint::Min(0),    // コンテンツ
            Constraint::Length(3), // ヘルプ
        ])
        .split(frame.area());

        // タイトル
        self.render_title(frame, chunks[0]);

        // コンテンツ
        if self.detail.is_some() {
            self.render_detail(frame, chunks[1]);
        } else if let Some(ref message) = self.progress_message {
            self.loading_spinner.tick();
            self.loading_spinner.render(frame, chunks[1], message);
        } else {
            self.loading_spinner.tick();
            self.loading_spinner.render(frame, chunks[1], "読み込み中...");
        }

        // ヘルプ
        self.render_help(frame, chunks[2]);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new("A-04: 仕訳詳細")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        frame.render_widget(title, area);
    }

    fn render_detail(&self, frame: &mut Frame, area: Rect) {
        let Some(ref detail) = self.detail else {
            return;
        };

        let chunks = Layout::vertical([
            Constraint::Length(8), // ヘッダー情報
            Constraint::Min(0),    // 明細
        ])
        .split(area);

        // ヘッダー情報
        self.render_header(frame, chunks[0], detail);

        // 明細
        self.render_lines(frame, chunks[1], detail);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect, detail: &JournalEntryDetailViewModel) {
        let header_text = vec![
            Line::from(vec![
                Span::styled("伝票番号: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    detail.entry_number.as_deref().unwrap_or("未採番"),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
                Span::raw("  "),
                Span::styled("ステータス: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    &detail.status_label,
                    Style::default().fg(self.status_color(&detail.status)),
                ),
            ]),
            Line::from(vec![
                Span::styled("取引日: ", Style::default().fg(Color::Gray)),
                Span::styled(&detail.transaction_date, Style::default().fg(Color::White)),
                Span::raw("  "),
                Span::styled("証憑番号: ", Style::default().fg(Color::Gray)),
                Span::styled(&detail.voucher_number, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("作成者: ", Style::default().fg(Color::Gray)),
                Span::styled(&detail.created_by, Style::default().fg(Color::White)),
                Span::raw("  "),
                Span::styled("作成日時: ", Style::default().fg(Color::Gray)),
                Span::styled(&detail.created_at, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("承認者: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    detail.approved_by.as_deref().unwrap_or("-"),
                    Style::default().fg(Color::White),
                ),
                Span::raw("  "),
                Span::styled("承認日時: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    detail.approved_at.as_deref().unwrap_or("-"),
                    Style::default().fg(Color::White),
                ),
            ]),
        ];

        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::ALL).title("仕訳情報"))
            .wrap(Wrap { trim: false });

        frame.render_widget(header, area);
    }

    fn render_lines(&self, frame: &mut Frame, area: Rect, detail: &JournalEntryDetailViewModel) {
        let header = Row::new(vec![
            "行",
            "借/貸",
            "勘定科目",
            "補助科目",
            "部門",
            "金額",
            "通貨",
            "税区分",
            "税額",
        ])
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .bottom_margin(1);

        let rows = detail.lines.iter().map(|line| {
            Row::new(vec![
                line.line_number.to_string(),
                line.side_label.clone(),
                format!("{} {}", line.account_code, line.account_name),
                line.sub_account_code.as_deref().unwrap_or("-").to_string(),
                line.department_code.as_deref().unwrap_or("-").to_string(),
                format!("{:.2}", line.amount),
                line.currency.clone(),
                line.tax_type.clone(),
                format!("{:.2}", line.tax_amount),
            ])
            .style(Style::default().fg(Color::White))
        });

        let table = Table::new(
            rows,
            [
                Constraint::Length(4),  // 行
                Constraint::Length(6),  // 借/貸
                Constraint::Min(20),    // 勘定科目
                Constraint::Length(12), // 補助科目
                Constraint::Length(8),  // 部門
                Constraint::Length(15), // 金額
                Constraint::Length(6),  // 通貨
                Constraint::Length(10), // 税区分
                Constraint::Length(12), // 税額
            ],
        )
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("仕訳明細"))
        .column_spacing(1);

        frame.render_widget(table, area);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let help = Paragraph::new("[Esc] 戻る")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, area);
    }

    fn status_color(&self, status: &str) -> Color {
        match status {
            "Draft" => Color::Gray,
            "PendingApproval" => Color::Yellow,
            "Posted" => Color::Green,
            "Reversed" => Color::Red,
            "Corrected" => Color::Blue,
            "Closed" => Color::DarkGray,
            "Deleted" => Color::LightRed,
            _ => Color::Gray,
        }
    }
}

impl Default for JournalDetailPage {
    fn default() -> Self {
        Self::new()
    }
}
