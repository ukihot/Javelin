// Application層ユースケース: CancelJournalEntry
// 正常系・異常系・イベント保存失敗のテスト

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use javelin_domain::journal_entry::{
        events::JournalEntryEvent, repositories::MockJournalEntryRepository,
    };
    use tokio::sync::mpsc;

    use crate::{
        dtos::{CancelJournalEntryRequest, RegisterJournalEntryResponse},
        input_ports::CancelJournalEntryUseCase,
        interactor::CancelJournalEntryInteractor,
        output_ports::JournalEntryOutputPort,
        query_service::JournalEntryFinderService,
    };

    // use auto-generated mock from domain rather than hand-rolled repository
    // (MockJournalEntryRepository already implements RepositoryBase<Event = JournalEntryEvent>)

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
        let mut mock_repo = MockJournalEntryRepository::new();
        mock_repo.expect_append().returning(|_| Ok(()));
        mock_repo.expect_append_events::<JournalEntryEvent>().returning(|_, _| Ok(0));
        mock_repo.expect_get_events().returning(|_| Ok(vec![]));
        mock_repo.expect_get_all_events().returning(|_| Ok(vec![]));
        mock_repo.expect_get_latest_sequence().returning(|| Ok(0));
        let repo = Arc::new(mock_repo);
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
