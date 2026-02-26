// Application層ユースケース: CancelJournalEntry
// 正常系・異常系・イベント保存失敗のテスト

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use javelin_domain::{
        financial_close::journal_entry::events::JournalEntryEvent,
        repositories::{JournalEntryRepository, RepositoryBase},
    };
    use tokio::sync::mpsc;

    use crate::{
        dtos::{CancelJournalEntryRequest, RegisterJournalEntryResponse},
        input_ports::CancelJournalEntryUseCase,
        interactor::CancelJournalEntryInteractor,
        output_ports::JournalEntryOutputPort,
        query_service::JournalEntryFinderService,
    };

    /// モックEventRepository
    struct MockEventRepository {
        saved_events: Arc<Mutex<Vec<(String, Vec<serde_json::Value>)>>>,
    }

    impl MockEventRepository {
        fn new() -> Self {
            Self { saved_events: Arc::new(Mutex::new(Vec::new())) }
        }
    }

    impl RepositoryBase for MockEventRepository {
        type Event = JournalEntryEvent;

        async fn append(&self, _event: Self::Event) -> javelin_domain::error::DomainResult<()> {
            Ok(())
        }

        async fn append_events<T>(
            &self,
            aggregate_id: &str,
            events: Vec<T>,
        ) -> javelin_domain::error::DomainResult<u64>
        where
            T: serde::Serialize + Send + 'static,
        {
            let json_events: Vec<serde_json::Value> =
                events.into_iter().map(|e| serde_json::to_value(e).unwrap()).collect();

            self.saved_events
                .lock()
                .unwrap()
                .push((aggregate_id.to_string(), json_events.clone()));

            Ok(json_events.len() as u64)
        }

        async fn get_events(
            &self,
            _aggregate_id: &str,
        ) -> javelin_domain::error::DomainResult<Vec<serde_json::Value>> {
            Ok(vec![])
        }

        async fn get_all_events(
            &self,
            _from_sequence: u64,
        ) -> javelin_domain::error::DomainResult<Vec<serde_json::Value>> {
            Ok(vec![])
        }

        async fn get_latest_sequence(&self) -> javelin_domain::error::DomainResult<u64> {
            Ok(0)
        }
    }

    impl JournalEntryRepository for MockEventRepository {}

    /// モックJournalEntryFinderService
    struct MockJournalEntryFinderService;

    #[allow(async_fn_in_trait)]
    impl JournalEntryFinderService for MockJournalEntryFinderService {
        async fn find_by_entry_number(
            &self,
            _entry_number: &str,
        ) -> crate::error::ApplicationResult<Option<crate::query_service::JournalEntrySearchResult>>
        {
            Ok(Some(crate::query_service::JournalEntrySearchResult {
                entry_id: "E001".to_string(),
                entry_number: Some("V-001".to_string()),
                transaction_date: "2024-01-15".to_string(),
                total_debit: 0,
                total_credit: 0,
                status: "Draft".to_string(),
            }))
        }

        async fn find_by_voucher_number(
            &self,
            _voucher_number: &str,
        ) -> crate::error::ApplicationResult<Vec<crate::query_service::JournalEntrySearchResult>>
        {
            Ok(vec![crate::query_service::JournalEntrySearchResult {
                entry_id: "E001".to_string(),
                entry_number: Some("V-001".to_string()),
                transaction_date: "2024-01-15".to_string(),
                total_debit: 0,
                total_credit: 0,
                status: "Draft".to_string(),
            }])
        }

        async fn find_by_date_range(
            &self,
            _from_date: &str,
            _to_date: &str,
        ) -> crate::error::ApplicationResult<Vec<crate::query_service::JournalEntrySearchResult>>
        {
            Ok(vec![])
        }

        async fn list_journal_entries(
            &self,
            _query: crate::dtos::ListJournalEntriesQuery,
        ) -> crate::error::ApplicationResult<()> {
            Ok(())
        }

        async fn get_journal_entry(
            &self,
            _query: crate::dtos::GetJournalEntryQuery,
        ) -> crate::error::ApplicationResult<()> {
            Ok(())
        }
    }

    /// モックJournalEntryOutputPort
    struct MockJournalEntryOutputPort {
        sender: mpsc::UnboundedSender<RegisterJournalEntryResponse>,
    }

    impl JournalEntryOutputPort for MockJournalEntryOutputPort {
        async fn present_register_result(&self, response: RegisterJournalEntryResponse) {
            let _ = self.sender.send(response);
        }

        async fn notify_progress(&self, _message: String) {}

        async fn notify_error(&self, _error_message: String) {}

        async fn present_approve_result(
            &self,
            _response: crate::dtos::ApproveJournalEntryResponse,
        ) {
        }

        async fn present_reject_result(&self, _response: crate::dtos::RejectJournalEntryResponse) {}

        async fn present_update_draft_result(
            &self,
            _response: crate::dtos::UpdateDraftJournalEntryResponse,
        ) {
        }

        async fn present_delete_draft_result(
            &self,
            _response: crate::dtos::DeleteDraftJournalEntryResponse,
        ) {
        }

        async fn present_correct_result(
            &self,
            _response: crate::dtos::CorrectJournalEntryResponse,
        ) {
        }

        async fn present_reverse_result(
            &self,
            _response: crate::dtos::ReverseJournalEntryResponse,
        ) {
        }

        async fn present_submit_for_approval_result(
            &self,
            _response: crate::dtos::SubmitForApprovalResponse,
        ) {
        }
    }

    #[tokio::test]
    async fn test_validation_error_empty_entry_id() {
        let repo = Arc::new(MockEventRepository::new());
        let (sender, _receiver) = mpsc::unbounded_channel();
        let output_port = Arc::new(MockJournalEntryOutputPort { sender });
        let finder_service = Arc::new(MockJournalEntryFinderService);

        let interactor = CancelJournalEntryInteractor::new(repo, output_port, finder_service);
        let request = CancelJournalEntryRequest {
            reference_entry_id: "".to_string(),
            transaction_date: "2024-01-15".to_string(),
            voucher_number: "V-002".to_string(),
            user_id: "user1".to_string(),
        };
        let result = interactor.execute(request).await;
        assert!(result.is_err());
    }
}
