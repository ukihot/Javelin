// TrialBalancePageState - PageState implementation for trial balance screen
// Uses ClosingPage which displays trial balance

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::ClosingPage,
};

pub struct TrialBalancePageState {
    page: ClosingPage,
}

impl TrialBalancePageState {
    pub fn new() -> Self {
        // Create channel for trial balance data
        let (_, trial_balance_rx) = tokio::sync::mpsc::unbounded_channel();
        Self { page: ClosingPage::new(trial_balance_rx) }
    }

    fn load_trial_balance(&self, controllers: &Controllers) {
        let generate_trial_balance = controllers.generate_trial_balance.clone();

        // 非同期で試算表を生成
        tokio::spawn(async move {
            // 試算表生成処理
            // 将来的にはプレゼンタ経由でデータを受信
            let _ = generate_trial_balance;
        });
    }
}

impl PageState for TrialBalancePageState {
    fn route(&self) -> Route {
        Route::TrialBalance
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        // 初回データロード
        self.load_trial_balance(controllers);

        loop {
            // Tick animation
            self.page.tick();

            // Update trial balance data
            self.page.update();

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

impl Default for TrialBalancePageState {
    fn default() -> Self {
        Self::new()
    }
}
