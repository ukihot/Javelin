// ApDetailPageState - 買掛金明細画面
// 責務: 買掛金の詳細明細表示

use std::sync::Arc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::LedgerPresenter,
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// 買掛金明細項目ViewModel
#[derive(Debug, Clone)]
pub struct ApDetailItemViewModel {
    pub date: String,
    pub vendor: String,
    pub invoice_number: String,
    pub description: String,
    pub amount: String,
    pub payment_status: String,
    pub due_date: String,
}

impl MasterListItem for ApDetailItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["日付", "取引先", "請求番号", "摘要", "金額", "支払状況", "期日"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Min(20),
            Constraint::Length(15),
            Constraint::Length(12),
            Constraint::Length(12),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.vendor.clone(),
            self.invoice_number.clone(),
            self.description.clone(),
            self.amount.clone(),
            self.payment_status.clone(),
            self.due_date.clone(),
        ]
    }
}

/// 買掛金明細画面
pub struct ApDetailPageState {
    page_id: Uuid,
    template: MasterListTemplate<ApDetailItemViewModel>,
    presenter_registry: Arc<PresenterRegistry>,
    ledger_rx: tokio::sync::mpsc::UnboundedReceiver<crate::presenter::LedgerViewModel>,
}

impl ApDetailPageState {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        let page_id = Uuid::new_v4();
        let template = MasterListTemplate::new("買掛金明細");

        let (ledger_tx, ledger_rx, trial_balance_tx, _trial_balance_rx) =
            LedgerPresenter::create_channels();

        let _presenter = LedgerPresenter::new(ledger_tx, trial_balance_tx);

        Self { page_id, template, presenter_registry, ledger_rx }
    }

    fn load_data(&self, controllers: &Controllers) {
        let ledger_controller = controllers.ledger.clone();

        tokio::spawn(async move {
            use javelin_application::query_service::GetLedgerQuery;

            let query = GetLedgerQuery {
                account_code: "2120".to_string(),
                from_date: None,
                to_date: None,
                limit: Some(100),
                offset: None,
            };

            let _ = ledger_controller.get_ledger(query).await;
        });
    }

    fn poll_ledger_data(&mut self) {
        while let Ok(ledger_data) = self.ledger_rx.try_recv() {
            let items: Vec<ApDetailItemViewModel> = ledger_data
                .entries
                .into_iter()
                .map(|entry| ApDetailItemViewModel {
                    date: entry.transaction_date.clone(),
                    vendor: "".to_string(),
                    invoice_number: entry.entry_number,
                    description: entry.description,
                    amount: format!("{:.2}", entry.debit_amount + entry.credit_amount),
                    payment_status: "未払".to_string(),
                    due_date: "".to_string(),
                })
                .collect();

            self.template.set_data(items, 0, 0);
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl Drop for ApDetailPageState {
    fn drop(&mut self) {
        self.presenter_registry.unregister_ledger_presenter(self.page_id);
    }
}

impl PageState for ApDetailPageState {
    fn route(&self) -> Route {
        Route::ApDetail
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        self.load_data(controllers);

        loop {
            self.poll_ledger_data();

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
