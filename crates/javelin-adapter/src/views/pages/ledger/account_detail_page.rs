// AccountDetailPage - 勘定科目明細画面
// 責務: 勘定科目明細の表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};
use uuid::Uuid;

use crate::presenter::LedgerViewModel;

/// 勘定科目明細画面
pub struct AccountDetailPage {
    id: Uuid,
    ledger_rx: tokio::sync::mpsc::UnboundedReceiver<LedgerViewModel>,
    ledger_data: Option<LedgerViewModel>,
    selected_index: usize,
}

impl AccountDetailPage {
    pub fn new(id: Uuid, ledger_rx: tokio::sync::mpsc::UnboundedReceiver<LedgerViewModel>) -> Self {
        Self { id, ledger_rx, ledger_data: None, selected_index: 0 }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn poll_ledger_data(&mut self) {
        while let Ok(data) = self.ledger_rx.try_recv() {
            self.ledger_data = Some(data);
            self.selected_index = 0;
        }
    }

    pub fn select_next(&mut self) {
        if let Some(ref data) = self.ledger_data
            && !data.entries.is_empty()
            && self.selected_index < data.entries.len() - 1
        {
            self.selected_index += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
                .split(frame.area());

        let title = Paragraph::new("B-02: Account Detail (勘定科目明細)")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        if let Some(ref data) = self.ledger_data {
            let header = Row::new(vec!["日付", "伝票番号", "摘要", "借方", "貸方", "残高"])
                .style(Style::default().add_modifier(Modifier::BOLD));

            let rows: Vec<Row> = data
                .entries
                .iter()
                .enumerate()
                .map(|(i, entry)| {
                    let style = if i == self.selected_index {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };

                    Row::new(vec![
                        Cell::from(entry.transaction_date.clone()),
                        Cell::from(entry.entry_number.clone()),
                        Cell::from(entry.description.clone()),
                        Cell::from(format!("{:.0}", entry.debit_amount)),
                        Cell::from(format!("{:.0}", entry.credit_amount)),
                        Cell::from(format!("{:.0}", entry.balance)),
                    ])
                    .style(style)
                })
                .collect();

            let table = Table::new(
                rows,
                vec![
                    Constraint::Length(12),
                    Constraint::Length(15),
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
                    .title(format!("勘定科目明細 ({}件)", data.entries.len())),
            );

            frame.render_widget(table, chunks[1]);
        } else {
            let loading = Paragraph::new("読み込み中...")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(loading, chunks[1]);
        }

        let help = Paragraph::new("[↑↓/jk] Navigate  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
