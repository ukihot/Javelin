// ValuationResultPage - IFRS評価結果一覧画面
// 責務: IFRS評価結果の表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

/// 評価結果項目
#[derive(Debug, Clone)]
pub struct ValuationResultItem {
    pub asset_id: String,
    pub asset_name: String,
    pub book_value: f64,
    pub fair_value: f64,
    pub adjustment: f64,
}

/// IFRS評価結果一覧画面
pub struct ValuationResultPage {
    results: Vec<ValuationResultItem>,
    selected_index: usize,
}

impl Default for ValuationResultPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ValuationResultPage {
    pub fn new() -> Self {
        Self { results: Vec::new(), selected_index: 0 }
    }

    pub fn set_results(&mut self, results: Vec<ValuationResultItem>) {
        self.results = results;
        self.selected_index = 0;
    }

    pub fn select_next(&mut self) {
        if !self.results.is_empty() && self.selected_index < self.results.len() - 1 {
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

        let title = Paragraph::new("D-07: IFRS Valuation Result (IFRS評価結果一覧)")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        if self.results.is_empty() {
            let empty = Paragraph::new("評価結果データがありません")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        } else {
            let header = Row::new(vec!["資産ID", "資産名", "帳簿価額", "公正価値", "調整額"])
                .style(Style::default().add_modifier(Modifier::BOLD));

            let rows: Vec<Row> = self
                .results
                .iter()
                .enumerate()
                .map(|(i, result)| {
                    let style = if i == self.selected_index {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };

                    Row::new(vec![
                        Cell::from(result.asset_id.clone()),
                        Cell::from(result.asset_name.clone()),
                        Cell::from(format!("{:.0}", result.book_value)),
                        Cell::from(format!("{:.0}", result.fair_value)),
                        Cell::from(format!("{:.0}", result.adjustment)),
                    ])
                    .style(style)
                })
                .collect();

            let table = Table::new(
                rows,
                vec![
                    Constraint::Length(10),
                    Constraint::Min(30),
                    Constraint::Length(15),
                    Constraint::Length(15),
                    Constraint::Length(15),
                ],
            )
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("IFRS評価結果 ({}件)", self.results.len())),
            );

            frame.render_widget(table, chunks[1]);
        }

        let help = Paragraph::new("[↑↓/jk] Navigate  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
