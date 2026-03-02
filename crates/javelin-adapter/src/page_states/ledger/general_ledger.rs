// GeneralLedgerPage - 総勘定元帳画面
// 責務: 総勘定元帳の表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
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
    template: MasterListTemplate<GeneralLedgerItemViewModel>,
}

impl GeneralLedgerPageState {
    pub fn new() -> Self {
        let mut template = MasterListTemplate::new("総勘定元帳");

        // ダミーデータを設定
        let dummy_items = vec![
            GeneralLedgerItemViewModel {
                date: "2024-01-15".to_string(),
                voucher_number: "V-2024-001".to_string(),
                description: "売上計上".to_string(),
                debit: "1,000,000".to_string(),
                credit: "".to_string(),
                balance: "1,000,000".to_string(),
            },
            GeneralLedgerItemViewModel {
                date: "2024-01-20".to_string(),
                voucher_number: "V-2024-002".to_string(),
                description: "仕入計上".to_string(),
                debit: "".to_string(),
                credit: "500,000".to_string(),
                balance: "500,000".to_string(),
            },
            GeneralLedgerItemViewModel {
                date: "2024-01-25".to_string(),
                voucher_number: "V-2024-003".to_string(),
                description: "給与支払".to_string(),
                debit: "".to_string(),
                credit: "300,000".to_string(),
                balance: "200,000".to_string(),
            },
        ];

        template.set_data(dummy_items, 0, 0);

        Self { template }
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for GeneralLedgerPageState {
    fn route(&self) -> Route {
        Route::GeneralLedger
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
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
                    KeyCode::Char('j') | KeyCode::Down => {
                        // 次の項目を選択（将来実装）
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        // 前の項目を選択（将来実装）
                    }
                    KeyCode::Enter => {
                        // 詳細画面への遷移（将来実装）
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for GeneralLedgerPageState {
    fn default() -> Self {
        Self::new()
    }
}
