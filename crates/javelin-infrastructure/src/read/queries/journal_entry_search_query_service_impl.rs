// JournalEntrySearchQueryServiceImpl - 仕訳検索サービス実装（Infrastructure層）
// JournalEntrySearchProjectionから仕訳データを検索

use std::sync::Arc;

use javelin_application::{
    dtos::{
        request::SearchCriteriaDto,
        response::{JournalEntryItemDto, JournalEntryLineItemDto, JournalEntrySearchResultDto},
    },
    error::{ApplicationError, ApplicationResult},
    query_service::JournalEntrySearchQueryService,
};

use crate::{
    EventStore,
    projection_trait::Apply,
    queries::{
        journal_entry_search_projection::JournalEntrySearchProjection,
        journal_entry_search_read_model::JournalEntrySearchReadModel,
    },
};

/// JournalEntrySearchQueryService実装
///
/// EventStoreからイベントを取得してJournalEntrySearchProjectionを構築し、
/// 検索条件に基づいて仕訳データを返す。
pub struct JournalEntrySearchQueryServiceImpl {
    event_store: Arc<EventStore>,
}

impl JournalEntrySearchQueryServiceImpl {
    /// 新しいインスタンスを作成
    pub fn new(event_store: Arc<EventStore>) -> Self {
        Self { event_store }
    }

    /// イベントストリームからJournalEntrySearchProjectionを構築
    async fn build_search_projection(&self) -> ApplicationResult<JournalEntrySearchProjection> {
        use javelin_domain::financial_close::journal_entry::events::JournalEntryEvent;

        let mut projection = JournalEntrySearchProjection::new();

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

    /// 日付範囲でフィルタリング
    fn filter_by_date_range(
        &self,
        entries: Vec<JournalEntrySearchReadModel>,
        from_date: Option<String>,
        to_date: Option<String>,
    ) -> Vec<JournalEntrySearchReadModel> {
        entries
            .into_iter()
            .filter(|entry| {
                let date = &entry.transaction_date;
                let from_ok = from_date.as_ref().map(|f| date >= f).unwrap_or(true);
                let to_ok = to_date.as_ref().map(|t| date <= t).unwrap_or(true);
                from_ok && to_ok
            })
            .collect()
    }

    /// 摘要でフィルタリング（部分一致、大文字小文字非区別）
    fn filter_by_description(
        &self,
        entries: Vec<JournalEntrySearchReadModel>,
        description: String,
    ) -> Vec<JournalEntrySearchReadModel> {
        entries
            .into_iter()
            .filter(|entry| entry.contains_description(&description))
            .collect()
    }

    /// 勘定科目でフィルタリング
    fn filter_by_account(
        &self,
        entries: Vec<JournalEntrySearchReadModel>,
        account_code: String,
    ) -> Vec<JournalEntrySearchReadModel> {
        entries
            .into_iter()
            .filter(|entry| entry.contains_account(&account_code))
            .collect()
    }

    /// 借方貸方区分でフィルタリング
    fn filter_by_debit_credit(
        &self,
        entries: Vec<JournalEntrySearchReadModel>,
        debit_credit: String,
    ) -> Vec<JournalEntrySearchReadModel> {
        entries.into_iter().filter(|entry| entry.contains_side(&debit_credit)).collect()
    }

    /// 金額範囲でフィルタリング
    fn filter_by_amount_range(
        &self,
        entries: Vec<JournalEntrySearchReadModel>,
        min_amount: Option<f64>,
        max_amount: Option<f64>,
    ) -> Vec<JournalEntrySearchReadModel> {
        entries
            .into_iter()
            .filter(|entry| entry.contains_amount_in_range(min_amount, max_amount))
            .collect()
    }
}

impl JournalEntrySearchQueryService for JournalEntrySearchQueryServiceImpl {
    async fn search(
        &self,
        criteria: SearchCriteriaDto,
    ) -> ApplicationResult<JournalEntrySearchResultDto> {
        // JournalEntrySearchProjectionを構築
        let projection = self.build_search_projection().await?;

        // 全エントリーを取得
        let mut entries: Vec<JournalEntrySearchReadModel> = projection.entries().to_vec();

        // 日付範囲でフィルタリング
        if criteria.from_date.is_some() || criteria.to_date.is_some() {
            entries = self.filter_by_date_range(
                entries,
                criteria.from_date.clone(),
                criteria.to_date.clone(),
            );
        }

        // 摘要でフィルタリング
        if let Some(description) = criteria.description.clone() {
            entries = self.filter_by_description(entries, description);
        }

        // 勘定科目でフィルタリング
        if let Some(account_code) = criteria.account_code.clone() {
            entries = self.filter_by_account(entries, account_code);
        }

        // 借方貸方区分でフィルタリング
        if let Some(debit_credit) = criteria.debit_credit.clone() {
            entries = self.filter_by_debit_credit(entries, debit_credit);
        }

        // 金額範囲でフィルタリング
        if criteria.min_amount.is_some() || criteria.max_amount.is_some() {
            entries =
                self.filter_by_amount_range(entries, criteria.min_amount, criteria.max_amount);
        }

        // 取引日付降順でソート
        entries.sort_by(|a, b| b.transaction_date.cmp(&a.transaction_date));

        // 総件数を保存
        let total_count = entries.len() as u32;

        // ページネーション適用
        let offset = criteria.offset.unwrap_or(0) as usize;
        let limit = criteria.limit.unwrap_or(100) as usize;
        let paginated_entries: Vec<JournalEntrySearchReadModel> =
            entries.into_iter().skip(offset).take(limit).collect();

        // DTOに変換
        let entry_dtos: Vec<JournalEntryItemDto> = paginated_entries
            .into_iter()
            .map(|entry| {
                let line_dtos: Vec<JournalEntryLineItemDto> = entry
                    .lines
                    .into_iter()
                    .map(|line| JournalEntryLineItemDto {
                        line_number: line.line_number,
                        side: line.side,
                        account_code: line.account_code,
                        account_name: line.account_name,
                        amount: line.amount,
                        description: line.description,
                    })
                    .collect();

                JournalEntryItemDto {
                    entry_id: entry.entry_id,
                    entry_number: entry.entry_number,
                    transaction_date: entry.transaction_date,
                    status: entry.status,
                    lines: line_dtos,
                }
            })
            .collect();

        Ok(JournalEntrySearchResultDto { entries: entry_dtos, total_count })
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_search_empty_criteria() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(event_store);

        let criteria = SearchCriteriaDto::new();
        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_date_range() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(event_store);

        let criteria = SearchCriteriaDto::new()
            .with_from_date("2024-01-01".to_string())
            .with_to_date("2024-12-31".to_string());

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_description() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(event_store);

        let criteria = SearchCriteriaDto::new().with_description("売上".to_string());

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_account_code() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(event_store);

        let criteria = SearchCriteriaDto::new().with_account_code("1000".to_string());

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_debit_credit() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(event_store);

        let criteria = SearchCriteriaDto::new().with_debit_credit("Debit".to_string());

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_amount_range() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(event_store);

        let criteria = SearchCriteriaDto::new().with_min_amount(10000.0).with_max_amount(100000.0);

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_pagination() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(event_store);

        let criteria = SearchCriteriaDto::new().with_limit(50).with_offset(10);

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }
}
