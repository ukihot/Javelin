// CorrectJournalEntryInteractor - 修正ユースケース実装

use std::sync::Arc;

use javelin_domain::{
    entity::EntityId,
    financial_close::journal_entry::{events::JournalEntryEvent, values::UserId},
    repositories::JournalEntryRepository,
};

use crate::{
    dtos::{CorrectJournalEntryRequest, CorrectJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::CorrectJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct CorrectJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    event_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> CorrectJournalEntryInteractor<R, O> {
    pub fn new(event_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { event_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> CorrectJournalEntryUseCase
    for CorrectJournalEntryInteractor<R, O>
{
    async fn execute(&self, request: CorrectJournalEntryRequest) -> ApplicationResult<()> {
        // 修正イベントを生成
        let user_id = UserId::new(request.user_id.clone());
        let correction_entry_id = format!("COR-{}", request.reversed_entry_id);

        let event = JournalEntryEvent::Corrected {
            entry_id: correction_entry_id.clone(),
            reversed_id: request.reversed_entry_id.clone(),
            reason: request.reason.clone(),
            corrected_by: user_id.value().to_string(),
            corrected_at: chrono::Utc::now(),
        };

        // イベントストアへの保存
        self.event_repository
            .append_events(&correction_entry_id, vec![event])
            .await
            .map_err(ApplicationError::DomainError)?;

        let response = CorrectJournalEntryResponse {
            entry_id: correction_entry_id,
            reversed_entry_id: request.reversed_entry_id,
            status: "Corrected".to_string(),
            corrected_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_correct_result(response).await;

        Ok(())
    }
}
