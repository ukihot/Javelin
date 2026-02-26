// LedgerPage - 元帳一覧画面
// 責務: 勘定科目別元帳の一覧表示（レトロで哀愁漂うデザイン）

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tokio::sync::mpsc;

use crate::{
    format_amount, format_balance,
    presenter::LedgerViewModel,
    truncate_text,
    views::components::{DataTable, InfoPanel},
};

/// 元帳一覧画面
pub struct LedgerPage {
    /// 元帳テーブル
    ledger_table: DataTable,
    /// 勘定情報パネル
    info_panel: InfoPanel,
    /// ViewModelレシーバー
    ledger_receiver: mpsc::UnboundedReceiver<LedgerViewModel>,
    /// 現在表示中の元帳データ
    current_ledger: Option<LedgerViewModel>,
    /// アニメーションフレーム
    animation_frame: usize,
}

impl LedgerPage {
    pub fn new(ledger_receiver: mpsc::UnboundedReceiver<LedgerViewModel>) -> Self {
        // レトロなヘッダー（昭和の元帳風）
        let headers = vec![
            "日付".to_string(),
            "伝票No".to_string(),
            "摘要".to_string(),
            "借方".to_string(),
            "貸方".to_string(),
            "残高".to_string(),
        ];

        let ledger_table = DataTable::new("◆ 総勘定元帳 ◆", headers)
            .with_column_widths(vec![12, 15, 35, 13, 13, 13]);

        let info_panel = InfoPanel::new("◇ 勘定情報 ◇").with_border_color(Color::Cyan);

        Self {
            ledger_table,
            info_panel,
            ledger_receiver,
            current_ledger: None,
            animation_frame: 0,
        }
    }

    /// ViewModelを受信してテーブルを更新
    pub fn update(&mut self) {
        if let Ok(view_model) = self.ledger_receiver.try_recv() {
            // テーブルデータを構築
            let rows: Vec<Vec<String>> = view_model
                .entries
                .iter()
                .map(|entry| {
                    vec![
                        entry.transaction_date.clone(),
                        entry.entry_number.clone(),
                        truncate_text!(&entry.description, 33),
                        format_amount!(entry.debit_amount, 11),
                        format_amount!(entry.credit_amount, 11),
                        format_balance!(entry.balance, 11),
                    ]
                })
                .collect();

            self.ledger_table.set_data(rows);

            // 情報パネルを更新
            self.update_info_panel(&view_model);

            self.current_ledger = Some(view_model);
        }
    }

    /// 情報パネルを更新
    fn update_info_panel(&mut self, ledger: &LedgerViewModel) {
        self.info_panel.clear();

        // レトロな罫線で区切り
        self.info_panel.add_text("━━━━━━━━━━━━━━");
        self.info_panel.add_line("科目コード", &ledger.account_code);
        self.info_panel.add_line("科目名", &ledger.account_name);
        self.info_panel.add_text("━━━━━━━━━━━━━━");
        self.info_panel.add_line("期首残高", &format_balance!(ledger.opening_balance));
        self.info_panel.add_line("当期借方", &format_amount!(ledger.total_debit));
        self.info_panel.add_line("当期貸方", &format_amount!(ledger.total_credit));
        self.info_panel.add_text("━━━━━━━━━━━━━━");
        self.info_panel.add_line("期末残高", &format_balance!(ledger.closing_balance));
        self.info_panel.add_text("━━━━━━━━━━━━━━");
    }

    /// 次の行を選択
    pub fn select_next(&mut self) {
        self.ledger_table.select_next();
    }

    /// 前の行を選択
    pub fn select_previous(&mut self) {
        self.ledger_table.select_previous();
    }

    /// 選択中のインデックスを取得
    pub fn selected_index(&self) -> Option<usize> {
        self.ledger_table.selected_index()
    }
    /// 選択中のエントリを取得
    pub fn get_selected_entry(&self) -> Option<&crate::presenter::LedgerEntryViewModel> {
        let index = self.selected_index()?;
        self.current_ledger.as_ref()?.entries.get(index)
    }

    /// 勘定科目コードを取得
    pub fn get_account_code(&self) -> Option<String> {
        self.current_ledger.as_ref().map(|l| l.account_code.clone())
    }

    /// 勘定科目名を取得
    pub fn get_account_name(&self) -> Option<String> {
        self.current_ledger.as_ref().map(|l| l.account_name.clone())
    }

    /// アニメーションフレームを進める
    pub fn tick(&mut self) {
        self.animation_frame = (self.animation_frame + 1) % 60;
        self.ledger_table.tick_loading();
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // 画面を上下に分割（メインエリア + ステータスバー）
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(area);

        // メインエリアを左右に分割（テーブル + 情報パネル）
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(chunks[0]);

        // 元帳テーブル
        self.ledger_table.render(frame, main_chunks[0]);

        // 情報パネル
        self.info_panel.render(frame, main_chunks[1]);

        // ステータスバー（レトロな雰囲気）
        self.render_status_bar(frame, chunks[1]);
    }

    /// ステータスバーを描画（レトロな雰囲気）
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        // 点滅するカーソル（レトロ感）
        let cursor = if self.animation_frame < 30 {
            "▮"
        } else {
            " "
        };

        let status_text = vec![Line::from(vec![
            Span::styled(" [↑↓] ", Style::default().fg(Color::DarkGray)),
            Span::styled("選択", Style::default().fg(Color::Gray)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[Enter] ", Style::default().fg(Color::DarkGray)),
            Span::styled("詳細", Style::default().fg(Color::Gray)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[F2] ", Style::default().fg(Color::DarkGray)),
            Span::styled("科目変更", Style::default().fg(Color::Gray)),
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

impl Default for LedgerPage {
    fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        drop(tx);
        Self::new(rx)
    }
}
