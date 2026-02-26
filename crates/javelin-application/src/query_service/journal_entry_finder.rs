// JournalEntryFinder - 仕訳検索・照会クエリサービス
// 責務: 仕訳の検索、一覧取得、詳細取得

use crate::{
    dtos::{GetJournalEntryQuery, ListJournalEntriesQuery},
    error::ApplicationResult,
};

/// 既存伝票検索結果（仕訳行為区分で使用）
#[derive(Debug, Clone)]
pub struct JournalEntrySearchResult {
    pub entry_id: String,
    pub entry_number: Option<String>,
    pub transaction_date: String,
    pub total_debit: i64,
    pub total_credit: i64,
    pub status: String,
}

/// 仕訳検索・照会クエリサービス
///
/// 以下の機能を提供：
/// 1. 既存伝票検索（仕訳行為区分で参照元を探す）
/// 2. 仕訳一覧取得（画面表示用）
/// 3. 仕訳詳細取得（画面表示用）
#[allow(async_fn_in_trait)]
pub trait JournalEntryFinderService: Send + Sync {
    // === 既存伝票検索（仕訳行為区分用） ===

    /// 伝票番号で検索
    async fn find_by_entry_number(
        &self,
        entry_number: &str,
    ) -> ApplicationResult<Option<JournalEntrySearchResult>>;

    /// 証憑番号で検索
    async fn find_by_voucher_number(
        &self,
        voucher_number: &str,
    ) -> ApplicationResult<Vec<JournalEntrySearchResult>>;

    /// 取引日範囲で検索
    async fn find_by_date_range(
        &self,
        from_date: &str,
        to_date: &str,
    ) -> ApplicationResult<Vec<JournalEntrySearchResult>>;

    // === 仕訳一覧・詳細取得（画面表示用） ===

    /// 仕訳一覧を取得してOutput Portへ送信
    async fn list_journal_entries(&self, query: ListJournalEntriesQuery) -> ApplicationResult<()>;

    /// 仕訳詳細を取得してOutput Portへ送信
    async fn get_journal_entry(&self, query: GetJournalEntryQuery) -> ApplicationResult<()>;
}
