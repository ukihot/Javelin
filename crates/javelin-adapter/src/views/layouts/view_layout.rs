// ViewLayout - データ閲覧レイアウト
// 責務: 帳票・レポート閲覧画面の構造定義
// デザイン: レトロな基幹システム風の閲覧画面

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

/// 閲覧レイアウト
pub struct ViewLayout {
    title: String,
    report_code: String,
    period: String,
    record_count: usize,
}

impl ViewLayout {
    pub fn new(
        title: impl Into<String>,
        report_code: impl Into<String>,
        period: impl Into<String>,
        record_count: usize,
    ) -> Self {
        Self {
            title: title.into(),
            report_code: report_code.into(),
            period: period.into(),
            record_count,
        }
    }

    /// レイアウトを描画
    pub fn render<F>(&self, frame: &mut Frame, render_content: F)
    where
        F: FnOnce(&mut Frame, Rect),
    {
        let size = frame.area();

        // 全体レイアウト: ヘッダー、コンテンツ、フッター
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(5), // ヘッダー
                Constraint::Min(10),   // コンテンツエリア
                Constraint::Length(3), // フッター
            ])
            .split(size);

        self.render_header(frame, chunks[0]);
        render_content(frame, chunks[1]);
        self.render_footer(frame, chunks[2]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(2)])
            .split(area);

        // タイトル行
        let title_line = Line::from(vec![
            Span::styled("◆ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(
                &self.title,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("  [{}]", self.report_code), Style::default().fg(Color::Yellow)),
        ]);

        let title = Paragraph::new(title_line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Cyan)),
        );

        frame.render_widget(title, header_chunks[0]);

        // 情報行
        let info_line = Line::from(vec![
            Span::styled("対象期間: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&self.period, Style::default().fg(Color::Yellow)),
            Span::styled("  |  ", Style::default().fg(Color::DarkGray)),
            Span::styled("件数: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}", self.record_count), Style::default().fg(Color::Green)),
            Span::styled(" 件", Style::default().fg(Color::DarkGray)),
        ]);

        let info = Paragraph::new(info_line).alignment(Alignment::Left);

        frame.render_widget(info, header_chunks[1]);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let footer_text = Line::from(vec![
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("↑↓", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("]スクロール ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "PgUp/PgDn",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::styled("]ページ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("f", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("]検索 ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("e", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("]出力 ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("]戻る", Style::default().fg(Color::DarkGray)),
        ]);

        let footer = Paragraph::new(footer_text).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        frame.render_widget(footer, area);
    }
}

/// 閲覧用のサマリー＋詳細レイアウト
pub fn summary_detail_layout(area: Rect, summary_height: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(summary_height), Constraint::Min(0)])
        .split(area)
        .to_vec()
}

/// 閲覧用の3ペインレイアウト（ナビ、メイン、詳細）
pub fn three_pane_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // ナビゲーション
            Constraint::Percentage(50), // メインコンテンツ
            Constraint::Percentage(30), // 詳細情報
        ])
        .split(area)
        .to_vec()
}
