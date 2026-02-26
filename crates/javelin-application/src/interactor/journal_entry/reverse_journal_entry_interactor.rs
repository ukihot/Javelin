// ReverseJournalEntryInteractor - 取消ユースケース実装

use std::sync::Arc;

use javelin_domain::{
    entity::EntityId,
    financial_close::journal_entry::{events::JournalEntryEvent, values::UserId},
    repositories::JournalEntryRepository,
};

use crate::{
    dtos::{ReverseJournalEntryRequest, ReverseJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::ReverseJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct ReverseJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    event_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> ReverseJournalEntryInteractor<R, O> {
    pub fn new(event_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { event_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> ReverseJournalEntryUseCase
    for ReverseJournalEntryInteractor<R, O>
{
    async fn execute(&self, request: ReverseJournalEntryRequest) -> ApplicationResult<()> {
        // 取消イベントを生成
        let user_id = UserId::new(request.user_id.clone());
        let reversal_entry_id = format!("REV-{}", request.entry_id);

        let event = JournalEntryEvent::Reversed {
            entry_id: reversal_entry_id.clone(),
            original_id: request.entry_id.clone(),
            reason: request.reason.clone(),
            reversed_by: user_id.value().to_string(),
            reversed_at: chrono::Utc::now(),
        };

        // イベントストアへの保存
        self.event_repository
            .append_events(&reversal_entry_id, vec![event])
            .await
            .map_err(ApplicationError::DomainError)?;

        let response = ReverseJournalEntryResponse {
            entry_id: reversal_entry_id,
            original_entry_id: request.entry_id,
            status: "Reversed".to_string(),
            reversed_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_reverse_result(response).await;

        Ok(())
    }
}
