// RejectJournalEntryInteractor - 差戻しユースケース実装

use std::sync::Arc;

use javelin_domain::{
    entity::EntityId,
    financial_close::journal_entry::{events::JournalEntryEvent, values::UserId},
    repositories::JournalEntryRepository,
};

use crate::{
    dtos::{RejectJournalEntryRequest, RejectJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::RejectJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct RejectJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    event_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> RejectJournalEntryInteractor<R, O> {
    pub fn new(event_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { event_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> RejectJournalEntryUseCase
    for RejectJournalEntryInteractor<R, O>
{
    async fn execute(&self, request: RejectJournalEntryRequest) -> ApplicationResult<()> {
        // 差戻しイベントを生成
        let user_id = UserId::new(request.rejected_by.clone());

        let event = JournalEntryEvent::Rejected {
            entry_id: request.entry_id.clone(),
            reason: request.reason.clone(),
            rejected_by: user_id.value().to_string(),
            rejected_at: chrono::Utc::now(),
        };

        // イベントストアへの保存
        self.event_repository
            .append_events(&request.entry_id, vec![event])
            .await
            .map_err(ApplicationError::DomainError)?;

        let response = RejectJournalEntryResponse {
            entry_id: request.entry_id,
            status: "Draft".to_string(),
            rejected_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_reject_result(response).await;

        Ok(())
    }
}
