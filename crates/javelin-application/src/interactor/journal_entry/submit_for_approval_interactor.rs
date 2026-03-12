// SubmitForApprovalInteractor - 承認申請ユースケース実装

use std::sync::Arc;

use javelin_domain::journal_entry::{repositories::JournalEntryRepository, values::UserId};

use crate::{
    dtos::{SubmitForApprovalRequest, SubmitForApprovalResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::SubmitForApprovalUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct SubmitForApprovalInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    journal_entry_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> SubmitForApprovalInteractor<R, O> {
    pub fn new(journal_entry_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { journal_entry_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> SubmitForApprovalUseCase
    for SubmitForApprovalInteractor<R, O>
{
    async fn execute(&self, request: SubmitForApprovalRequest) -> ApplicationResult<()> {
        self.output_port
            .notify_progress(format!("承認申請を開始: {}", request.entry_id))
            .await;

        // 1. Repository の load() で集約を復元
        let mut journal_entry = self
            .journal_entry_repository
            .load(&request.entry_id)
            .await
            .map_err(ApplicationError::DomainError)?
            .ok_or_else(|| {
                ApplicationError::ValidationFailed(vec![format!(
                    "Journal entry not found: {}",
                    request.entry_id
                )])
            })?;

        // 2. 承認申請処理を実行（集約内部でイベントが生成される）
        let user_id = UserId::new(request.user_id.clone());
        journal_entry
            .submit_for_approval(user_id)
            .map_err(ApplicationError::DomainError)?;

        // 3. Repository の save() で永続化
        self.journal_entry_repository
            .save(&journal_entry)
            .await
            .map_err(ApplicationError::DomainError)?;

        // 4. レスポンスを作成
        let response = SubmitForApprovalResponse {
            entry_id: request.entry_id,
            status: journal_entry.status().as_str().to_string(),
            submitted_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_submit_for_approval_result(response).await;

        self.output_port.notify_progress("承認申請が完了".to_string()).await;

        Ok(())
    }
}
