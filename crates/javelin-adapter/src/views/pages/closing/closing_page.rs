// ClosingPage - 決算処理画面（試算表表示）
// 責務: 月次決算処理と試算表の表示（レトロで哀愁漂うデザイン）

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tokio::sync::mpsc;

use crate::{
    format_amount, format_balance, format_number, presenter::TrialBalanceViewModel, truncate_text,
    views::components::DataTable,
};

/// 決算画面の状態
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosingPageState {
    /// 試算表表示中
    TrialBalance,
    /// 決算処理中
    Processing,
    /// 完了
    Completed,
}

/// 決算処理画面
pub struct ClosingPage {
    /// 試算表テーブル
    trial_balance_table: DataTable,
    /// ViewModelレシーバー
    trial_balance_receiver: mpsc::UnboundedReceiver<TrialBalanceViewModel>,
    /// 現在の試算表データ
    current_trial_balance: Option<TrialBalanceViewModel>,
    /// 画面状態
    state: ClosingPageState,
    /// アニメーションフレーム
    animation_frame: usize,
    /// 処理進捗（0-100）
    progress: u8,
}

impl ClosingPage {
    pub fn new(trial_balance_receiver: mpsc::UnboundedReceiver<TrialBalanceViewModel>) -> Self {
        // レトロなヘッダー（昭和の試算表風）
        let headers = vec![
            "科目コード".to_string(),
            "科目名".to_string(),
            "期首残高".to_string(),
            "借方合計".to_string(),
            "貸方合計".to_string(),
            "期末残高".to_string(),
        ];

        let trial_balance_table =
            DataTable::new("◆ 試算表 ◆", headers).with_column_widths(vec![12, 25, 13, 13, 13, 13]);

        Self {
            trial_balance_table,
            trial_balance_receiver,
            current_trial_balance: None,
            state: ClosingPageState::TrialBalance,
            animation_frame: 0,
            progress: 0,
        }
    }

    /// ViewModelを受信してテーブルを更新
    pub fn update(&mut self) {
        if let Ok(view_model) = self.trial_balance_receiver.try_recv() {
            // テーブルデータを構築
            let rows: Vec<Vec<String>> = view_model
                .entries
                .iter()
                .map(|entry| {
                    vec![
                        entry.account_code.clone(),
                        truncate_text!(&entry.account_name, 23),
                        format_balance!(entry.opening_balance, 11),
                        format_amount!(entry.debit_amount, 11),
                        format_amount!(entry.credit_amount, 11),
                        format_balance!(entry.closing_balance, 11),
                    ]
                })
                .collect();

            self.trial_balance_table.set_data(rows);
            self.current_trial_balance = Some(view_model);
            self.state = ClosingPageState::TrialBalance;
        }
    }

    /// 決算処理を開始
    pub fn start_closing(&mut self) {
        self.state = ClosingPageState::Processing;
        self.progress = 0;
    }

    /// 決算処理の進捗を更新
    pub fn update_progress(&mut self, progress: u8) {
        self.progress = progress.min(100);
        if self.progress >= 100 {
            self.state = ClosingPageState::Completed;
        }
    }

    /// 次の行を選択
    pub fn select_next(&mut self) {
        self.trial_balance_table.select_next();
    }

    /// 前の行を選択
    pub fn select_previous(&mut self) {
        self.trial_balance_table.select_previous();
    }

    /// アニメーションフレームを進める
    pub fn tick(&mut self) {
        self.animation_frame = (self.animation_frame + 1) % 60;
        self.trial_balance_table.tick_loading();

        // 処理中は自動的に進捗を進める（デモ用）
        if self.state == ClosingPageState::Processing && self.progress < 100 {
            self.progress = (self.progress + 1).min(100);
            if self.progress >= 100 {
                self.state = ClosingPageState::Completed;
            }
        }
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // 画面を上下に分割（メインエリア + ステータスバー）
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(area);

        // メインエリア
        self.render_main_area(frame, chunks[0]);

        // ステータスバー
        self.render_status_bar(frame, chunks[1]);
    }

    /// メインエリアを描画
    fn render_main_area(&mut self, frame: &mut Frame, area: Rect) {
        match self.state {
            ClosingPageState::TrialBalance => {
                // 試算表表示
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(10), Constraint::Length(5)])
                    .split(area);

                self.trial_balance_table.render(frame, chunks[0]);
                self.render_summary(frame, chunks[1]);
            }
            ClosingPageState::Processing => {
                // 処理中表示
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(40),
                        Constraint::Length(7),
                        Constraint::Percentage(40),
                    ])
                    .split(area);

                self.render_processing(frame, chunks[1]);
            }
            ClosingPageState::Completed => {
                // 完了表示
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(10), Constraint::Length(5)])
                    .split(area);

                self.trial_balance_table.render(frame, chunks[0]);
                self.render_completed(frame, chunks[1]);
            }
        }
    }

    /// 試算表サマリーを描画（レトロな集計表示）
    fn render_summary(&self, frame: &mut Frame, area: Rect) {
        if let Some(tb) = &self.current_trial_balance {
            let text = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("  借方合計: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:>15}", format_number!(tb.total_debit)),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("    貸方合計: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:>15}", format_number!(tb.total_credit)),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ),
                ]),
            ];

            let paragraph = Paragraph::new(text).block(
                Block::default()
                    .title("◇ 合計 ◇")
                    .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Cyan)),
            );

            frame.render_widget(paragraph, area);
        }
    }

    /// 処理中画面を描画（レトロなプログレスバー）
    fn render_processing(&self, frame: &mut Frame, area: Rect) {
        let progress_width =
            (area.width.saturating_sub(4) as f32 * self.progress as f32 / 100.0) as u16;
        let bar = "█".repeat(progress_width as usize);
        let empty =
            "░".repeat((area.width.saturating_sub(4).saturating_sub(progress_width)) as usize);

        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(" 月次決算処理中", Style::default().fg(Color::Yellow)),
                Span::styled("...", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(bar, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(empty, Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![Span::styled(
                format!(" {}%", self.progress),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )]),
        ];

        let paragraph = Paragraph::new(text).block(
            Block::default()
                .title("◆ 処理状況 ◆")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Yellow)),
        );

        frame.render_widget(paragraph, area);
    }

    /// 完了画面を描画
    fn render_completed(&self, frame: &mut Frame, area: Rect) {
        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(" ✓ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled("月次決算処理が完了しました", Style::default().fg(Color::White)),
            ]),
        ];

        let paragraph = Paragraph::new(text).block(
            Block::default()
                .title("◇ 処理結果 ◇")
                .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Green)),
        );

        frame.render_widget(paragraph, area);
    }

    /// ステータスバーを描画
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let cursor = if self.animation_frame < 30 {
            "▮"
        } else {
            " "
        };

        let status_text = match self.state {
            ClosingPageState::TrialBalance => vec![Line::from(vec![
                Span::styled(" [↑↓] ", Style::default().fg(Color::DarkGray)),
                Span::styled("選択", Style::default().fg(Color::Gray)),
                Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                Span::styled("[F5] ", Style::default().fg(Color::DarkGray)),
                Span::styled("決算実行", Style::default().fg(Color::Gray)),
                Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                Span::styled("[Esc] ", Style::default().fg(Color::DarkGray)),
                Span::styled("戻る", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!(" {}", cursor),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
            ])],
            ClosingPageState::Processing => vec![Line::from(vec![
                Span::styled(" 処理中...", Style::default().fg(Color::Yellow)),
                Span::styled(" しばらくお待ちください", Style::default().fg(Color::Gray)),
            ])],
            ClosingPageState::Completed => vec![Line::from(vec![
                Span::styled(" [Enter] ", Style::default().fg(Color::DarkGray)),
                Span::styled("確認", Style::default().fg(Color::Gray)),
                Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                Span::styled("[Esc] ", Style::default().fg(Color::DarkGray)),
                Span::styled("戻る", Style::default().fg(Color::Gray)),
            ])],
        };

        let paragraph = Paragraph::new(status_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        frame.render_widget(paragraph, area);
    }
}

impl Default for ClosingPage {
    fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        drop(tx);
        Self::new(rx)
    }
}
