// Calendar - カレンダーコンポーネント
// 責務: 月次カレンダーの表示（ratatui公式ウィジェット）

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::calendar::{CalendarEventStore, Monthly},
};
use time::{Date, Month, OffsetDateTime};

/// カレンダーコンポーネント
pub struct Calendar {
    /// 表示する年月（Noneの場合は現在の年月）
    pub year_month: Option<(i32, u32)>,
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

impl Calendar {
    pub fn new() -> Self {
        Self { year_month: None }
    }

    /// 年月を指定
    pub fn with_year_month(mut self, year: i32, month: u32) -> Self {
        self.year_month = Some((year, month));
        self
    }

    /// カレンダーを描画
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        use ratatui::layout::{Constraint, Direction, Layout};

        // 現在の日付を取得
        let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());

        let (year, month_num) = self.year_month.unwrap_or_else(|| {
            let date = now.date();
            (date.year(), date.month() as u32)
        });

        // u32からtime::Monthに変換
        let month = match month_num {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => Month::January,
        };

        // 月の最初の日を作成
        let date = Date::from_calendar_date(year, month, 1).unwrap();

        // イベントストアを作成（今日の日付を強調）
        let event_store = CalendarEventStore::today(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

        // カレンダーを中央寄せするためのレイアウト
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(area);

        // カレンダーウィジェットを作成
        let calendar = Monthly::new(date, event_store)
            .show_month_header(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .show_weekdays_header(Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD))
            .default_style(Style::default().fg(Color::White).bg(Color::Rgb(50, 50, 50)))
            .show_surrounding(Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM));

        frame.render_widget(calendar, horizontal_chunks[1]);
    }
}
