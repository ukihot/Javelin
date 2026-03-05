// OutputPort - 出力抽象
// 責務: Presenter連携

pub mod adjust_accounts;
pub mod closing;
pub mod common;
pub mod invoice_print;
pub mod invoice_printer;
pub mod journal_entry;
pub mod master_data;
pub mod query;
pub mod search;

pub use adjust_accounts::AdjustAccountsOutputPort;
pub use closing::ClosingOutputPort;
pub use common::OutputPort;
pub use invoice_print::InvoicePrintOutputPort;
pub use invoice_printer::InvoicePrinter;
pub use journal_entry::JournalEntryOutputPort;
pub use master_data::{
    AccountMasterOutputPort, ApplicationSettingsOutputPort, CompanyMasterOutputPort,
    SubsidiaryAccountMasterOutputPort,
};
pub use query::QueryOutputPort;
pub use search::SearchOutputPort;

/// イベント通知情報（EventOutputPort用の汎用型）
#[derive(Clone, Debug)]
pub struct EventNotification {
    pub user: String,
    pub location: String,
    pub action: String,
    pub success: bool,
}

impl EventNotification {
    pub fn success(
        user: impl Into<String>,
        location: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            user: user.into(),
            location: location.into(),
            action: action.into(),
            success: true,
        }
    }

    pub fn failure(
        user: impl Into<String>,
        location: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            user: user.into(),
            location: location.into(),
            action: action.into(),
            success: false,
        }
    }
}

/// EventOutputPort - イベント通知専用（仕訳処理等で使用）
pub trait EventOutputPort: Send + Sync {
    /// イベントをプレゼンターに通知（非同期）
    fn notify_event(
        &self,
        event: EventNotification,
    ) -> impl std::future::Future<Output = ()> + Send;
}
