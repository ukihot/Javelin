// RouAssetListPage - 使用権資産台帳画面
// 責務: 使用権資産の一覧表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::presenter::RouAssetViewModel;

/// 使用権資産台帳画面
pub struct RouAssetListPage {
    assets: Vec<RouAssetViewModel>,
    selected_index: usize,
}

impl Default for RouAssetListPage {
    fn default() -> Self {
        Self::new()
    }
}

impl RouAssetListPage {
    pub fn new() -> Self {
        Self { assets: Vec::new(), selected_index: 0 }
    }

    pub fn set_assets(&mut self, assets: Vec<RouAssetViewModel>) {
        self.assets = assets;
        self.selected_index = 0;
    }

    pub fn select_next(&mut self) {
        if !self.assets.is_empty() && self.selected_index < self.assets.len() - 1 {
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

        let title = Paragraph::new("C-09: ROU Asset Ledger (使用権資産台帳)")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        if self.assets.is_empty() {
            let empty = Paragraph::new("使用権資産データがありません")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        } else {
            let header = Row::new(vec![
                "資産ID",
                "リース契約ID",
                "資産名",
                "初期原価",
                "減価償却累計額",
                "帳簿価額",
                "残存期間",
            ])
            .style(Style::default().add_modifier(Modifier::BOLD));

            let rows: Vec<Row> = self
                .assets
                .iter()
                .enumerate()
                .map(|(i, asset)| {
                    let style = if i == self.selected_index {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };

                    Row::new(vec![
                        Cell::from(asset.asset_id.clone()),
                        Cell::from(asset.lease_contract_id.clone()),
                        Cell::from(asset.asset_name.clone()),
                        Cell::from(format!("{:.0}", asset.initial_cost)),
                        Cell::from(format!("{:.0}", asset.accumulated_depreciation)),
                        Cell::from(format!("{:.0}", asset.carrying_amount)),
                        Cell::from(format!("{}", asset.remaining_term)),
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
                    Constraint::Length(15),
                    Constraint::Length(18),
                    Constraint::Length(15),
                    Constraint::Length(10),
                ],
            )
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("使用権資産台帳 ({}件)", self.assets.len())),
            );

            frame.render_widget(table, chunks[1]);
        }

        let help = Paragraph::new("[↑↓/jk] Navigate  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
