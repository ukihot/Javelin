// LeaseSchedulePage - リース負債スケジュール画面
// 責務: リース負債スケジュールの表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::presenter::LeaseScheduleViewModel;

/// リース負債スケジュール画面
pub struct LeaseSchedulePage {
    schedules: Vec<LeaseScheduleViewModel>,
    selected_index: usize,
}

impl Default for LeaseSchedulePage {
    fn default() -> Self {
        Self::new()
    }
}

impl LeaseSchedulePage {
    pub fn new() -> Self {
        Self { schedules: Vec::new(), selected_index: 0 }
    }

    pub fn set_schedules(&mut self, schedules: Vec<LeaseScheduleViewModel>) {
        self.schedules = schedules;
        self.selected_index = 0;
    }

    pub fn select_next(&mut self) {
        if !self.schedules.is_empty() && self.selected_index < self.schedules.len() - 1 {
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

        let title = Paragraph::new("C-08: Lease Liability Schedule (リース負債スケジュール)")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        if self.schedules.is_empty() {
            let empty = Paragraph::new("リーススケジュールデータがありません")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        } else {
            let header = Row::new(vec!["支払日", "支払額", "元本", "利息", "残高"])
                .style(Style::default().add_modifier(Modifier::BOLD));

            let rows: Vec<Row> = self
                .schedules
                .iter()
                .enumerate()
                .map(|(i, schedule)| {
                    let style = if i == self.selected_index {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };

                    Row::new(vec![
                        Cell::from(schedule.payment_date.clone()),
                        Cell::from(format!("{:.0}", schedule.payment_amount)),
                        Cell::from(format!("{:.0}", schedule.principal)),
                        Cell::from(format!("{:.0}", schedule.interest)),
                        Cell::from(format!("{:.0}", schedule.remaining_balance)),
                    ])
                    .style(style)
                })
                .collect();

            let table = Table::new(
                rows,
                vec![
                    Constraint::Length(12),
                    Constraint::Length(15),
                    Constraint::Length(15),
                    Constraint::Length(15),
                    Constraint::Length(15),
                ],
            )
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("リース負債スケジュール ({}件)", self.schedules.len())),
            );

            frame.render_widget(table, chunks[1]);
        }

        let help = Paragraph::new("[↑↓/jk] Navigate  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
