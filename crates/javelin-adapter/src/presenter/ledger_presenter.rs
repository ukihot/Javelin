// LedgerPresenter実装
// 元帳・試算表の出力を整形してビューに渡す

use javelin_application::{
    dtos::{JournalEntryDetail, JournalEntryListResult},
    output_ports::{ClosingOutputPort, QueryOutputPort},
    query_service::{LedgerResult, TrialBalanceResult},
};
use tokio::sync::mpsc;

/// 元帳ViewModel
#[derive(Debug, Clone)]
pub struct LedgerViewModel {
    pub account_code: String,
    pub account_name: String,
    pub opening_balance: f64,
    pub entries: Vec<LedgerEntryViewModel>,
    pub closing_balance: f64,
    pub total_debit: f64,
    pub total_credit: f64,
}

/// 元帳明細ViewModel
#[derive(Debug, Clone)]
pub struct LedgerEntryViewModel {
    pub transaction_date: String,
    pub entry_number: String,
    pub entry_id: String,
    pub description: String,
    pub debit_amount: f64,
    pub credit_amount: f64,
    pub balance: f64,
}

/// 試算表ViewModel
#[derive(Debug, Clone)]
pub struct TrialBalanceViewModel {
    pub period_year: u32,
    pub period_month: u8,
    pub entries: Vec<TrialBalanceEntryViewModel>,
    pub total_debit: f64,
    pub total_credit: f64,
}

/// 試算表明細ViewModel
#[derive(Debug, Clone)]
pub struct TrialBalanceEntryViewModel {
    pub account_code: String,
    pub account_name: String,
    pub opening_balance: f64,
    pub debit_amount: f64,
    pub credit_amount: f64,
    pub closing_balance: f64,
}

/// 元帳Presenter
pub struct LedgerPresenter {
    ledger_sender: mpsc::UnboundedSender<LedgerViewModel>,
    trial_balance_sender: mpsc::UnboundedSender<TrialBalanceViewModel>,
}

impl LedgerPresenter {
    pub fn new(
        ledger_sender: mpsc::UnboundedSender<LedgerViewModel>,
        trial_balance_sender: mpsc::UnboundedSender<TrialBalanceViewModel>,
    ) -> Self {
        Self { ledger_sender, trial_balance_sender }
    }

    /// チャネルを作成
    pub fn create_channels() -> (
        mpsc::UnboundedSender<LedgerViewModel>,
        mpsc::UnboundedReceiver<LedgerViewModel>,
        mpsc::UnboundedSender<TrialBalanceViewModel>,
        mpsc::UnboundedReceiver<TrialBalanceViewModel>,
    ) {
        let (ledger_tx, ledger_rx) = mpsc::unbounded_channel();
        let (trial_balance_tx, trial_balance_rx) = mpsc::unbounded_channel();
        (ledger_tx, ledger_rx, trial_balance_tx, trial_balance_rx)
    }
}

#[allow(async_fn_in_trait)]
impl QueryOutputPort for LedgerPresenter {
    async fn present_journal_entry_list(&self, _result: JournalEntryListResult) {
        // Not implemented for LedgerPresenter
    }

    async fn present_journal_entry_detail(&self, _result: JournalEntryDetail) {
        // Not implemented for LedgerPresenter
    }

    async fn present_ledger(&self, result: LedgerResult) {
        let entries = result
            .entries
            .into_iter()
            .map(|entry| LedgerEntryViewModel {
                transaction_date: entry.transaction_date,
                entry_number: entry.entry_number,
                entry_id: entry.entry_id,
                description: entry.description,
                debit_amount: entry.debit_amount.parse::<f64>().unwrap_or(0.0),
                credit_amount: entry.credit_amount.parse::<f64>().unwrap_or(0.0),
                balance: entry.balance.parse::<f64>().unwrap_or(0.0),
            })
            .collect();

        let view_model = LedgerViewModel {
            account_code: result.account_code,
            account_name: result.account_name,
            opening_balance: result.opening_balance.parse::<f64>().unwrap_or(0.0),
            entries,
            closing_balance: result.closing_balance.parse::<f64>().unwrap_or(0.0),
            total_debit: result.total_debit.parse::<f64>().unwrap_or(0.0),
            total_credit: result.total_credit.parse::<f64>().unwrap_or(0.0),
        };

        let _ = self.ledger_sender.send(view_model);
    }

    async fn present_trial_balance(&self, result: TrialBalanceResult) {
        let entries = result
            .entries
            .into_iter()
            .map(|entry| TrialBalanceEntryViewModel {
                account_code: entry.account_code,
                account_name: entry.account_name,
                opening_balance: entry.opening_balance.parse::<f64>().unwrap_or(0.0),
                debit_amount: entry.debit_amount.parse::<f64>().unwrap_or(0.0),
                credit_amount: entry.credit_amount.parse::<f64>().unwrap_or(0.0),
                closing_balance: entry.closing_balance.parse::<f64>().unwrap_or(0.0),
            })
            .collect();

        let view_model = TrialBalanceViewModel {
            period_year: result.period_year,
            period_month: result.period_month,
            entries,
            total_debit: result.total_debit.parse::<f64>().unwrap_or(0.0),
            total_credit: result.total_credit.parse::<f64>().unwrap_or(0.0),
        };

        let _ = self.trial_balance_sender.send(view_model);
    }
}

#[allow(async_fn_in_trait)]
impl ClosingOutputPort for LedgerPresenter {
    async fn notify_judgment_log(
        &self,
        judgment_type: String,
        accounting_standard: String,
        model_used: String,
        assumptions: Vec<String>,
        sensitivity_analysis: Vec<String>,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) {
        // 判断ログをログに出力（実装される際にビューへ送信）
        eprintln!(
            "[{}] {} - {} ({})\n  Accounting Standard: {}\n  Model: {}\n  Assumptions: {:?}\n  Sensitivity: {:?}",
            timestamp,
            judgment_type,
            accounting_standard,
            model_used,
            accounting_standard,
            model_used,
            assumptions,
            sensitivity_analysis
        );
    }

    async fn notify_progress(&self, message: String) {
        eprintln!("[Progress] {}", message);
    }

    async fn notify_error(&self, error_message: String) {
        eprintln!("[Error] {}", error_message);
    }
}
