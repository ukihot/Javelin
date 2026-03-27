// RataTUI Risk Radar Screen
// 月次決算コンプライアンス・リスク監視レーダー（レトロ風）

use javelin_application::query_service::{ComplianceRiskSnapshot, RiskMeasurement};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
};

/// リスクレーダー表示コンポーネント
pub struct RiskRadarScreen;

impl RiskRadarScreen {
    /// リスクレーダー画面の全体レイアウト
    pub fn render(f: &mut Frame, snapshot: &ComplianceRiskSnapshot) {
        let size = f.area();

        // メインレイアウト: ヘッダー + ボディ
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(0), Constraint::Length(2)])
            .split(size);

        // ヘッダー
        Self::render_header(f, chunks[0], snapshot);

        // ボディ: 8個のリスク指標
        Self::render_risk_indicators(f, chunks[1], snapshot);

        // フッター: レトロ感を出すため
        Self::render_footer(f, chunks[2], snapshot);
    }

    /// ヘッダー描画
    fn render_header(f: &mut Frame, area: Rect, snapshot: &ComplianceRiskSnapshot) {
        let title_block = Block::default()
            .title_alignment(Alignment::Center)
            .title(format!(
                " ◄ COMPLIANCE RISK RADAR [{}/{}] ► ",
                snapshot.period_year, snapshot.period_month
            ))
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

        let overall_level_color = match snapshot.overall_risk_level.as_str() {
            "Critical" => Color::Red,
            "High" => Color::Yellow,
            "Medium" => Color::Magenta,
            _ => Color::Green,
        };

        let header_text = vec![Line::from(vec![
            Span::styled(
                format!("OVERALL: {}", snapshot.overall_risk_level),
                Style::default().fg(overall_level_color).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled(
                format!("AVG: {:.1}%", snapshot.average_risk_score),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(" | "),
            Span::styled(
                format!("MAX: {:.1}%", snapshot.maximum_risk_score),
                Style::default().fg(Color::Red),
            ),
            Span::raw(" | "),
            Span::styled(
                format!("CRITICAL: {}", snapshot.critical_count),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ])];

        let header = Paragraph::new(header_text).block(title_block).alignment(Alignment::Center);

        f.render_widget(header, area);
    }

    /// 8個のリスク指標レーダー描画
    fn render_risk_indicators(f: &mut Frame, area: Rect, snapshot: &ComplianceRiskSnapshot) {
        // 4x2グリッドレイアウト（2列x4行）
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        // 行ごとに2分割
        for (row_idx, row_area) in vertical_chunks.iter().enumerate() {
            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(*row_area);

            for (col_idx, &col_area) in horizontal_chunks.iter().enumerate() {
                let measurement_idx = row_idx * 2 + col_idx;
                if measurement_idx < snapshot.measurements.len() {
                    Self::render_single_indicator(
                        f,
                        col_area,
                        &snapshot.measurements[measurement_idx],
                    );
                }
            }
        }
    }

    /// 1個のリスク指標を描画
    fn render_single_indicator(f: &mut Frame, area: Rect, measurement: &RiskMeasurement) {
        let level_color = match measurement.level.as_str() {
            "Critical" => Color::Red,
            "High" => Color::Yellow,
            "Medium" => Color::Magenta,
            _ => Color::Green,
        };

        let level_symbol = match measurement.level.as_str() {
            "Critical" => "◆",
            "High" => "▲",
            "Medium" => "■",
            _ => "●",
        };

        let block = Block::default()
            .title(measurement.name.as_str())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .style(Style::default().fg(level_color));

        // ゲージ描画
        let score_f64 = measurement.score.parse::<f64>().unwrap_or(0.0);
        let gauge = Gauge::default()
            .block(block)
            .gauge_style(Style::default().fg(level_color))
            .percent(score_f64 as u16)
            .label(format!(" {} {:.0}% [{:02}]", level_symbol, score_f64, measurement.count));

        f.render_widget(gauge, area);
    }

    /// フッター描画
    fn render_footer(f: &mut Frame, area: Rect, snapshot: &ComplianceRiskSnapshot) {
        let footer_block = Block::default()
            .borders(Borders::TOP)
            .style(Style::default().fg(Color::DarkGray));

        let footer_text = format!(
            " [{:<19}] | Captured: {}",
            "Press 'q' to quit",
            snapshot.captured_at.split('T').next().unwrap_or("")
        );

        let footer = Paragraph::new(footer_text)
            .block(footer_block)
            .style(Style::default().fg(Color::DarkGray));

        f.render_widget(footer, area);
    }
}
