// AccountAdjustmentPageState - PageState implementation for account adjustment history screen

use std::sync::Arc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::BatchHistoryPresenter,
    views::pages::AccountAdjustmentPage,
};

pub struct AccountAdjustmentPageState {
    page: AccountAdjustmentPage,
    page_id: Uuid,
    registry: Arc<PresenterRegistry>,
    result_rx: tokio::sync::mpsc::Receiver<crate::presenter::BatchHistoryViewModel>,
    error_rx: tokio::sync::mpsc::Receiver<String>,
}

impl AccountAdjustmentPageState {
    pub fn new(controllers: &Controllers) -> Self {
        let page_id = Uuid::new_v4();

        let (presenter, channels) = BatchHistoryPresenter::create_channels();
        let presenter_arc = std::sync::Arc::new(presenter);

        let registry = Arc::clone(controllers.batch_history.presenter_registry());
        registry.register_batch_history_presenter(page_id, presenter_arc);

        let mut page = AccountAdjustmentPage::new();
        page.set_loading();

        let controller = std::sync::Arc::clone(&controllers.batch_history);
        let batch_type = "AccountAdjustment".to_string();
        tokio::spawn(async move {
            let _ = controller.handle_get_history(page_id, batch_type).await;
        });

        Self {
            page,
            page_id,
            registry,
            result_rx: channels.result_rx,
            error_rx: channels.error_rx,
        }
    }
}

impl PageState for AccountAdjustmentPageState {
    fn route(&self) -> Route {
        Route::AccountAdjustment
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            if let Ok(result) = self.result_rx.try_recv() {
                let is_empty = result.items.is_empty();
                self.page.set_history(result.items);
                if is_empty {
                    self.page.add_info("実行履歴がありません");
                }
            }

            if let Ok(error) = self.error_rx.try_recv() {
                self.page.set_error(error);
            }

            self.page.tick();

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
                    KeyCode::Char('e') => {
                        return Ok(NavAction::Go(Route::AccountAdjustmentExecution));
                    }
                    KeyCode::Char('j') | KeyCode::Down => self.page.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.page.select_previous(),
                    _ => {}
                }
            }
        }
    }

    fn on_navigation_error(&mut self, error_message: &str) {
        self.page.add_error(error_message);
    }
}

impl Drop for AccountAdjustmentPageState {
    fn drop(&mut self) {
        self.registry.unregister_batch_history_presenter(self.page_id);
    }
}
