// Presenter - 出力整形
// 責務: OutputPort実装、Viewモデル生成
// 禁止: 業務判断

pub mod account_master_presenter;
pub mod application_settings_presenter;
pub mod batch_history_presenter;
pub mod company_master_presenter;
pub mod journal_entry_presenter;
pub mod ledger_presenter;
pub mod search_presenter;
pub mod subsidiary_account_master_presenter;

pub use account_master_presenter::{
    AccountMasterItemViewModel, AccountMasterPresenter, AccountMasterViewModel,
};
pub use application_settings_presenter::{
    ApplicationSettingsPresenter, ApplicationSettingsViewModel,
};
pub use batch_history_presenter::{
    BatchHistoryChannels, BatchHistoryPresenter, BatchHistoryViewModel,
};
pub use company_master_presenter::{
    CompanyMasterItemViewModel, CompanyMasterPresenter, CompanyMasterViewModel,
};
use javelin_application::output_ports::{EventNotification, EventOutputPort};
pub use journal_entry_presenter::{
    JournalEntryDetailViewModel, JournalEntryLineViewModel, JournalEntryListItemViewModel,
    JournalEntryListViewModel, JournalEntryPresenter, JournalEntryViewModel,
};
pub use ledger_presenter::{
    LedgerEntryViewModel, LedgerPresenter, LedgerViewModel, TrialBalanceEntryViewModel,
    TrialBalanceViewModel,
};
pub use search_presenter::{
    JournalEntryItemViewModel, JournalEntryLineItemViewModel, SearchChannels, SearchPresenter,
    SearchResultViewModel,
};
pub use subsidiary_account_master_presenter::{
    SubsidiaryAccountMasterItemViewModel, SubsidiaryAccountMasterPresenter,
    SubsidiaryAccountMasterViewModel,
};
use tokio::sync::mpsc;

/// イベント通知用のチャネル
pub type EventSender = mpsc::UnboundedSender<EventNotification>;
pub type EventReceiver = mpsc::UnboundedReceiver<EventNotification>;

/// Presenter - イベント通知を管理
pub struct Presenter {
    event_sender: EventSender,
}

impl Presenter {
    pub fn new(event_sender: EventSender) -> Self {
        Self { event_sender }
    }

    /// イベント通知チャネルを作成
    pub fn create_channel() -> (EventSender, EventReceiver) {
        mpsc::unbounded_channel()
    }
}

impl EventOutputPort for Presenter {
    fn notify_event(
        &self,
        event: EventNotification,
    ) -> impl std::future::Future<Output = ()> + Send {
        let sender = self.event_sender.clone();
        async move {
            // イベントをチャネル経由で送信（失敗しても無視）
            let _ = sender.send(event);
        }
    }
}

impl Default for Presenter {
    fn default() -> Self {
        let (sender, _) = Self::create_channel();
        Self::new(sender)
    }
}
