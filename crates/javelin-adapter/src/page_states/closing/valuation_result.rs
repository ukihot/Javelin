// ValuationResultPageState - IFRS評価結果一覧画面
// 責務: IFRS評価結果のデータ管理とライフサイクル

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::ValuationResultPage,
};

pub struct ValuationResultPageState {
    page: ValuationResultPage,
}

impl ValuationResultPageState {
    pub fn new() -> Self {
        Self { page: ValuationResultPage::new() }
    }
}

impl PageState for ValuationResultPageState {
    fn route(&self) -> Route {
        Route::ValuationResult
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            terminal
                .draw(|frame| {
                    self.page.render(frame);
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

                match key.code {
                    KeyCode::Esc => return Ok(NavAction::Back),
                    KeyCode::Char('j') | KeyCode::Down => self.page.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.page.select_previous(),
                    _ => {}
                }
            }
        }
    }
}

impl Default for ValuationResultPageState {
    fn default() -> Self {
        Self::new()
    }
}
