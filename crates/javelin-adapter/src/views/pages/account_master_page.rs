// AccountMasterPage - 勘定科目マスタ画面のビューコンポーネント

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::presenter::AccountMasterItemViewModel;

#[derive(Debug, Clone, PartialEq)]
enum LoadingState {
    Loading,
    Loaded,
    Error(String),
}

pub struct AccountMasterPage {
    accounts: Vec<AccountMasterItemViewModel>,
    current_page: usize,
    items_per_page: usize,
    selected_index: usize,
    loading_state: LoadingState,
}

impl AccountMasterPage {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            current_page: 0,
            items_per_page: 10,
            selected_index: 0,
            loading_state: LoadingState::Loading,
        }
    }

    pub fn set_data(
        &mut self,
        accounts: Vec<AccountMasterItemViewModel>,
        current_page: usize,
        selected_index: usize,
    ) {
        self.accounts = accounts;
        self.current_page = current_page;
        self.selected_index = selected_index;
        self.loading_state = LoadingState::Loaded;
    }

    pub fn set_loading(&mut self) {
        self.loading_state = LoadingState::Loading;
    }

    pub fn set_error(&mut self, error: String) {
        self.loading_state = LoadingState::Error(error);
    }

    pub fn total_items(&self) -> usize {
        self.accounts.len()
    }

    fn current_page_items(&self) -> &[AccountMasterItemViewModel] {
        let start = self.current_page * self.items_per_page;
        let end = (start + self.items_per_page).min(self.accounts.len());
        &self.accounts[start..end]
    }

    pub fn current_page_items_len(&self) -> usize {
        self.current_page_items().len()
    }

    fn total_pages(&self) -> usize {
        if self.accounts.is_empty() {
            1
        } else {
            self.accounts.len().div_ceil(self.items_per_page)
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        if self.loading_state == LoadingState::Loading {
            let loading = Paragraph::new("読み込み中...")
                .block(Block::default().borders(Borders::ALL).title("勘定科目マスタ"));
            frame.render_widget(loading, area);
            return;
        }

        if let LoadingState::Error(error) = &self.loading_state {
            let error_widget = Paragraph::new(error.as_str())
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("エラー"));
            frame.render_widget(error_widget, area);
            return;
        }

        let chunks = Layout::vertical([Constraint::Min(3), Constraint::Length(3)]).split(area);

        // テーブル
        let header = Row::new(vec!["コード", "名称", "種別"])
            .style(Style::default().add_modifier(Modifier::BOLD));

        let rows: Vec<Row> = self
            .current_page_items()
            .iter()
            .enumerate()
            .map(|(i, account)| {
                let style = if i == self.selected_index {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };

                let type_str = &account.account_type_label;

                Row::new(vec![
                    Cell::from(account.code.as_str()),
                    Cell::from(account.name.as_str()),
                    Cell::from(type_str.as_str()),
                ])
                .style(style)
            })
            .collect();

        let table =
            Table::new(rows, [Constraint::Length(10), Constraint::Min(20), Constraint::Length(10)])
                .header(header)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("勘定科目マスタ ({}件)", self.accounts.len())),
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

impl Default for AccountMasterPage {
    fn default() -> Self {
        Self::new()
    }
}
