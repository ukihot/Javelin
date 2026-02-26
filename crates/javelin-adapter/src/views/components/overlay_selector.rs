// OverlaySelector - オーバーレイ選択コンポーネント
// 責務: 画面の8割をオーバーレイしてテーブル表示、hjklで選択

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
};

use super::LoadingSpinner;

/// オーバーレイ選択の状態
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlayState {
    /// 非表示
    Hidden,
    /// ローディング中
    Loading,
    /// データ表示中
    Showing,
    /// エラー表示中
    Error(String),
}

/// オーバーレイセレクタ
/// ジェネリックなデータ構造を受け取り、テーブルとして表示
pub struct OverlaySelector {
    title: String,
    state: OverlayState,
    selected_index: usize,
    // データはプレゼンタから受け取る（構造は知らない）
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    loading_spinner: LoadingSpinner,
}

impl OverlaySelector {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            state: OverlayState::Hidden,
            selected_index: 0,
            headers: Vec::new(),
            rows: Vec::new(),
            loading_spinner: LoadingSpinner::new(),
        }
    }

    /// ローディング状態に設定
    pub fn start_loading(&mut self) {
        self.state = OverlayState::Loading;
    }

    /// データを設定（プレゼンタから受け取る）
    pub fn set_data(&mut self, headers: Vec<String>, rows: Vec<Vec<String>>) {
        self.headers = headers;
        self.rows = rows;
        self.state = OverlayState::Showing;
        self.selected_index = 0;
    }

    /// エラー状態に設定
    pub fn set_error(&mut self, message: String) {
        self.state = OverlayState::Error(message);
    }

    /// 非表示に設定
    pub fn hide(&mut self) {
        self.state = OverlayState::Hidden;
        self.selected_index = 0;
    }

    /// 表示中かどうか
    pub fn is_visible(&self) -> bool {
        !matches!(self.state, OverlayState::Hidden)
    }

    /// 状態を取得
    pub fn state(&self) -> OverlayState {
        self.state.clone()
    }

    /// 選択を上に移動
    pub fn select_previous(&mut self) {
        if !matches!(self.state, OverlayState::Showing) {
            return;
        }
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// 選択を下に移動
    pub fn select_next(&mut self) {
        if !matches!(self.state, OverlayState::Showing) {
            return;
        }
        if self.selected_index < self.rows.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// 選択された行を取得
    pub fn selected_row(&self) -> Option<&Vec<String>> {
        if matches!(self.state, OverlayState::Showing) {
            self.rows.get(self.selected_index)
        } else {
            None
        }
    }

    /// ローディングアニメーションを更新
    pub fn tick_loading(&mut self) {
        if matches!(self.state, OverlayState::Loading) {
            self.loading_spinner.tick();
        }
    }

    /// 描画
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if matches!(self.state, OverlayState::Hidden) {
            return;
        }

        // 画面の8割をオーバーレイ
        let overlay_area = Self::centered_rect(80, 80, area);

        // 背景をクリア
        frame.render_widget(Clear, overlay_area);

        match &self.state {
            OverlayState::Loading => self.render_loading(frame, overlay_area),
            OverlayState::Showing => self.render_table(frame, overlay_area),
            OverlayState::Error(msg) => self.render_error(frame, overlay_area, msg),
            OverlayState::Hidden => {}
        }
    }

    /// ローディング画面を描画
    fn render_loading(&self, frame: &mut Frame, area: Rect) {
        // 外枠を描画
        let block = Block::default()
            .title(format!(" {} ", self.title))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        frame.render_widget(block, area);

        // 内側にスピナーを描画
        let inner = area.inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 });
        self.loading_spinner.render(frame, inner, "データ読み込み中...");
    }

    /// エラー画面を描画
    fn render_error(&self, frame: &mut Frame, area: Rect, message: &str) {
        let text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "✗ エラー",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(Span::styled(message, Style::default().fg(Color::Gray))),
            Line::from(""),
            Line::from(Span::styled("Escキーで閉じる", Style::default().fg(Color::DarkGray))),
        ];

        let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(
            Block::default()
                .title(format!(" {} ", self.title))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        );

        frame.render_widget(paragraph, area);
    }

    /// テーブルを描画
    fn render_table(&self, frame: &mut Frame, area: Rect) {
        // エリアを分割：タイトル + テーブル + フッター
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(3), // タイトル
                Constraint::Min(0),    // テーブル
                Constraint::Length(3), // フッター
            ])
            .split(area);

        // タイトル
        let title_text = Line::from(vec![
            Span::styled("v ", Style::default().fg(Color::Cyan)),
            Span::styled(
                &self.title,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("  ({} 件)", self.rows.len()), Style::default().fg(Color::Gray)),
        ]);

        let title_widget = Paragraph::new(title_text).alignment(Alignment::Left).block(
            Block::default()
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(Color::Cyan)),
        );

        frame.render_widget(title_widget, chunks[0]);

        // テーブル
        if self.rows.is_empty() {
            let empty_text = Paragraph::new("データがありません")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::DarkGray))
                .block(
                    Block::default()
                        .borders(Borders::LEFT | Borders::RIGHT)
                        .border_style(Style::default().fg(Color::Cyan)),
                );
            frame.render_widget(empty_text, chunks[1]);
        } else {
            self.render_data_table(frame, chunks[1]);
        }

        // フッター
        let footer_text = Line::from(vec![
            Span::styled("h", Style::default().fg(Color::Yellow)),
            Span::styled(":左 ", Style::default().fg(Color::Gray)),
            Span::styled("j", Style::default().fg(Color::Yellow)),
            Span::styled(":下 ", Style::default().fg(Color::Gray)),
            Span::styled("k", Style::default().fg(Color::Yellow)),
            Span::styled(":上 ", Style::default().fg(Color::Gray)),
            Span::styled("l", Style::default().fg(Color::Yellow)),
            Span::styled(":右 ", Style::default().fg(Color::Gray)),
            Span::styled("| ", Style::default().fg(Color::DarkGray)),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::styled(":選択 ", Style::default().fg(Color::Gray)),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::styled(":キャンセル", Style::default().fg(Color::Gray)),
        ]);

        let footer_widget = Paragraph::new(footer_text).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(Color::Cyan)),
        );

        frame.render_widget(footer_widget, chunks[2]);
    }

    /// データテーブルを描画
    fn render_data_table(&self, frame: &mut Frame, area: Rect) {
        // ヘッダー行
        let header_cells = self
            .headers
            .iter()
            .map(|h| Cell::from(h.as_str()).style(Style::default().fg(Color::Yellow)))
            .collect::<Vec<_>>();
        let header = Row::new(header_cells).style(Style::default().bg(Color::DarkGray)).height(1);

        // データ行
        let rows = self.rows.iter().enumerate().map(|(i, row)| {
            let cells = row.iter().map(|c| Cell::from(c.as_str())).collect::<Vec<_>>();
            let style = if i == self.selected_index {
                Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            Row::new(cells).style(style).height(1)
        });

        // カラム幅を均等に分割
        let column_count = self.headers.len().max(1);
        let widths = vec![Constraint::Percentage((100 / column_count) as u16); column_count];

        let table = Table::new(rows, widths)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .column_spacing(1);

        frame.render_widget(table, area);
    }

    /// 中央に配置されたRectを計算
    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}
