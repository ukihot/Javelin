// GeneralLedgerPage - 総勘定元帳画面
// 責務: 総勘定元帳の表示

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

/// 総勘定元帳項目ViewModel
#[derive(Debug, Clone)]
pub struct GeneralLedgerItemViewModel {
    pub date: String,
    pub voucher_number: String,
    pub description: String,
    pub debit: String,
    pub credit: String,
    pub balance: String,
}

impl MasterListItem for GeneralLedgerItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["日付", "伝票番号", "摘要", "借方", "貸方", "残高"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Min(20),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.voucher_number.clone(),
            self.description.clone(),
            self.debit.clone(),
            self.credit.clone(),
            self.balance.clone(),
        ]
    }
}

/// 総勘定元帳画面
pub struct GeneralLedgerPageState {
    page_id: Uuid,
    template: MasterListTemplate<GeneralLedgerItemViewModel>,
    presenter_registry: Arc<PresenterRegistry>,
    ledger_rx: tokio::sync::mpsc::UnboundedReceiver<crate::presenter::LedgerViewModel>,
}

impl GeneralLedgerPageState {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        let page_id = Uuid::new_v4();
        let template = MasterListTemplate::new("総勘定元帳");

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
                account_code: "0000".to_string(), // 全勘定科目
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
            let items: Vec<GeneralLedgerItemViewModel> = ledger_data
                .entries
                .into_iter()
                .map(|entry| GeneralLedgerItemViewModel {
                    date: entry.transaction_date,
                    voucher_number: entry.entry_number,
                    description: entry.description,
                    debit: if entry.debit_amount > 0.0 {
                        format!("{:.2}", entry.debit_amount)
                    } else {
                        "".to_string()
                    },
                    credit: if entry.credit_amount > 0.0 {
                        format!("{:.2}", entry.credit_amount)
                    } else {
                        "".to_string()
                    },
                    balance: format!("{:.2}", entry.balance),
                })
                .collect();

            self.template.set_data(items, 0, 0);
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl Drop for GeneralLedgerPageState {
    fn drop(&mut self) {
        // TODO: PresenterRegistryから登録解除
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

                match key.code {
                    KeyCode::Esc => {
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Enter => {
                        // 詳細画面への遷移
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
