// DeleteDraftJournalEntryInteractor - 下書き削除ユースケース実装

use std::sync::Arc;

use javelin_domain::journal_entry::{repositories::JournalEntryRepository, values::UserId};

use crate::{
    dtos::{DeleteDraftJournalEntryRequest, DeleteDraftJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::DeleteDraftJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct DeleteDraftJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    journal_entry_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> DeleteDraftJournalEntryInteractor<R, O> {
    pub fn new(journal_entry_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { journal_entry_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> DeleteDraftJournalEntryUseCase
    for DeleteDraftJournalEntryInteractor<R, O>
{
    async fn execute(&self, request: DeleteDraftJournalEntryRequest) -> ApplicationResult<()> {
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

        // 2. 削除処理を実行（集約内部でイベントが生成される）
        let user_id = UserId::new(request.user_id.clone());
        journal_entry.delete(user_id).map_err(ApplicationError::DomainError)?;

        // 3. Repository の save() で永続化
        self.journal_entry_repository
            .save(&journal_entry)
            .await
            .map_err(ApplicationError::DomainError)?;

        // 4. レスポンスを作成
        let response = DeleteDraftJournalEntryResponse {
            entry_id: request.entry_id,
            deleted_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_delete_draft_result(response).await;

        Ok(())
    }
}
