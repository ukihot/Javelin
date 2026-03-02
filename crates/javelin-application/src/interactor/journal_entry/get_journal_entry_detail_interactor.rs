// GetJournalEntryDetailInteractor - 仕訳詳細取得インタラクター
// 仕訳IDを受け取り、詳細情報を取得してOutputPortに渡す

use std::sync::Arc;

use crate::{
    error::{ApplicationError, ApplicationResult},
    input_ports::GetJournalEntryDetailUseCase,
    output_ports::QueryOutputPort,
    query_service::JournalEntrySearchQueryService,
};

/// 仕訳詳細取得インタラクター
pub struct GetJournalEntryDetailInteractor<Q, O>
where
    Q: JournalEntrySearchQueryService,
    O: QueryOutputPort,
{
    query_service: Arc<Q>,
    output_port: Arc<O>,
}

impl<Q, O> GetJournalEntryDetailInteractor<Q, O>
where
    Q: JournalEntrySearchQueryService,
    O: QueryOutputPort,
{
    /// 新しいインタラクターインスタンスを作成
    pub fn new(query_service: Arc<Q>, output_port: Arc<O>) -> Self {
        Self { query_service, output_port }
    }
}

impl<Q, O> GetJournalEntryDetailUseCase for GetJournalEntryDetailInteractor<Q, O>
where
    Q: JournalEntrySearchQueryService,
    O: QueryOutputPort,
{
    async fn execute(&self, entry_id: String) -> ApplicationResult<()> {
        // Query Serviceから仕訳詳細を取得
        let detail = self.query_service.get_detail(&entry_id).await?;

        // 詳細が見つからない場合はエラー
        let Some(detail) = detail else {
            return Err(ApplicationError::NotFound(format!(
                "Journal entry not found: {}",
                entry_id
            )));
        };

        // OutputPortに結果を渡す
        self.output_port.present_journal_entry_detail(detail).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtos::{
        request::SearchCriteriaDto,
        response::{JournalEntryDetail, JournalEntryLineDetail},
    };

    struct MockQueryService {
        detail: Option<JournalEntryDetail>,
    }

    impl JournalEntrySearchQueryService for MockQueryService {
        async fn search(
            &self,
            _criteria: SearchCriteriaDto,
        ) -> ApplicationResult<crate::dtos::response::JournalEntrySearchResultDto> {
            Ok(crate::dtos::response::JournalEntrySearchResultDto {
                entries: vec![],
                total_count: 0,
            })
        }

        async fn get_voucher_numbers_by_fiscal_year(
            &self,
            _fiscal_year: u32,
        ) -> ApplicationResult<Vec<String>> {
            Ok(vec![])
        }

        async fn get_detail(
            &self,
            _entry_id: &str,
        ) -> ApplicationResult<Option<JournalEntryDetail>> {
            Ok(self.detail.clone())
        }
    }

    struct MockOutputPort {
        presented: Arc<tokio::sync::Mutex<Option<JournalEntryDetail>>>,
    }

    impl QueryOutputPort for MockOutputPort {
        async fn present_journal_entry_list(
            &self,
            _result: crate::dtos::response::JournalEntryListResult,
        ) {
        }

        async fn present_journal_entry_detail(&self, result: JournalEntryDetail) {
            *self.presented.lock().await = Some(result);
        }

        async fn present_ledger(&self, _result: crate::query_service::LedgerResult) {}

        async fn present_trial_balance(&self, _result: crate::query_service::TrialBalanceResult) {}
    }

    #[tokio::test]
    async fn test_execute_success() {
        let detail = JournalEntryDetail {
            entry_id: "entry-123".to_string(),
            entry_number: Some("JE-2024-001".to_string()),
            status: "Posted".to_string(),
            transaction_date: "2024-01-15".to_string(),
            voucher_number: "V-001".to_string(),
            lines: vec![JournalEntryLineDetail {
                line_number: 1,
                side: "Debit".to_string(),
                account_code: "1010".to_string(),
                account_name: "現金".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 10000.0,
                currency: "JPY".to_string(),
                tax_type: "None".to_string(),
                tax_amount: 0.0,
            }],
            created_by: "user1".to_string(),
            created_at: "2024-01-15T10:00:00Z".to_string(),
            updated_by: None,
            updated_at: None,
            approved_by: Some("manager1".to_string()),
            approved_at: Some("2024-01-15T11:00:00Z".to_string()),
        };

        let query_service = Arc::new(MockQueryService { detail: Some(detail.clone()) });
        let presented = Arc::new(tokio::sync::Mutex::new(None));
        let output_port = Arc::new(MockOutputPort { presented: Arc::clone(&presented) });

        let interactor = GetJournalEntryDetailInteractor::new(query_service, output_port);

        let result = interactor.execute("entry-123".to_string()).await;
        assert!(result.is_ok());

        let presented_detail = presented.lock().await;
        assert!(presented_detail.is_some());
        assert_eq!(presented_detail.as_ref().unwrap().entry_id, "entry-123");
    }

    #[tokio::test]
    async fn test_execute_not_found() {
        let query_service = Arc::new(MockQueryService { detail: None });
        let presented = Arc::new(tokio::sync::Mutex::new(None));
        let output_port = Arc::new(MockOutputPort { presented: Arc::clone(&presented) });

        let interactor = GetJournalEntryDetailInteractor::new(query_service, output_port);

        let result = interactor.execute("entry-999".to_string()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));

        let presented_detail = presented.lock().await;
        assert!(presented_detail.is_none());
    }
}
