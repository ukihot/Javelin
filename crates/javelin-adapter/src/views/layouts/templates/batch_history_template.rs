//! BatchHistoryTemplate - バッチ実行履歴画面の汎用テンプレート
//!
//! バッチ処理の実行履歴を一覧表示します。
//!
//! # 概要
//!
//! このテンプレートは、過去のバッチ実行履歴を表形式で表示し、
//! 各実行の詳細を確認できる機能を提供します。
//!
//! # 使用例
//!
//! ```rust
//! use javelin_adapter::views::layouts::templates::{BatchHistoryItem, BatchHistoryTemplate};
//!
//! let mut template = BatchHistoryTemplate::new("元帳集約処理 - 実行履歴");
//!
//! let history = vec![BatchHistoryItem {
//!     execution_id: "20240224-001".to_string(),
//!     executed_at: "2024-02-24 10:30:00".to_string(),
//!     status: "完了".to_string(),
//!     duration: "2分30秒".to_string(),
//!     processed_count: 150,
//!     result_summary: "正常終了".to_string(),
//! }];
//!
//! template.set_history(history);
//! ```

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::views::components::{EventViewer, LoadingSpinner};

/// バッチ実行履歴項目
#[derive(Debug, Clone)]
pub struct BatchHistoryItem {
    /// 実行ID
    pub execution_id: String,
    /// 実行日時
    pub executed_at: String,
    /// 状態（完了/エラー/実行中）
    pub status: String,
    /// 実行時間
    pub duration: String,
    /// 処理件数
    pub processed_count: usize,
    /// 結果サマリー
    pub result_summary: String,
}

/// ローディング状態
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingState {
    /// 読み込み中
    Loading,
    /// 読み込み完了
    Loaded,
    /// エラー
    Error(String),
}

/// バッチ実行履歴テンプレート
#[allow(dead_code)]
pub struct BatchHistoryTemplate {
    /// タイトル
    title: String,
    /// 実行履歴リスト
    history: Vec<BatchHistoryItem>,
    /// 選択中の履歴インデックス
    selected_index: usize,
    /// ローディング状態
    loading_state: LoadingState,
    /// イベントビューア（詳細ログ表示用）
    event_viewer: EventViewer,
    /// ローディングスピナー
    loading_spinner: LoadingSpinner,
    /// アニメーションフレームカウンター
    animation_frame: usize,
}

impl BatchHistoryTemplate {
    /// 新しいBatchHistoryTemplateを作成
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            history: Vec::new(),
            selected_index: 0,
            loading_state: LoadingState::Loading,
            event_viewer: EventViewer::new(),
            loading_spinner: LoadingSpinner::new(),
            animation_frame: 0,
        }
    }

    /// 実行履歴を設定
    pub fn set_history(&mut self, history: Vec<BatchHistoryItem>) {
        self.history = history;
        self.loading_state = LoadingState::Loaded;
    }

    /// ローディング状態に設定
    pub fn set_loading(&mut self) {
        self.loading_state = LoadingState::Loading;
    }

    /// エラー状態に設定
    pub fn set_error(&mut self, error: impl Into<String>) {
        self.loading_state = LoadingState::Error(error.into());
    }

    /// 情報メッセージを追加
    pub fn add_info(&mut self, message: impl Into<String>) {
        self.event_viewer.add_info(message);
    }

    /// エラーメッセージを追加
    pub fn add_error(&mut self, message: impl Into<String>) {
        self.event_viewer.add_error(message);
    }

    /// 次の履歴を選択
    pub fn select_next(&mut self) {
        if self.selected_index < self.history.len().saturating_sub(1) {
            self.selected_index += 1;
            self.load_execution_detail();
        }
    }

    /// 前の履歴を選択
    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.load_execution_detail();
        }
    }

    /// 選択中の実行の詳細を読み込み
    fn load_execution_detail(&mut self) {
        if let Some(item) = self.history.get(self.selected_index) {
            self.event_viewer.clear();
            self.event_viewer.add_info(format!("実行ID: {}", item.execution_id));
            self.event_viewer.add_info(format!("実行日時: {}", item.executed_at));
            self.event_viewer.add_info(format!("状態: {}", item.status));
            self.event_viewer.add_info(format!("実行時間: {}", item.duration));
            self.event_viewer.add_info(format!("処理件数: {}", item.processed_count));
            self.event_viewer.add_info(format!("結果: {}", item.result_summary));
        }
    }

    /// アニメーションフレームを更新
    pub fn tick(&mut self) {
        self.animation_frame = (self.animation_frame + 1) % 60;
        if self.loading_state == LoadingState::Loading {
            self.loading_spinner.tick();
        }
    }

    /// 履歴テーブルを描画
    fn render_history_table(&self, frame: &mut Frame, area: Rect) {
        // ヘッダー
        let header = Row::new(vec![
            Cell::from("実行ID"),
            Cell::from("実行日時"),
            Cell::from("状態"),
            Cell::from("実行時間"),
            Cell::from("処理件数"),
            Cell::from("結果"),
        ])
        .style(Style::default().add_modifier(Modifier::BOLD));

        // データ行
        let rows: Vec<Row> = self
            .history
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected_index {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };

                let status_color = match item.status.as_str() {
                    "完了" => Color::Green,
                    "エラー" => Color::Red,
                    "実行中" => Color::Yellow,
                    _ => Color::Gray,
                };

                Row::new(vec![
                    Cell::from(item.execution_id.clone()),
                    Cell::from(item.executed_at.clone()),
                    Cell::from(item.status.clone()).style(Style::default().fg(status_color)),
                    Cell::from(item.duration.clone()),
                    Cell::from(item.processed_count.to_string()),
                    Cell::from(item.result_summary.clone()),
                ])
                .style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            vec![
                Constraint::Length(15),
                Constraint::Length(20),
                Constraint::Length(10),
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Min(20),
            ],
        )
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(format!(
            "{} ({}件)",
            self.title,
            self.history.len()
        )));

        frame.render_widget(table, area);
    }

    /// ステータスバーを描画
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let status = "[↑↓] 選択  [e] 新規実行  [Enter] 詳細  [Esc] 戻る";

        let paragraph = Paragraph::new(status).block(Block::default().borders(Borders::ALL));

        frame.render_widget(paragraph, area);
    }

    /// メインレイアウトを描画
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // ローディング中
        if self.loading_state == LoadingState::Loading {
            self.loading_spinner.render(frame, area, "読み込み中...");
            return;
        }

        // エラー表示
        if let LoadingState::Error(error) = &self.loading_state {
            let error_widget = Paragraph::new(error.as_str())
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("エラー"));
            frame.render_widget(error_widget, area);
            return;
        }

        // 水平分割: 履歴テーブル (60%) | 詳細ログ (40%)
        let horizontal_chunks =
            Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(area);

        // 左側の垂直分割: テーブル | ステータスバー
        let left_chunks = Layout::vertical([Constraint::Min(10), Constraint::Length(3)])
            .split(horizontal_chunks[0]);

        // 各セクションを描画
        self.render_history_table(frame, left_chunks[0]);
        self.render_status_bar(frame, left_chunks[1]);

        // 右側: 詳細ログ
        self.event_viewer.render(frame, horizontal_chunks[1]);
    }
}
