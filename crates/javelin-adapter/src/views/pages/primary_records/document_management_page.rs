// DocumentManagementPage - 証憑管理画面
// 責務: 証憑の一覧表示

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

/// 証憑項目
#[derive(Debug, Clone)]
pub struct DocumentItem {
    pub document_id: String,
    pub document_type: String,
    pub date: String,
    pub description: String,
    pub file_path: String,
}

/// 証憑管理画面
pub struct DocumentManagementPage {
    documents: Vec<DocumentItem>,
    selected_index: usize,
}

impl Default for DocumentManagementPage {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentManagementPage {
    pub fn new() -> Self {
        Self { documents: Vec::new(), selected_index: 0 }
    }

    pub fn set_documents(&mut self, documents: Vec<DocumentItem>) {
        self.documents = documents;
        self.selected_index = 0;
    }

    pub fn select_next(&mut self) {
        if !self.documents.is_empty() && self.selected_index < self.documents.len() - 1 {
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

        let title = Paragraph::new("A-04: Document Management (証憑管理)")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        if self.documents.is_empty() {
            let empty = Paragraph::new("証憑データがありません")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        } else {
            let header = Row::new(vec!["証憑ID", "種類", "日付", "摘要", "ファイルパス"])
                .style(Style::default().add_modifier(Modifier::BOLD));

            let rows: Vec<Row> = self
                .documents
                .iter()
                .enumerate()
                .map(|(i, doc)| {
                    let style = if i == self.selected_index {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };

                    Row::new(vec![
                        Cell::from(doc.document_id.clone()),
                        Cell::from(doc.document_type.clone()),
                        Cell::from(doc.date.clone()),
                        Cell::from(doc.description.clone()),
                        Cell::from(doc.file_path.clone()),
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
                    Constraint::Min(20),
                    Constraint::Min(30),
                ],
            )
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("証憑管理 ({}件)", self.documents.len())),
            );

            frame.render_widget(table, chunks[1]);
        }

        let help = Paragraph::new("[↑↓/jk] Navigate  [Enter] View  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
