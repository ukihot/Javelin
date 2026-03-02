// GeneralLedgerPageState - 総勘定元帳画面
// 責務: 総勘定元帳のデータ管理とライフサイクル

use std::sync::Arc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::LedgerPresenter,
    views::pages::GeneralLedgerPage,
};

/// 総勘定元帳画面
pub struct GeneralLedgerPageState {
    page_id: Uuid,
    page: GeneralLedgerPage,
    presenter_registry: Arc<PresenterRegistry>,
}

impl GeneralLedgerPageState {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        let page_id = Uuid::new_v4();

        let (ledger_tx, ledger_rx, trial_balance_tx, _trial_balance_rx) =
            LedgerPresenter::create_channels();

        let presenter = LedgerPresenter::new(ledger_tx, trial_balance_tx);
        presenter_registry.register_ledger_presenter(page_id, Arc::new(presenter));

        let page = GeneralLedgerPage::new(page_id, ledger_rx);

        Self { page_id, page, presenter_registry }
    }

    fn load_data(&self, controllers: &Controllers) {
        let ledger_controller = controllers.ledger.clone();

        tokio::spawn(async move {
            use javelin_application::query_service::GetLedgerQuery;

            let query = GetLedgerQuery {
                account_code: "0000".to_string(),
                from_date: None,
                to_date: None,
                limit: Some(100),
                offset: None,
            };

            let _ = ledger_controller.get_ledger(query).await;
        });
    }
}

impl Drop for GeneralLedgerPageState {
    fn drop(&mut self) {
        self.presenter_registry.unregister_ledger_presenter(self.page_id);
    }
}

impl PageState for GeneralLedgerPageState {
    fn route(&self) -> Route {
        Route::GeneralLedger
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        self.load_data(controllers);

        loop {
            self.page.poll_ledger_data();

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

impl Default for GeneralLedgerPageState {
    fn default() -> Self {
        Self::new(Arc::new(PresenterRegistry::new()))
    }
}
