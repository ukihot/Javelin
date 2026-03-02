// CashLogListPage - キャッシュログ一覧画面
// 責務: キャッシュログの一覧表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

/// キャッシュログ項目
#[derive(Debug, Clone)]
pub struct CashLogItem {
    pub log_id: String,
    pub date: String,
    pub transaction_type: String,
    pub amount: f64,
    pub balance: f64,
    pub description: String,
}

/// キャッシュログ一覧画面
pub struct CashLogListPage {
    logs: Vec<CashLogItem>,
    selected_index: usize,
}

impl Default for CashLogListPage {
    fn default() -> Self {
        Self::new()
    }
}

impl CashLogListPage {
    pub fn new() -> Self {
        Self { logs: Vec::new(), selected_index: 0 }
    }

    pub fn set_logs(&mut self, logs: Vec<CashLogItem>) {
        self.logs = logs;
        self.selected_index = 0;
    }

    pub fn select_next(&mut self) {
        if !self.logs.is_empty() && self.selected_index < self.logs.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
                .split(frame.area());

        let title = Paragraph::new("A-05: Cash Log List (キャッシュログ一覧)")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        if self.logs.is_empty() {
            let empty = Paragraph::new("キャッシュログデータがありません")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        } else {
            let header = Row::new(vec!["ログID", "日付", "取引種別", "金額", "残高", "摘要"])
                .style(Style::default().add_modifier(Modifier::BOLD));

            let rows: Vec<Row> = self
                .logs
                .iter()
                .enumerate()
                .map(|(i, log)| {
                    let style = if i == self.selected_index {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };

                    Row::new(vec![
                        Cell::from(log.log_id.clone()),
                        Cell::from(log.date.clone()),
                        Cell::from(log.transaction_type.clone()),
                        Cell::from(format!("{:.0}", log.amount)),
                        Cell::from(format!("{:.0}", log.balance)),
                        Cell::from(log.description.clone()),
                    ])
                    .style(style)
                })
                .collect();

            let table = Table::new(
                rows,
                vec![
                    Constraint::Length(10),
                    Constraint::Length(12),
                    Constraint::Length(12),
                    Constraint::Length(15),
                    Constraint::Length(15),
                    Constraint::Min(20),
                ],
            )
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("キャッシュログ一覧 ({}件)", self.logs.len())),
            );

            frame.render_widget(table, chunks[1]);
        }

        let help = Paragraph::new("[↑↓/jk] Navigate  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
