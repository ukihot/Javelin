// ComprehensiveFinancialStatementsPageState - 包括的財務諸表生成画面
// 責務: 包括的財務諸表生成処理の実行と結果表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::ComprehensiveFinancialStatementsPage,
};

pub struct ComprehensiveFinancialStatementsPageState {
    page: ComprehensiveFinancialStatementsPage,
}

impl ComprehensiveFinancialStatementsPageState {
    pub fn new() -> Self {
        Self { page: ComprehensiveFinancialStatementsPage::new() }
    }
}

impl PageState for ComprehensiveFinancialStatementsPageState {
    fn route(&self) -> Route {
        Route::ComprehensiveFinancialStatements
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
                        self.page.start_generation(controllers);
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

impl Default for ComprehensiveFinancialStatementsPageState {
    fn default() -> Self {
        Self::new()
    }
}
