// CorrectJournalEntryInteractor - 修正ユースケース実装

use std::sync::Arc;

use javelin_domain::{
    entity::{Entity, EntityId},
    journal_entry::{repositories::JournalEntryRepository, values::UserId},
};

use crate::{
    dtos::{CorrectJournalEntryRequest, CorrectJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::CorrectJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct CorrectJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    journal_entry_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> CorrectJournalEntryInteractor<R, O> {
    pub fn new(journal_entry_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { journal_entry_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> CorrectJournalEntryUseCase
    for CorrectJournalEntryInteractor<R, O>
{
    async fn execute(&self, request: CorrectJournalEntryRequest) -> ApplicationResult<()> {
        // 1. Repository の load() で集約を復元
        let mut journal_entry = self
            .journal_entry_repository
            .load(&request.reversed_entry_id)
            .await
            .map_err(ApplicationError::DomainError)?
            .ok_or_else(|| {
                ApplicationError::ValidationFailed(vec![format!(
                    "Journal entry not found: {}",
                    request.reversed_entry_id
                )])
            })?;

        // 2. 修正処理を実行（集約内部でイベントが生成される）
        let user_id = UserId::new(request.user_id.clone());
        journal_entry
            .correct(request.reversed_entry_id.clone(), request.reason.clone(), user_id)
            .map_err(ApplicationError::DomainError)?;

        // 3. Repository の save() で永続化
        self.journal_entry_repository
            .save(&journal_entry)
            .await
            .map_err(ApplicationError::DomainError)?;

        // 4. レスポンスを作成
        let response = CorrectJournalEntryResponse {
            entry_id: journal_entry.id().value().to_string(),
            reversed_entry_id: request.reversed_entry_id,
            status: journal_entry.status().as_str().to_string(),
            corrected_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_correct_result(response).await;

        Ok(())
    }
}
