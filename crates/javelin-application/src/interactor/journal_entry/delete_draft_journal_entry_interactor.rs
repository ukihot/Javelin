// DeleteDraftJournalEntryInteractor - 下書き削除ユースケース実装

use std::sync::Arc;

use javelin_domain::{
    entity::EntityId,
    financial_close::journal_entry::{events::JournalEntryEvent, values::UserId},
    repositories::JournalEntryRepository,
};

use crate::{
    dtos::{DeleteDraftJournalEntryRequest, DeleteDraftJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::DeleteDraftJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct DeleteDraftJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    event_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> DeleteDraftJournalEntryInteractor<R, O> {
    pub fn new(event_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { event_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> DeleteDraftJournalEntryUseCase
    for DeleteDraftJournalEntryInteractor<R, O>
{
    async fn execute(&self, request: DeleteDraftJournalEntryRequest) -> ApplicationResult<()> {
        // 削除イベントを生成
        let user_id = UserId::new(request.user_id.clone());

        let event = JournalEntryEvent::Deleted {
            entry_id: request.entry_id.clone(),
            deleted_by: user_id.value().to_string(),
            deleted_at: chrono::Utc::now(),
        };

        // イベントストアへの保存
        self.event_repository
            .append_events(&request.entry_id, vec![event])
            .await
            .map_err(ApplicationError::DomainError)?;

        let response = DeleteDraftJournalEntryResponse {
            entry_id: request.entry_id,
            deleted_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_delete_draft_result(response).await;

        Ok(())
    }
}
