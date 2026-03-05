// 請求書印刷画面のページステート

use std::sync::Arc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::invoice_print_presenter::{InvoicePrintPresenter, InvoicePrintViewModel},
    views::pages::InvoicePrintPage,
};

/// 請求書印刷ページステート
pub struct InvoicePrintPageState {
    id: Uuid,
    view_model_receiver: mpsc::UnboundedReceiver<InvoicePrintViewModel>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl InvoicePrintPageState {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        let id = Uuid::new_v4();

        // チャネルを作成
        let (tx, rx) = mpsc::unbounded_channel();

        // プレゼンターを作成して登録
        let presenter = Arc::new(InvoicePrintPresenter::new(tx));
        presenter_registry.register_invoice_print_presenter(id, presenter);

        Self { id, view_model_receiver: rx, presenter_registry }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl Drop for InvoicePrintPageState {
    fn drop(&mut self) {
        // ページが破棄される時にプレゼンターを登録解除
        self.presenter_registry.unregister_invoice_print_presenter(self.id);
    }
}

impl PageState for InvoicePrintPageState {
    fn route(&self) -> Route {
        Route::InvoicePrint
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        let mut page = InvoicePrintPage::new();

        loop {
            // プレゼンターからの通知をポーリング
            while let Ok(view_model) = self.view_model_receiver.try_recv() {
                page.set_status(&view_model.message);
            }

            // Render the page
            terminal
                .draw(|frame| {
                    page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // Handle events
            if let Event::Key(key) =
                event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Enter => {
                        // プレゼンターを取得してコントローラーに渡す
                        if let Some(presenter) =
                            self.presenter_registry.get_invoice_print_presenter(self.id)
                        {
                            controllers.invoice_print.print_invoice_with_presenter(
                                "mock-invoice-001".to_string(),
                                presenter,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
