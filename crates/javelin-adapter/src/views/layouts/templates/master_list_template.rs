// MasterListTemplate - マスタ一覧画面の汎用テンプレート
// 責務: ページング付きテーブル表示の共通レイアウト

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

/// マスタ一覧画面のデータ項目
pub trait MasterListItem {
    /// テーブルのヘッダーを返す
    fn headers() -> Vec<&'static str>;

    /// テーブルの列幅を返す
    fn column_widths() -> Vec<Constraint>;

    /// テーブルの行データを返す
    fn to_row(&self) -> Vec<String>;
}

/// マスタ一覧画面の状態
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingState {
    Loading,
    Loaded,
    Error(String),
}

/// マスタ一覧画面のテンプレート
pub struct MasterListTemplate<T: MasterListItem> {
    /// タイトル
    title: String,
    /// データ項目
    items: Vec<T>,
    /// 現在のページ番号（0始まり）
    current_page: usize,
    /// 1ページあたりの表示件数
    items_per_page: usize,
    /// 選択中の行インデックス
    selected_index: usize,
    /// ローディング状態
    loading_state: LoadingState,
}

impl<T: MasterListItem> MasterListTemplate<T> {
    /// 新しいテンプレートを作成
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            current_page: 0,
            items_per_page: 10,
            selected_index: 0,
            loading_state: LoadingState::Loading,
        }
    }

    /// データを設定
    pub fn set_data(&mut self, items: Vec<T>, current_page: usize, selected_index: usize) {
        self.items = items;
        self.current_page = current_page;
        self.selected_index = selected_index;
        self.loading_state = LoadingState::Loaded;
    }

    /// ローディング状態を設定
    pub fn set_loading(&mut self) {
        self.loading_state = LoadingState::Loading;
    }

    /// エラー状態を設定
    pub fn set_error(&mut self, error: String) {
        self.loading_state = LoadingState::Error(error);
    }

    /// 総アイテム数を取得
    pub fn total_items(&self) -> usize {
        self.items.len()
    }

    /// 現在のページのアイテムを取得
    fn current_page_items(&self) -> &[T] {
        let start = self.current_page * self.items_per_page;
        let end = (start + self.items_per_page).min(self.items.len());
        &self.items[start..end]
    }

    /// 現在のページのアイテム数を取得
    pub fn current_page_items_len(&self) -> usize {
        self.current_page_items().len()
    }

    /// 総ページ数を取得
    fn total_pages(&self) -> usize {
        if self.items.is_empty() {
            1
        } else {
            self.items.len().div_ceil(self.items_per_page)
        }
    }

    /// 画面を描画
    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        // ローディング中
        if self.loading_state == LoadingState::Loading {
            let loading = Paragraph::new("読み込み中...")
                .block(Block::default().borders(Borders::ALL).title(&*self.title));
            frame.render_widget(loading, area);
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

        // レイアウト分割
        let chunks = Layout::vertical([Constraint::Min(3), Constraint::Length(3)]).split(area);

        // テーブルヘッダー
        let header = Row::new(T::headers().iter().map(|h| Cell::from(*h)))
            .style(Style::default().add_modifier(Modifier::BOLD));

        // テーブル行
        let rows: Vec<Row> = self
            .current_page_items()
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected_index {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };

                let row_data = item.to_row();
                Row::new(row_data.into_iter().map(Cell::from)).style(style)
            })
            .collect();

        // テーブル
        let table = Table::new(rows, T::column_widths()).header(header).block(
            Block::default().borders(Borders::ALL).title(format!(
                "{} ({}件)",
                self.title,
                self.items.len()
            )),
        );

        frame.render_widget(table, chunks[0]);

        // ページング情報
        let page_info = Paragraph::new(format!(
            "ページ {}/{} | [↑↓] 選択 [←→] ページ [Esc] 戻る",
            self.current_page + 1,
            self.total_pages()
        ))
        .block(Block::default().borders(Borders::ALL));

        frame.render_widget(page_info, chunks[1]);
    }
}
