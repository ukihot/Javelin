// MaterialityEvaluationPageState - 重要性判定画面
// 責務: 重要性判定処理の実行と結果表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    presenter::MaterialityEvaluationPresenter,
    views::pages::MaterialityEvaluationPage,
};

pub struct MaterialityEvaluationPageState {
    page: MaterialityEvaluationPage,
}

impl MaterialityEvaluationPageState {
    pub fn new() -> Self {
        let (result_tx, result_rx, progress_tx, progress_rx) =
            MaterialityEvaluationPresenter::create_channels();

        // 未使用のチャネルを保持（Presenterが使用）
        let _ = (result_tx, progress_tx);

        Self { page: MaterialityEvaluationPage::new(result_rx, progress_rx) }
    }
}

impl PageState for MaterialityEvaluationPageState {
    fn route(&self) -> Route {
        Route::MaterialityEvaluation
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            // Tick animation
            self.page.tick();

            // Update page data
            self.page.update(controllers);

            // Render the page
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // Handle events with timeout for animation updates
            if event::poll(std::time::Duration::from_millis(100))
                .map_err(crate::error::AdapterError::EventReadFailed)?
                && let Event::Key(key) =
                    event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => {
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Char('s') => {
                        self.page.start_evaluation(controllers);
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.page.select_next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.page.select_previous();
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for MaterialityEvaluationPageState {
    fn default() -> Self {
        Self::new()
    }
}
