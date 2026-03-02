// FixedAssetListPage - 固定資産一覧画面
// 責務: 固定資産の一覧表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};
use uuid::Uuid;

use crate::presenter::FixedAssetViewModel;

/// 固定資産一覧画面
pub struct FixedAssetListPage {
    id: Uuid,
    fixed_asset_rx: tokio::sync::mpsc::UnboundedReceiver<Vec<FixedAssetViewModel>>,
    assets: Vec<FixedAssetViewModel>,
    selected_index: usize,
}

impl FixedAssetListPage {
    pub fn new(
        id: Uuid,
        fixed_asset_rx: tokio::sync::mpsc::UnboundedReceiver<Vec<FixedAssetViewModel>>,
    ) -> Self {
        Self { id, fixed_asset_rx, assets: Vec::new(), selected_index: 0 }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn poll_assets(&mut self) {
        while let Ok(assets) = self.fixed_asset_rx.try_recv() {
            self.assets = assets;
            self.selected_index = 0;
        }
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

    pub fn render(&mut self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
                .split(frame.area());

        let title = Paragraph::new("C-02: Fixed Asset List (固定資産一覧)")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        if self.assets.is_empty() {
            let empty = Paragraph::new("固定資産データがありません")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        } else {
            let header = Row::new(vec![
                "資産ID",
                "資産名",
                "取得日",
                "取得原価",
                "減価償却累計額",
                "帳簿価額",
                "耐用年数",
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
                        Cell::from(asset.asset_name.clone()),
                        Cell::from(asset.acquisition_date.clone()),
                        Cell::from(format!("{:.0}", asset.acquisition_cost)),
                        Cell::from(format!("{:.0}", asset.accumulated_depreciation)),
                        Cell::from(format!("{:.0}", asset.carrying_amount)),
                        Cell::from(format!("{}", asset.useful_life)),
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
                    Constraint::Length(10),
                ],
            )
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("固定資産一覧 ({}件)", self.assets.len())),
            );

            frame.render_widget(table, chunks[1]);
        }

        let help = Paragraph::new("[↑↓/jk] Navigate  [Enter] Detail  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
