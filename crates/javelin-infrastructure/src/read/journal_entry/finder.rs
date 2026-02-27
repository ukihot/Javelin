// JournalEntryFinderService具象実装 - Infrastructure層
// Application層のJournalEntryFinderServiceトレイトを実装

use std::sync::Arc;

use javelin_application::{
    dtos::{
        GetJournalEntryQuery, JournalEntryDetail, JournalEntryLineDetail, JournalEntryListItem,
        JournalEntryListResult, ListJournalEntriesQuery,
    },
    error::{ApplicationError, ApplicationResult},
    output_ports::QueryOutputPort,
    query_service::{JournalEntryFinderService, JournalEntrySearchResult},
};

use crate::read::infrastructure::db::ProjectionDb;

/// 仕訳検索・照会サービス具象実装
///
/// ProjectionDBから仕訳データを取得し、Output Portを通じて結果を送信する。
pub struct JournalEntryFinderImpl<O: QueryOutputPort> {
    projection_db: Arc<ProjectionDb>,
    output_port: Arc<O>,
}

impl<O: QueryOutputPort> JournalEntryFinderImpl<O> {
    /// 新しいJournalEntryFinderImplを作成
    pub fn new(projection_db: Arc<ProjectionDb>, output_port: Arc<O>) -> Self {
        Self { projection_db, output_port }
    }
}

impl<O: QueryOutputPort> JournalEntryFinderService for JournalEntryFinderImpl<O> {
    // === 既存伝票検索（仕訳行為区分用） ===

    async fn find_by_entry_number(
        &self,
        entry_number: &str,
    ) -> ApplicationResult<Option<JournalEntrySearchResult>> {
        // ProjectionDBから伝票番号で検索
        for i in 0..1000 {
            let entry_id = format!("entry-{}", i);
            let key = format!("journal_entry:{}", entry_id);

            if let Some(data) = self
                .projection_db
                .get_projection(&key)
                .await
                .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
            {
                let stored_entry: StoredJournalEntry = serde_json::from_slice(&data)
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                if let Some(ref num) = stored_entry.entry_number
                    && num == entry_number
                {
                    return Ok(Some(JournalEntrySearchResult {
                        entry_id: stored_entry.entry_id,
                        entry_number: stored_entry.entry_number,
                        transaction_date: stored_entry.transaction_date,
                        total_debit: stored_entry.total_debit as i64,
                        total_credit: stored_entry.total_credit as i64,
                        status: stored_entry.status,
                    }));
                }
            }
        }
        Ok(None)
    }

    async fn find_by_voucher_number(
        &self,
        voucher_number: &str,
    ) -> ApplicationResult<Vec<JournalEntrySearchResult>> {
        // モダンプラクティス: 初期キャパシティを確保
        let mut results = Vec::with_capacity(10);

        for i in 0..1000 {
            let entry_id = format!("entry-{}", i);
            let key = format!("journal_entry:{}", entry_id);

            if let Some(data) = self
                .projection_db
                .get_projection(&key)
                .await
                .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
            {
                let stored_entry: StoredJournalEntry = serde_json::from_slice(&data)
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                if stored_entry.voucher_number == voucher_number {
                    results.push(JournalEntrySearchResult {
                        entry_id: stored_entry.entry_id,
                        entry_number: stored_entry.entry_number,
                        transaction_date: stored_entry.transaction_date,
                        total_debit: stored_entry.total_debit as i64,
                        total_credit: stored_entry.total_credit as i64,
                        status: stored_entry.status,
                    });
                }
            }
        }

        Ok(results)
    }

    async fn find_by_date_range(
        &self,
        from_date: &str,
        to_date: &str,
    ) -> ApplicationResult<Vec<JournalEntrySearchResult>> {
        // モダンプラクティス: 初期キャパシティを確保
        let mut results = Vec::with_capacity(50);

        for i in 0..1000 {
            let entry_id = format!("entry-{}", i);
            let key = format!("journal_entry:{}", entry_id);

            if let Some(data) = self
                .projection_db
                .get_projection(&key)
                .await
                .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
            {
                let stored_entry: StoredJournalEntry = serde_json::from_slice(&data)
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                if stored_entry.transaction_date.as_str() >= from_date
                    && stored_entry.transaction_date.as_str() <= to_date
                {
                    results.push(JournalEntrySearchResult {
                        entry_id: stored_entry.entry_id,
                        entry_number: stored_entry.entry_number,
                        transaction_date: stored_entry.transaction_date,
                        total_debit: stored_entry.total_debit as i64,
                        total_credit: stored_entry.total_credit as i64,
                        status: stored_entry.status,
                    });
                }
            }
        }

        Ok(results)
    }

    // === 仕訳一覧・詳細取得（画面表示用） ===

    async fn list_journal_entries(&self, query: ListJournalEntriesQuery) -> ApplicationResult<()> {
        // モダンプラクティス: 初期キャパシティを確保
        let mut all_entries: Vec<JournalEntryListItem> = Vec::with_capacity(100);

        for i in 0..1000 {
            let entry_id = format!("entry-{}", i);
            let key = format!("journal_entry:{}", entry_id);

            if let Some(data) = self
                .projection_db
                .get_projection(&key)
                .await
                .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
            {
                let stored_entry: StoredJournalEntry = serde_json::from_slice(&data)
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                // フィルタリング: ステータス
                if let Some(ref status_filter) = query.status
                    && &stored_entry.status != status_filter
                {
                    continue;
                }

                // フィルタリング: 日付範囲
                if let Some(ref from_date) = query.from_date
                    && stored_entry.transaction_date < *from_date
                {
                    continue;
                }

                if let Some(ref to_date) = query.to_date
                    && stored_entry.transaction_date > *to_date
                {
                    continue;
                }

                all_entries.push(JournalEntryListItem {
                    entry_id: stored_entry.entry_id,
                    entry_number: stored_entry.entry_number,
                    status: stored_entry.status,
                    transaction_date: stored_entry.transaction_date,
                    voucher_number: stored_entry.voucher_number,
                    total_debit: stored_entry.total_debit,
                    total_credit: stored_entry.total_credit,
                    created_by: stored_entry.created_by,
                    created_at: stored_entry.created_at,
                });
            }
        }

        // ソート（日付順）
        all_entries.sort_by(|a, b| b.transaction_date.cmp(&a.transaction_date));

        let total_count = all_entries.len() as u32;

        // ページネーション適用
        let offset = query.offset.unwrap_or(0) as usize;
        let limit = query.limit.unwrap_or(100) as usize;

        let items: Vec<JournalEntryListItem> =
            all_entries.into_iter().skip(offset).take(limit).collect();

        let result = JournalEntryListResult { items, total_count };

        self.output_port.present_journal_entry_list(result).await;
        Ok(())
    }

    async fn get_journal_entry(&self, query: GetJournalEntryQuery) -> ApplicationResult<()> {
        let key = format!("journal_entry:{}", query.entry_id);

        let entry_data = self
            .projection_db
            .get_projection(&key)
            .await
            .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

        if let Some(data) = entry_data {
            let stored_entry: StoredJournalEntry = serde_json::from_slice(&data)
                .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

            let lines: Vec<JournalEntryLineDetail> = stored_entry
                .lines
                .into_iter()
                .map(|line| JournalEntryLineDetail {
                    line_number: line.line_number,
                    side: line.side,
                    account_code: line.account_code,
                    account_name: line.account_name,
                    sub_account_code: line.sub_account_code,
                    department_code: line.department_code,
                    amount: line.amount,
                    currency: line.currency,
                    tax_type: line.tax_type,
                    tax_amount: line.tax_amount,
                })
                .collect();

            let result = JournalEntryDetail {
                entry_id: stored_entry.entry_id,
                entry_number: stored_entry.entry_number,
                status: stored_entry.status,
                transaction_date: stored_entry.transaction_date,
                voucher_number: stored_entry.voucher_number,
                lines,
                created_by: stored_entry.created_by,
                created_at: stored_entry.created_at,
                updated_by: stored_entry.updated_by,
                updated_at: stored_entry.updated_at,
                approved_by: stored_entry.approved_by,
                approved_at: stored_entry.approved_at,
            };

            self.output_port.present_journal_entry_detail(result).await;
        } else {
            let result = JournalEntryDetail {
                entry_id: query.entry_id,
                entry_number: None,
                status: "NotFound".to_string(),
                transaction_date: "".to_string(),
                voucher_number: "".to_string(),
                lines: vec![],
                created_by: "".to_string(),
                created_at: "".to_string(),
                updated_by: None,
                updated_at: None,
                approved_by: None,
                approved_at: None,
            };

            self.output_port.present_journal_entry_detail(result).await;
        }

        Ok(())
    }
}

/// ProjectionDBに保存される仕訳エントリデータ構造
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct StoredJournalEntry {
    entry_id: String,
    entry_number: Option<String>,
    status: String,
    transaction_date: String,
    voucher_number: String,
    total_debit: f64,
    total_credit: f64,
    created_by: String,
    created_at: String,
    updated_by: Option<String>,
    updated_at: Option<String>,
    approved_by: Option<String>,
    approved_at: Option<String>,
    lines: Vec<StoredJournalEntryLine>,
}

/// ProjectionDBに保存される仕訳明細データ構造
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct StoredJournalEntryLine {
    line_number: u32,
    side: String,
    account_code: String,
    account_name: String,
    sub_account_code: Option<String>,
    department_code: Option<String>,
    amount: f64,
    currency: String,
    tax_type: String,
    tax_amount: f64,
}
