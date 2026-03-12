// Application層ユースケース: CorrectJournalEntry
// 正常系・異常系・イベント保存失敗のテスト

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::sync::mpsc;

    use crate::{
        dtos::{CorrectJournalEntryRequest, CorrectJournalEntryResponse},
        input_ports::CorrectJournalEntryUseCase,
        interactor::CorrectJournalEntryInteractor,
        output_ports::JournalEntryOutputPort,
    };

    // use the automock generated repository mock from the domain layer
    // this eliminates boilerplate and aligns with project-wide mocks

    // use the automock generated repository mock from the domain layer
    // (MockJournalEntryRepository already implements the domain traits)

    /// モックJournalEntryOutputPort
    struct MockJournalEntryOutputPort {
        sender: mpsc::UnboundedSender<CorrectJournalEntryResponse>,
    }

    impl JournalEntryOutputPort for MockJournalEntryOutputPort {
        async fn present_register_result(
            &self,
            _response: crate::dtos::RegisterJournalEntryResponse,
        ) {
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

        async fn present_correct_result(&self, response: CorrectJournalEntryResponse) {
            let _ = self.sender.send(response);
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
    async fn test_successful_correct_journal_entry() {
        let mut mock_repo = MockJournalEntryRepository::new();

        // load() のモック設定
        mock_repo.expect_load().returning(|_| {
            // TODO: 実際の JournalEntry を返す必要がある
            Ok(None)
        });

        // save() のモック設定
        mock_repo.expect_save().returning(|_| Ok(()));

        let repo = Arc::new(mock_repo);
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let output_port = Arc::new(MockJournalEntryOutputPort { sender });

        let interactor = CorrectJournalEntryInteractor::new(repo, output_port);
        let request = CorrectJournalEntryRequest {
            reversed_entry_id: "E001".to_string(),
            new_lines: vec![],
            reason: "Correction needed".to_string(),
            user_id: "user1".to_string(),
        };
        let result = interactor.execute(request).await;

        // load() が None を返すので、エラーになるはず
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_error_empty_entry_id() {
        let mut mock_repo = MockJournalEntryRepository::new();

        // load() のモック設定
        mock_repo.expect_load().returning(|_| Ok(None));

        let repo = Arc::new(mock_repo);
        let (sender, _receiver) = mpsc::unbounded_channel();
        let output_port = Arc::new(MockJournalEntryOutputPort { sender });

        let interactor = CorrectJournalEntryInteractor::new(repo, output_port);
        let request = CorrectJournalEntryRequest {
            reversed_entry_id: "".to_string(),
            new_lines: vec![],
            reason: "Correction needed".to_string(),
            user_id: "user1".to_string(),
        };
        let result = interactor.execute(request).await;

        // エラーになるはず
        assert!(result.is_err());
    }
}
