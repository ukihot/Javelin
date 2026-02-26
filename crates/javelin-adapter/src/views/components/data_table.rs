// DataTable - データテーブルコンポーネント
// 責務: 表形式データの表示（Ratatui Tableウィジェット活用）

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Row, Table, TableState},
};

use super::LoadingSpinner;

/// データテーブルの状態
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataTableState {
    /// ローディング中
    Loading,
    /// 進捗メッセージ付きローディング中
    LoadingWithProgress(String),
    /// データ表示中
    Showing,
    /// エラー
    Error(String),
}

/// データテーブル
pub struct DataTable {
    title: String,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    column_widths: Vec<u16>,
    table_state: TableState,
    highlight_style: Style,
    state: DataTableState,
    loading_spinner: LoadingSpinner,
}

impl DataTable {
    pub fn new(title: impl Into<String>, headers: Vec<String>) -> Self {
        let column_count = headers.len();
        Self {
            title: title.into(),
            headers,
            rows: Vec::new(),
            column_widths: vec![15; column_count],
            table_state: TableState::default(),
            highlight_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            state: DataTableState::Loading,
            loading_spinner: LoadingSpinner::new(),
        }
    }

    pub fn with_rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.rows = rows;
        self.state = DataTableState::Showing;
        self
    }

    pub fn with_column_widths(mut self, widths: Vec<u16>) -> Self {
        self.column_widths = widths;
        self
    }

    /// ローディング状態に設定
    pub fn start_loading(&mut self) {
        self.state = DataTableState::Loading;
    }

    /// 進捗メッセージ付きローディング状態に設定
    pub fn set_loading_progress(&mut self, message: String) {
        self.state = DataTableState::LoadingWithProgress(message);
    }

    /// データを設定
    pub fn set_data(&mut self, rows: Vec<Vec<String>>) {
        self.rows = rows;
        self.state = DataTableState::Showing;
    }

    /// エラー状態に設定
    pub fn set_error(&mut self, message: String) {
        self.state = DataTableState::Error(message);
    }

    /// ローディングアニメーションを更新
    pub fn tick_loading(&mut self) {
        if matches!(self.state, DataTableState::Loading | DataTableState::LoadingWithProgress(_)) {
            self.loading_spinner.tick();
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn select_next(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.rows.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn select_previous(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.rows.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.table_state.selected()
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        match &self.state {
            DataTableState::Loading => self.render_loading(frame, area, "データ読み込み中..."),
            DataTableState::LoadingWithProgress(msg) => self.render_loading(frame, area, msg),
            DataTableState::Showing => self.render_table(frame, area),
            DataTableState::Error(msg) => self.render_error(frame, area, msg),
        }
    }

    /// ローディング画面を描画
    fn render_loading(&self, frame: &mut Frame, area: Rect, message: &str) {
        // 外枠を描画
        let block = Block::default()
            .title(self.title.as_str())
            .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(Color::Cyan));

        frame.render_widget(block, area);

        // 内側にスピナーを描画
        let inner = area.inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 });
        self.loading_spinner.render(frame, inner, message);
    }

    /// エラー画面を描画
    fn render_error(&self, frame: &mut Frame, area: Rect, message: &str) {
        use ratatui::{
            layout::Alignment,
            text::{Line, Span},
            widgets::Paragraph,
        };

        let text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "✗ データ取得エラー",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(Span::styled(message, Style::default().fg(Color::Gray))),
        ];

        let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(
            Block::default()
                .title(self.title.as_str())
                .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(Color::Red)),
        );

        frame.render_widget(paragraph, area);
    }

    /// テーブルを描画
    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        // ヘッダー行
        let header = Row::new(self.headers.clone())
            .style(Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD))
            .height(1);

        // データ行
        let rows: Vec<Row> = self
            .rows
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let style = if i % 2 == 0 {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::Gray)
                };
                Row::new(row.clone()).style(style)
            })
            .collect();

        // カラム幅の制約
        let constraints: Vec<ratatui::layout::Constraint> = self
            .column_widths
            .iter()
            .map(|&w| ratatui::layout::Constraint::Length(w))
            .collect();

        // テーブル
        let table = Table::new(rows, constraints)
            .header(header)
            .row_highlight_style(self.highlight_style)
            .highlight_symbol("> ")
            .block(
                Block::default()
                    .title(self.title.as_str())
                    .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(Color::Cyan)),
            );

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }
}
