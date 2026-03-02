// LeaseContractListPage - リース契約一覧画面
// 責務: リース契約の一覧表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::presenter::LeaseContractViewModel;

/// リース契約一覧画面
pub struct LeaseContractListPage {
    contracts: Vec<LeaseContractViewModel>,
    selected_index: usize,
}

impl Default for LeaseContractListPage {
    fn default() -> Self {
        Self::new()
    }
}

impl LeaseContractListPage {
    pub fn new() -> Self {
        Self { contracts: Vec::new(), selected_index: 0 }
    }

    pub fn set_contracts(&mut self, contracts: Vec<LeaseContractViewModel>) {
        self.contracts = contracts;
        self.selected_index = 0;
    }

    pub fn select_next(&mut self) {
        if !self.contracts.is_empty() && self.selected_index < self.contracts.len() - 1 {
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

        let title = Paragraph::new("C-06: Lease Contract List (リース契約一覧)")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        if self.contracts.is_empty() {
            let empty = Paragraph::new("リース契約データがありません")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        } else {
            let header = Row::new(vec![
                "契約ID",
                "貸主",
                "資産名",
                "開始日",
                "終了日",
                "月額支払額",
                "総負債額",
            ])
            .style(Style::default().add_modifier(Modifier::BOLD));

            let rows: Vec<Row> = self
                .contracts
                .iter()
                .enumerate()
                .map(|(i, contract)| {
                    let style = if i == self.selected_index {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };

                    Row::new(vec![
                        Cell::from(contract.contract_id.clone()),
                        Cell::from(contract.lessor.clone()),
                        Cell::from(contract.asset_name.clone()),
                        Cell::from(contract.start_date.clone()),
                        Cell::from(contract.end_date.clone()),
                        Cell::from(format!("{:.0}", contract.monthly_payment)),
                        Cell::from(format!("{:.0}", contract.total_liability)),
                    ])
                    .style(style)
                })
                .collect();

            let table = Table::new(
                rows,
                vec![
                    Constraint::Length(10),
                    Constraint::Length(15),
                    Constraint::Min(20),
                    Constraint::Length(12),
                    Constraint::Length(12),
                    Constraint::Length(15),
                    Constraint::Length(15),
                ],
            )
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("リース契約一覧 ({}件)", self.contracts.len())),
            );

            frame.render_widget(table, chunks[1]);
        }

        let help = Paragraph::new("[↑↓/jk] Navigate  [Enter] Detail  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
