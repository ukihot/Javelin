// SubmitForApprovalInteractor - 承認申請ユースケース実装

use std::sync::Arc;

use javelin_domain::{
    entity::EntityId,
    financial_close::journal_entry::{events::JournalEntryEvent, values::UserId},
    repositories::JournalEntryRepository,
};

use crate::{
    dtos::{SubmitForApprovalRequest, SubmitForApprovalResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::SubmitForApprovalUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct SubmitForApprovalInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    event_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> SubmitForApprovalInteractor<R, O> {
    pub fn new(event_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { event_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> SubmitForApprovalUseCase
    for SubmitForApprovalInteractor<R, O>
{
    async fn execute(&self, request: SubmitForApprovalRequest) -> ApplicationResult<()> {
        self.output_port
            .notify_progress(format!("承認申請を開始: {}", request.entry_id))
            .await;

        // 承認申請イベントを生成
        let user_id = UserId::new(request.user_id.clone());

        let event = JournalEntryEvent::ApprovalRequested {
            entry_id: request.entry_id.clone(),
            requested_by: user_id.value().to_string(),
            requested_at: chrono::Utc::now(),
        };

        // イベントストアへの保存
        self.event_repository
            .append_events(&request.entry_id, vec![event])
            .await
            .map_err(ApplicationError::DomainError)?;

        let response = SubmitForApprovalResponse {
            entry_id: request.entry_id,
            status: "PendingApproval".to_string(),
            submitted_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_submit_for_approval_result(response).await;

        self.output_port.notify_progress("承認申請が完了".to_string()).await;

        Ok(())
    }
}
