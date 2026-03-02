// LedgerConsistencyVerificationPageState - 元帳整合性検証画面
// 責務: 元帳整合性検証処理の実行と結果表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    presenter::LedgerConsistencyVerificationPresenter,
    views::pages::LedgerConsistencyVerificationPage,
};

pub struct LedgerConsistencyVerificationPageState {
    page: LedgerConsistencyVerificationPage,
}

impl LedgerConsistencyVerificationPageState {
    pub fn new() -> Self {
        let (result_tx, result_rx, progress_tx, progress_rx) =
            LedgerConsistencyVerificationPresenter::create_channels();
        let _ = (result_tx, progress_tx);
        Self { page: LedgerConsistencyVerificationPage::new(result_rx, progress_rx) }
    }
}

impl PageState for LedgerConsistencyVerificationPageState {
    fn route(&self) -> Route {
        Route::LedgerConsistencyVerification
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            self.page.tick();
            self.page.update(controllers);

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
                    KeyCode::Char('s') => self.page.start_verification(controllers),
                    KeyCode::Char('j') | KeyCode::Down => self.page.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.page.select_previous(),
                    _ => {}
                }
            }
        }
    }
}

impl Default for LedgerConsistencyVerificationPageState {
    fn default() -> Self {
        Self::new()
    }
}
