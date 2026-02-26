// ListSelector - リスト選択コンポーネント
// 責務: メニューやリスト選択（Ratatui Listウィジェット活用）

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
};

/// リスト項目
#[derive(Clone)]
pub struct ListItemData {
    pub code: String,
    pub label: String,
    pub description: String,
}

impl ListItemData {
    pub fn new(
        code: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self { code: code.into(), label: label.into(), description: description.into() }
    }
}

/// リスト選択コンポーネント
pub struct ListSelector {
    title: String,
    items: Vec<ListItemData>,
    state: ListState,
    is_active: bool,
}

impl ListSelector {
    pub fn new(title: impl Into<String>, items: Vec<ListItemData>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }

        Self { title: title.into(), items, state, is_active: false }
    }

    /// アクティブ状態を設定
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select_previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn selected_item(&self) -> Option<&ListItemData> {
        self.state.selected().and_then(|i| self.items.get(i))
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let selected_idx = self.state.selected();

        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_selected = Some(i) == selected_idx;

                let line = Line::from(vec![
                    Span::styled(
                        format!("[{}] ", item.code),
                        if is_selected {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::Yellow)
                        },
                    ),
                    Span::styled(
                        &item.label,
                        if is_selected {
                            Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        },
                    ),
                    Span::styled(
                        format!("  - {}", item.description),
                        if is_selected {
                            Style::default().fg(Color::Gray)
                        } else {
                            Style::default().fg(Color::DarkGray)
                        },
                    ),
                ]);

                ListItem::new(line)
            })
            .collect();

        // アクティブ状態に応じてスタイルを変更
        let (title_color, border_color) = if self.is_active {
            (Color::Cyan, Color::Cyan)
        } else {
            (Color::DarkGray, Color::DarkGray)
        };

        let title_text = if self.is_active {
            format!("▶ {}", self.title)
        } else {
            self.title.clone()
        };

        let list = List::new(list_items)
            .block(
                Block::default()
                    .title(title_text)
                    .title_style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(border_color)),
            )
            .highlight_style(
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
