// JournalListPage - 仕訳検索・一覧画面
// 責務: 仕訳の検索結果を一覧表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use uuid::Uuid;

use crate::{
    presenter::{JournalEntryItemViewModel, SearchChannels, SearchResultViewModel},
    views::components::loading_spinner::LoadingSpinner,
};

/// 仕訳検索・一覧画面
pub struct JournalListPage {
    id: Uuid,
    channels: SearchChannels,
    search_results: Option<SearchResultViewModel>,
    list_state: ListState,
    loading_spinner: LoadingSpinner,
    error_message: Option<String>,
    progress_message: Option<String>,
    execution_time: Option<usize>,
    pending_search: bool,
    is_loading: bool,
}

impl JournalListPage {
    pub fn new(id: Uuid, channels: SearchChannels) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            id,
            channels,
            search_results: None,
            list_state,
            loading_spinner: LoadingSpinner::new(),
            error_message: None,
            progress_message: None,
            execution_time: None,
            pending_search: true, // 初回自動検索
            is_loading: false,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    /// 検索結果をポーリング
    pub fn poll_search_results(&mut self) {
        // 検索結果を受信
        while let Ok(result) = self.channels.result_rx.try_recv() {
            self.search_results = Some(result);
            self.is_loading = false;
            self.error_message = None;

            // リストの選択状態をリセット
            if let Some(ref results) = self.search_results
                && !results.items.is_empty()
            {
                self.list_state.select(Some(0));
            }
        }

        // エラーメッセージを受信
        while let Ok(error) = self.channels.error_rx.try_recv() {
            self.error_message = Some(error);
            self.is_loading = false;
        }

        // 進捗メッセージを受信
        while let Ok(progress) = self.channels.progress_rx.try_recv() {
            self.progress_message = Some(progress);
        }

        // 実行時間を受信
        while let Ok(time) = self.channels.execution_time_rx.try_recv() {
            self.execution_time = Some(time);
        }
    }

    /// 検索が保留中か
    pub fn has_pending_search(&self) -> bool {
        self.pending_search
    }

    /// 検索保留をクリア
    pub fn clear_pending_search(&mut self) {
        self.pending_search = false;
        self.is_loading = true;
        self.error_message = None;
        self.progress_message = None;
    }

    /// 検索をトリガー
    pub fn trigger_search(&mut self) {
        self.pending_search = true;
    }

    /// 次の項目を選択
    pub fn select_next(&mut self) {
        if let Some(ref results) = self.search_results {
            if results.items.is_empty() {
                return;
            }

            let i = match self.list_state.selected() {
                Some(i) => {
                    if i >= results.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }

    /// 前の項目を選択
    pub fn select_previous(&mut self) {
        if let Some(ref results) = self.search_results {
            if results.items.is_empty() {
                return;
            }

            let i = match self.list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        results.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }

    /// アニメーションフレームを更新
    pub fn tick(&mut self) {
        if self.is_loading {
            self.loading_spinner.tick();
        }
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(3), // タイトル
            Constraint::Min(0),    // リスト
            Constraint::Length(3), // ヘルプ
        ])
        .split(frame.area());

        // タイトル
        let title_text = if let Some(time) = self.execution_time {
            format!("A-03: Journal Entry Search / List (仕訳検索・一覧) - 実行時間: {}ms", time)
        } else {
            "A-03: Journal Entry Search / List (仕訳検索・一覧)".to_string()
        };

        let title = Paragraph::new(title_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        // リスト
        if self.is_loading {
            let loading_message = if let Some(ref progress) = self.progress_message {
                progress.as_str()
            } else {
                "検索中..."
            };

            self.loading_spinner.render(frame, chunks[1], loading_message);
        } else if let Some(ref error) = self.error_message {
            let error_widget = Paragraph::new(vec![
                Line::from(Span::styled("エラーが発生しました:", Style::default().fg(Color::Red))),
                Line::from(""),
                Line::from(error.as_str()),
                Line::from(""),
                Line::from("[s]キーで再検索してください。"),
            ])
            .block(Block::default().borders(Borders::ALL).title("Error"))
            .style(Style::default().fg(Color::Red));
            frame.render_widget(error_widget, chunks[1]);
        } else if let Some(ref results) = self.search_results {
            if results.items.is_empty() {
                let empty = Paragraph::new(vec![
                    Line::from("検索結果がありません。"),
                    Line::from(""),
                    Line::from("[s]キーで検索を実行してください。"),
                ])
                .block(Block::default().borders(Borders::ALL).title("Search Results"))
                .style(Style::default().fg(Color::Gray));
                frame.render_widget(empty, chunks[1]);
            } else {
                let items: Vec<ListItem> = results
                    .items
                    .iter()
                    .map(|item| Self::format_journal_entry_item(item))
                    .collect();

                let list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(format!("Search Results ({}件)", results.total_count)),
                    )
                    .highlight_style(
                        Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                frame.render_stateful_widget(list, chunks[1], &mut self.list_state);
            }
        } else {
            let empty = Paragraph::new(vec![
                Line::from("検索結果がありません。"),
                Line::from(""),
                Line::from("[s]キーで検索を実行してください。"),
            ])
            .block(Block::default().borders(Borders::ALL).title("Search Results"))
            .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        }

        // ヘルプ
        let help = Paragraph::new("[s] Search  [↑↓/jk] Navigate  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }

    /// 仕訳項目をフォーマット
    fn format_journal_entry_item(item: &JournalEntryItemViewModel) -> ListItem<'_> {
        // 合計金額を計算
        let total: f64 = item.lines.iter().map(|line| line.amount).sum();

        let line = Line::from(vec![
            Span::styled(format!("{} ", item.transaction_date), Style::default().fg(Color::Green)),
            Span::styled(
                format!("{:<10} ", item.entry_number.as_deref().unwrap_or("-")),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(format!("{:<10} ", item.status_label), Style::default().fg(Color::Yellow)),
            Span::styled(format!("¥{:>12.0}", total), Style::default().fg(Color::White)),
        ]);

        ListItem::new(line)
    }
}
