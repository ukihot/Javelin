// LedgerQueryServiceImpl - 元帳照会サービス実装（Infrastructure層）
// LedgerProjectionから元帳データを取得

use std::sync::Arc;

use javelin_application::{
    error::{ApplicationError, ApplicationResult},
    query_service::ledger_query_service::{
        GetLedgerQuery, GetTrialBalanceQuery, LedgerEntry, LedgerQueryService, LedgerResult,
        TrialBalanceResult,
    },
};

use crate::{
    EventStore,
    projection_trait::Apply,
    queries::ledger_projection::{LedgerEntryReadModel, LedgerProjection},
};

/// LedgerQueryService実装
///
/// EventStoreからイベントを取得してLedgerProjectionを構築し、
/// 元帳データを返す。
pub struct LedgerQueryServiceImpl {
    event_store: Arc<EventStore>,
}

impl LedgerQueryServiceImpl {
    /// 新しいインスタンスを作成
    pub fn new(event_store: Arc<EventStore>) -> Self {
        Self { event_store }
    }

    /// イベントストリームからLedgerProjectionを構築
    async fn build_ledger_projection(&self) -> ApplicationResult<LedgerProjection> {
        use javelin_domain::financial_close::journal_entry::events::JournalEntryEvent;

        let mut projection = LedgerProjection::new();

        // 全イベントを取得（EventStoreから直接）
        let events = self
            .event_store
            .get_all_events(0)
            .await
            .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

        // イベントを適用
        for stored_event in events.iter() {
            // JournalEntryEventにデシリアライズ
            if let Ok(event) = serde_json::from_slice::<JournalEntryEvent>(&stored_event.payload) {
                projection
                    .apply(event)
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;
            }
        }

        Ok(projection)
    }
}

impl LedgerQueryService for LedgerQueryServiceImpl {
    async fn get_ledger(&self, query: GetLedgerQuery) -> ApplicationResult<LedgerResult> {
        // LedgerProjectionを構築
        let projection = self.build_ledger_projection().await?;

        // 元帳エントリを取得
        let all_entries = projection.entries();

        // 勘定科目でフィルタリング
        let mut filtered_entries: Vec<&LedgerEntryReadModel> = all_entries
            .iter()
            .filter(|entry| entry.account_code == query.account_code)
            .collect();

        // 日付範囲でフィルタリング
        if let Some(ref from_date) = query.from_date {
            filtered_entries.retain(|entry| entry.transaction_date >= *from_date);
        }
        if let Some(ref to_date) = query.to_date {
            filtered_entries.retain(|entry| entry.transaction_date <= *to_date);
        }

        // 取引日付でソート
        filtered_entries.sort_by(|a, b| a.transaction_date.cmp(&b.transaction_date));

        // ページネーション適用
        let offset = query.offset.unwrap_or(0) as usize;
        let limit = query.limit.unwrap_or(100) as usize;
        let paginated_entries: Vec<&LedgerEntryReadModel> =
            filtered_entries.iter().skip(offset).take(limit).copied().collect();

        // 期首残高を計算（フィルタ前の最初のエントリの残高 - その借方貸方差額）
        let opening_balance = if let Some(first_entry) = filtered_entries.first() {
            first_entry.balance - (first_entry.debit_amount - first_entry.credit_amount)
        } else {
            0.0
        };

        // 借方合計、貸方合計を計算
        let mut total_debit = 0.0;
        let mut total_credit = 0.0;
        for entry in &paginated_entries {
            total_debit += entry.debit_amount;
            total_credit += entry.credit_amount;
        }

        // 期末残高（最後のエントリの残高）
        let closing_balance =
            paginated_entries.last().map(|entry| entry.balance).unwrap_or(opening_balance);

        // LedgerEntryに変換
        let entries: Vec<LedgerEntry> = paginated_entries
            .into_iter()
            .map(|entry| LedgerEntry {
                transaction_date: entry.transaction_date.clone(),
                entry_number: entry.entry_number.clone(),
                entry_id: entry.entry_number.clone(), // entry_idがないのでentry_numberを使用
                description: entry.description.clone(),
                debit_amount: entry.debit_amount,
                credit_amount: entry.credit_amount,
                balance: entry.balance,
            })
            .collect();

        Ok(LedgerResult {
            account_code: query.account_code.clone(),
            account_name: format!("勘定科目{}", query.account_code), // TODO: マスタデータから取得
            opening_balance,
            entries,
            closing_balance,
            total_debit,
            total_credit,
        })
    }

    async fn get_trial_balance(
        &self,
        query: GetTrialBalanceQuery,
    ) -> ApplicationResult<TrialBalanceResult> {
        use std::collections::HashMap;

        use javelin_application::query_service::TrialBalanceEntry;

        // LedgerProjectionを構築
        let projection = self.build_ledger_projection().await?;

        // 元帳エントリを取得
        let all_entries = projection.entries();

        // 期間でフィルタリング（YYYY-MM形式）
        let period_str = format!("{:04}-{:02}", query.period_year, query.period_month);
        let filtered_entries: Vec<&LedgerEntryReadModel> = all_entries
            .iter()
            .filter(|entry| entry.transaction_date.starts_with(&period_str))
            .collect();

        // 勘定科目ごとに集計
        let mut account_map: HashMap<String, (f64, f64, f64, f64)> = HashMap::new();
        for entry in filtered_entries {
            let (_opening, debit, credit, closing) =
                account_map.entry(entry.account_code.clone()).or_insert((0.0, 0.0, 0.0, 0.0));
            *debit += entry.debit_amount;
            *credit += entry.credit_amount;
            *closing = entry.balance; // 最後のエントリの残高が期末残高
        }

        // 期首残高を計算（期末残高 - 借方 + 貸方）
        for (_account_code, (opening, debit, credit, closing)) in account_map.iter_mut() {
            *opening = *closing - *debit + *credit;
        }

        // TrialBalanceEntryに変換
        let mut entries: Vec<TrialBalanceEntry> = account_map
            .into_iter()
            .map(|(account_code, (opening_balance, debit_amount, credit_amount, closing_balance))| {
                TrialBalanceEntry {
                    account_code: account_code.clone(),
                    account_name: format!("勘定科目{}", account_code), // TODO: マスタデータから取得
                    opening_balance,
                    debit_amount,
                    credit_amount,
                    closing_balance,
                }
            })
            .collect();

        // 勘定科目コードでソート
        entries.sort_by(|a, b| a.account_code.cmp(&b.account_code));

        // 借貸合計を計算
        let mut total_debit = 0.0;
        let mut total_credit = 0.0;
        for entry in &entries {
            total_debit += entry.debit_amount;
            total_credit += entry.credit_amount;
        }

        Ok(TrialBalanceResult {
            period_year: query.period_year,
            period_month: query.period_month,
            entries,
            total_debit,
            total_credit,
        })
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_get_ledger() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let service = LedgerQueryServiceImpl::new(event_store);

        let query = GetLedgerQuery {
            account_code: "1001".to_string(),
            from_date: None,
            to_date: None,
            limit: None,
            offset: None,
        };

        let result = service.get_ledger(query).await.unwrap();
        assert_eq!(result.account_code, "1001");
        assert_eq!(result.entries.len(), 0);
    }

    #[tokio::test]
    async fn test_get_trial_balance() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let service = LedgerQueryServiceImpl::new(event_store);

        let query = GetTrialBalanceQuery { period_year: 2024, period_month: 1 };

        let result = service.get_trial_balance(query).await.unwrap();
        assert_eq!(result.period_year, 2024);
        assert_eq!(result.period_month, 1);
        assert_eq!(result.entries.len(), 0);
    }
}
