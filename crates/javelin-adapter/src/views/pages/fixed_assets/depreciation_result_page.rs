// DepreciationResultPage - 償却計算結果一覧画面
// 責務: 減価償却計算結果の表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};
use uuid::Uuid;

use crate::presenter::DepreciationResultViewModel;

/// 償却計算結果一覧画面
pub struct DepreciationResultPage {
    id: Uuid,
    depreciation_result_rx: tokio::sync::mpsc::UnboundedReceiver<Vec<DepreciationResultViewModel>>,
    results: Vec<DepreciationResultViewModel>,
    selected_index: usize,
}

impl DepreciationResultPage {
    pub fn new(
        id: Uuid,
        depreciation_result_rx: tokio::sync::mpsc::UnboundedReceiver<
            Vec<DepreciationResultViewModel>,
        >,
    ) -> Self {
        Self { id, depreciation_result_rx, results: Vec::new(), selected_index: 0 }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn poll_results(&mut self) {
        while let Ok(results) = self.depreciation_result_rx.try_recv() {
            self.results = results;
            self.selected_index = 0;
        }
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

    pub fn render(&mut self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
                .split(frame.area());

        let title = Paragraph::new("C-05: Depreciation Result (償却計算結果一覧)")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        if self.results.is_empty() {
            let empty = Paragraph::new("償却計算結果がありません")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        } else {
            let header = Row::new(vec![
                "資産ID",
                "資産名",
                "償却方法",
                "償却額",
                "減価償却累計額",
                "帳簿価額",
            ])
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
                        Cell::from(result.depreciation_method.clone()),
                        Cell::from(format!("{:.0}", result.depreciation_amount)),
                        Cell::from(format!("{:.0}", result.accumulated_depreciation)),
                        Cell::from(format!("{:.0}", result.carrying_amount)),
                    ])
                    .style(style)
                })
                .collect();

            let table = Table::new(
                rows,
                vec![
                    Constraint::Length(10),
                    Constraint::Min(20),
                    Constraint::Length(12),
                    Constraint::Length(15),
                    Constraint::Length(18),
                    Constraint::Length(15),
                ],
            )
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("償却計算結果 ({}件)", self.results.len())),
            );

            frame.render_widget(table, chunks[1]);
        }

        let help = Paragraph::new("[↑↓/jk] Navigate  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
