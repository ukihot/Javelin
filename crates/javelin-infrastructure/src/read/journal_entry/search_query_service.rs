// JournalEntrySearchQueryServiceImpl - 仕訳検索サービス実装（Infrastructure層）
// ProjectionDBから仕訳データを検索（CQRS読み取り側）

use std::sync::Arc;

use javelin_application::{
    dtos::{
        request::SearchCriteriaDto,
        response::{JournalEntryItemDto, JournalEntryLineItemDto, JournalEntrySearchResultDto},
    },
    error::{ApplicationError, ApplicationResult},
    query_service::JournalEntrySearchQueryService,
};

use crate::read::{
    infrastructure::db::ProjectionDb, journal_entry::search_projection::JournalEntrySearchReadModel,
};

/// JournalEntrySearchQueryService実装
///
/// ProjectionDBから仕訳データを検索する。
/// CQRS原則: クエリサービスはProjectionDB（読み取り最適化）を使用
pub struct JournalEntrySearchQueryServiceImpl {
    projection_db: Arc<ProjectionDb>,
}

impl JournalEntrySearchQueryServiceImpl {
    /// 新しいインスタンスを作成
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection_db }
    }

    /// ProjectionDBから仕訳エントリを取得
    async fn get_journal_entries(&self) -> ApplicationResult<Vec<JournalEntrySearchReadModel>> {
        // ProjectionDBから仕訳エントリを取得
        // キー形式: "journal_entry:{entry_id}"
        // プレフィックススキャンで全エントリを取得
        let results = self
            .projection_db
            .scan_prefix("journal_entry:")
            .await
            .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

        let mut entries = Vec::with_capacity(results.len());

        for (_key, data) in results {
            let entry: JournalEntrySearchReadModel = serde_json::from_slice(&data)
                .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

            entries.push(entry);
        }

        Ok(entries)
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
        // ProjectionDBから仕訳エントリを取得
        let mut entries = self.get_journal_entries().await?;

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

    async fn get_voucher_numbers_by_fiscal_year(
        &self,
        fiscal_year: u32,
    ) -> ApplicationResult<Vec<String>> {
        // ProjectionDBから仕訳エントリを取得
        let entries = self.get_journal_entries().await?;

        // 指定された会計年度の伝票番号のみを抽出
        // 取引日付から年度を判定（簡易的に年を使用）
        let voucher_numbers: Vec<String> = entries
            .into_iter()
            .filter_map(|entry| {
                // transaction_date形式: "YYYY-MM-DD"
                if let Some(year_str) = entry.transaction_date.split('-').next()
                    && let Ok(year) = year_str.parse::<u32>()
                    && year == fiscal_year
                {
                    // entry_numberがSomeの場合のみ返す
                    return entry.entry_number;
                }
                None
            })
            .collect();

        Ok(voucher_numbers)
    }

    async fn get_detail(
        &self,
        entry_id: &str,
    ) -> ApplicationResult<Option<javelin_application::dtos::response::JournalEntryDetail>> {
        // ProjectionDBから仕訳詳細を取得
        let key = format!("journal_entry:{}", entry_id);

        let Some(data) = self
            .projection_db
            .get_projection(&key)
            .await
            .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
        else {
            return Ok(None);
        };

        let entry: JournalEntrySearchReadModel = serde_json::from_slice(&data)
            .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

        // JournalEntryDetailに変換
        let lines = entry
            .lines
            .into_iter()
            .map(|line| javelin_application::dtos::response::JournalEntryLineDetail {
                line_number: line.line_number,
                side: line.side,
                account_code: line.account_code,
                account_name: line.account_name,
                sub_account_code: None, // TODO: 補助科目対応
                department_code: None,  // TODO: 部門対応
                amount: line.amount,
                currency: "JPY".to_string(),  // TODO: 通貨対応
                tax_type: "None".to_string(), // TODO: 税区分対応
                tax_amount: 0.0,              // TODO: 税額対応
            })
            .collect();

        let detail = javelin_application::dtos::response::JournalEntryDetail {
            entry_id: entry.entry_id,
            entry_number: entry.entry_number,
            status: entry.status,
            transaction_date: entry.transaction_date,
            voucher_number: "V-001".to_string(), // TODO: 証憑番号対応
            lines,
            created_by: "system".to_string(), // TODO: 作成者対応
            created_at: chrono::Utc::now().to_rfc3339(), // TODO: 作成日時対応
            updated_by: None,
            updated_at: None,
            approved_by: None,
            approved_at: None,
        };

        Ok(Some(detail))
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_search_empty_criteria() {
        let temp_dir = TempDir::new().unwrap();
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(projection_db);

        let criteria = SearchCriteriaDto::new();
        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_date_range() {
        let temp_dir = TempDir::new().unwrap();
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(projection_db);

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
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(projection_db);

        let criteria = SearchCriteriaDto::new().with_description("売上".to_string());

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_account_code() {
        let temp_dir = TempDir::new().unwrap();
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(projection_db);

        let criteria = SearchCriteriaDto::new().with_account_code("1000".to_string());

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_debit_credit() {
        let temp_dir = TempDir::new().unwrap();
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(projection_db);

        let criteria = SearchCriteriaDto::new().with_debit_credit("Debit".to_string());

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_amount_range() {
        let temp_dir = TempDir::new().unwrap();
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(projection_db);

        let criteria = SearchCriteriaDto::new().with_min_amount(10000.0).with_max_amount(100000.0);

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_pagination() {
        let temp_dir = TempDir::new().unwrap();
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
        let service = JournalEntrySearchQueryServiceImpl::new(projection_db);

        let criteria = SearchCriteriaDto::new().with_limit(50).with_offset(10);

        let result = service.search(criteria).await.unwrap();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }
}
