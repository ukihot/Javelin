// CashLogInputPageState - キャッシュログ入力画面
// 責務: キャッシュログの入力

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
};

/// キャッシュログ入力画面
pub struct CashLogInputPageState {}

impl CashLogInputPageState {
    pub fn new() -> Self {
        Self {}
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).split(area);

        let content =
            Paragraph::new("キャッシュログ入力画面\n\n日付、金額、摘要を入力してください")
                .block(Block::default().borders(Borders::ALL).title("キャッシュログ入力"));
        frame.render_widget(content, chunks[0]);

        let footer = Paragraph::new("[Esc] 戻る | [Enter] 保存")
            .block(Block::default().borders(Borders::ALL).title("操作"));
        frame.render_widget(footer, chunks[1]);
    }
}

impl PageState for CashLogInputPageState {
    fn route(&self) -> Route {
        Route::CashLogInput
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            terminal
                .draw(|frame| {
                    self.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            if event::poll(std::time::Duration::from_millis(100))
                .map_err(crate::error::AdapterError::EventReadFailed)?
                && let Event::Key(key) =
                    event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.code == KeyCode::Esc {
                    return Ok(NavAction::Back);
                }
            }
        }
    }
}

impl Default for CashLogInputPageState {
    fn default() -> Self {
        Self::new()
    }
}
