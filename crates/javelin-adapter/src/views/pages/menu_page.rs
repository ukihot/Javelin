// MenuPage - Generic menu page component
// Reusable menu page for navigation with retro styling

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{
        Color, Modifier, Style, Stylize,
        palette::tailwind::{BLUE, CYAN, SLATE},
    },
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Wrap,
    },
};

// Color scheme for retro terminal look
const HEADER_BG: Color = BLUE.c800;
const HEADER_FG: Color = SLATE.c100;
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG: Color = SLATE.c900;
const SELECTED_BG: Color = CYAN.c800;
const SELECTED_FG: Color = SLATE.c950;
const TEXT_FG: Color = SLATE.c200;
const DESCRIPTION_FG: Color = SLATE.c500;
const BORDER_FG: Color = CYAN.c600;

/// Generic menu page component
///
/// Displays a list of menu items with selection support.
/// Used for all menu screens (A-01, B-01, C-01, etc.)
pub struct MenuPage {
    title: String,
    items: Vec<MenuItem>,
    state: ListState,
}

struct MenuItem {
    icon: String,
    title: String,
    description: String,
}

impl MenuPage {
    /// Create a new menu page
    ///
    /// # Arguments
    ///
    /// * `title` - Menu title
    /// * `items` - List of (title, description) tuples
    pub fn new(title: &str, items: &[(&str, &str)]) -> Self {
        let menu_items: Vec<MenuItem> = items
            .iter()
            .enumerate()
            .map(|(i, (t, d))| {
                let icon = match i {
                    0 => "📝",
                    1 => "📊",
                    2 => "💰",
                    3 => "📈",
                    4 => "📋",
                    5 => "🔍",
                    6 => "📌",
                    7 => "⚙️",
                    _ => "▸",
                };
                MenuItem {
                    icon: icon.to_string(),
                    title: t.to_string(),
                    description: d.to_string(),
                }
            })
            .collect();

        let mut state = ListState::default();
        state.select(Some(0)); // Select first item by default

        Self { title: title.to_string(), items: menu_items, state }
    }

    /// Select next item
    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    /// Select previous item
    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.state.selected().unwrap_or(0)
    }

    /// Render the menu page
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // Create layout: header + menu list + detail + footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Menu list
                Constraint::Length(5), // Detail area
                Constraint::Length(3), // Footer
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_menu_list(frame, chunks[1]);
        self.render_detail(frame, chunks[2]);
        self.render_footer(frame, chunks[3]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header = Paragraph::new(Line::from(vec![
            Span::raw("  "),
            Span::styled("◆", Style::default().fg(CYAN.c400)),
            Span::raw("  "),
            Span::styled(&self.title, Style::default().fg(HEADER_FG).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled("◆", Style::default().fg(CYAN.c400)),
        ]))
        .style(Style::default().bg(HEADER_BG))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER_FG))
                .padding(Padding::horizontal(1)),
        );

        frame.render_widget(header, area);
    }

    fn render_menu_list(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let bg_color = if i % 2 == 0 {
                    NORMAL_ROW_BG
                } else {
                    ALT_ROW_BG
                };

                let line = Line::from(vec![
                    Span::raw("  "),
                    Span::styled(&item.icon, Style::default().fg(CYAN.c400)),
                    Span::raw("  "),
                    Span::styled(&item.title, Style::default().fg(TEXT_FG)),
                ]);

                ListItem::new(line).bg(bg_color)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(BORDER_FG))
                    .border_set(symbols::border::ROUNDED)
                    .title(Line::from(vec![
                        Span::raw(" "),
                        Span::styled("▼", Style::default().fg(CYAN.c400)),
                        Span::raw(" Menu "),
                    ]))
                    .padding(Padding::horizontal(1)),
            )
            .highlight_style(
                Style::default().bg(SELECTED_BG).fg(SELECTED_FG).add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, frame.buffer_mut(), &mut self.state);
    }

    fn render_detail(&self, frame: &mut Frame, area: Rect) {
        let selected_idx = self.state.selected().unwrap_or(0);
        let selected_item = &self.items[selected_idx];

        let detail_text = vec![
            Line::from(vec![
                Span::styled("▸ ", Style::default().fg(CYAN.c400)),
                Span::styled(
                    &selected_item.title,
                    Style::default().fg(TEXT_FG).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                &selected_item.description,
                Style::default().fg(DESCRIPTION_FG),
            )),
        ];

        let detail = Paragraph::new(detail_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(BORDER_FG))
                    .border_set(symbols::border::ROUNDED)
                    .title(Line::from(vec![
                        Span::raw(" "),
                        Span::styled("ℹ", Style::default().fg(CYAN.c400)),
                        Span::raw(" Details "),
                    ]))
                    .padding(Padding::horizontal(2))
                    .style(Style::default().bg(NORMAL_ROW_BG)),
            )
            .wrap(Wrap { trim: false });

        frame.render_widget(detail, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let help_text = Line::from(vec![
            Span::styled("↑↓", Style::default().fg(CYAN.c400).add_modifier(Modifier::BOLD)),
            Span::styled(" Navigate  ", Style::default().fg(TEXT_FG)),
            Span::styled("Enter", Style::default().fg(CYAN.c400).add_modifier(Modifier::BOLD)),
            Span::styled(" Select  ", Style::default().fg(TEXT_FG)),
            Span::styled("Esc", Style::default().fg(CYAN.c400).add_modifier(Modifier::BOLD)),
            Span::styled(" Back", Style::default().fg(TEXT_FG)),
        ]);

        let footer = Paragraph::new(help_text).centered().block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER_FG))
                .style(Style::default().bg(SLATE.c900)),
        );

        frame.render_widget(footer, area);
    }
}
