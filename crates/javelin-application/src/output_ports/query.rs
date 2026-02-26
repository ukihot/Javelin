use crate::{
    dtos::response::{JournalEntryDetail, JournalEntryListResult},
    query_service::{LedgerResult, TrialBalanceResult},
};

/// QueryOutputPort - クエリ結果の出力
#[allow(async_fn_in_trait)]
pub trait QueryOutputPort: Send + Sync {
    /// 仕訳一覧結果を出力
    async fn present_journal_entry_list(&self, result: JournalEntryListResult);

    /// 仕訳詳細結果を出力
    async fn present_journal_entry_detail(&self, result: JournalEntryDetail);

    /// 元帳結果を出力
    async fn present_ledger(&self, result: LedgerResult);

    /// 試算表結果を出力
    async fn present_trial_balance(&self, result: TrialBalanceResult);
}
