// AdjustmentJournalListPage - 補正仕訳一覧画面
// 責務: 補正仕訳の一覧表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

/// 補正仕訳項目
#[derive(Debug, Clone)]
pub struct AdjustmentJournalItem {
    pub journal_id: String,
    pub date: String,
    pub description: String,
    pub debit_account: String,
    pub credit_account: String,
    pub amount: f64,
}

/// 補正仕訳一覧画面
pub struct AdjustmentJournalListPage {
    journals: Vec<AdjustmentJournalItem>,
    selected_index: usize,
}

impl Default for AdjustmentJournalListPage {
    fn default() -> Self {
        Self::new()
    }
}

impl AdjustmentJournalListPage {
    pub fn new() -> Self {
        Self { journals: Vec::new(), selected_index: 0 }
    }

    pub fn set_journals(&mut self, journals: Vec<AdjustmentJournalItem>) {
        self.journals = journals;
        self.selected_index = 0;
    }

    pub fn select_next(&mut self) {
        if !self.journals.is_empty() && self.selected_index < self.journals.len() - 1 {
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

        let title = Paragraph::new("D-09: Adjustment Journal List (補正仕訳一覧)")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        if self.journals.is_empty() {
            let empty = Paragraph::new("補正仕訳データがありません")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        } else {
            let header = Row::new(vec!["仕訳ID", "日付", "摘要", "借方科目", "貸方科目", "金額"])
                .style(Style::default().add_modifier(Modifier::BOLD));

            let rows: Vec<Row> = self
                .journals
                .iter()
                .enumerate()
                .map(|(i, journal)| {
                    let style = if i == self.selected_index {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };

                    Row::new(vec![
                        Cell::from(journal.journal_id.clone()),
                        Cell::from(journal.date.clone()),
                        Cell::from(journal.description.clone()),
                        Cell::from(journal.debit_account.clone()),
                        Cell::from(journal.credit_account.clone()),
                        Cell::from(format!("{:.0}", journal.amount)),
                    ])
                    .style(style)
                })
                .collect();

            let table = Table::new(
                rows,
                vec![
                    Constraint::Length(10),
                    Constraint::Length(12),
                    Constraint::Min(20),
                    Constraint::Length(15),
                    Constraint::Length(15),
                    Constraint::Length(15),
                ],
            )
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("補正仕訳一覧 ({}件)", self.journals.len())),
            );

            frame.render_widget(table, chunks[1]);
        }

        let help = Paragraph::new("[↑↓/jk] Navigate  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
