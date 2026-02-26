// ClosingMenuPageState - D-01: Monthly Closing Menu (Close Calendar)

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::MenuPage,
};

pub struct ClosingMenuPageState {
    page: MenuPage,
}

impl ClosingMenuPageState {
    pub fn new() -> Self {
        let menu_items = vec![
            ("Closing Preparation Execution", "締準備処理実行"),
            ("Closing Preparation History", "締準備実行履歴"),
            ("Closing Lock", "締日固定処理実行"),
            ("Trial Balance", "試算表表示"),
            ("Account Adjustment Execution", "勘定補正処理実行"),
            ("Account Adjustment History", "勘定補正実行履歴"),
            ("Valuation Execution", "評価処理実行"),
            ("Valuation History", "評価処理実行履歴"),
            ("Notes Draft", "注記草案表示"),
            ("Financial Statement Generation", "財務諸表生成実行"),
            ("Financial Statement History", "財務諸表生成履歴"),
        ];

        Self { page: MenuPage::new("D. Monthly Closing (Close Calendar)", &menu_items) }
    }
}

impl Default for ClosingMenuPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl PageState for ClosingMenuPageState {
    fn route(&self) -> Route {
        Route::ClosingMenu
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

            if let Event::Key(key) =
                event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => return Ok(NavAction::Back),
                    KeyCode::Char('j') | KeyCode::Down => self.page.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.page.select_previous(),
                    KeyCode::Enter => {
                        let route = match self.page.selected_index() {
                            0 => Route::ClosingPreparationExecution,           // D-02
                            1 => Route::ClosingPreparation,                    // Legacy: History
                            2 => Route::ClosingLockExecution,                  // D-04
                            3 => Route::TrialBalance,                          // D-06
                            4 => Route::AccountAdjustmentExecution,            // D-07
                            5 => Route::AccountAdjustment,                     // Legacy: History
                            6 => Route::ValuationExecution,                    // D-09
                            7 => Route::IfrsValuation,                         // Legacy: History
                            8 => Route::NotesDraft,                            // D-12
                            9 => Route::FinancialStatementGenerationExecution, // D-13
                            10 => Route::FinancialStatement,                   // Legacy: History
                            _ => continue,
                        };
                        return Ok(NavAction::Go(route));
                    }
                    _ => {}
                }
            }
        }
    }
}
