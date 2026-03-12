// ApproveJournalEntryInteractor - 承認ユースケース実装

use std::sync::Arc;

use javelin_domain::journal_entry::{
    repositories::JournalEntryRepository,
    values::{EntryNumber, UserId},
};

use crate::{
    dtos::{ApproveJournalEntryRequest, ApproveJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::ApproveJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct ApproveJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    journal_entry_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> ApproveJournalEntryInteractor<R, O> {
    pub fn new(journal_entry_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { journal_entry_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> ApproveJournalEntryUseCase
    for ApproveJournalEntryInteractor<R, O>
{
    async fn execute(&self, request: ApproveJournalEntryRequest) -> ApplicationResult<()> {
        // 1. Repository の load() で集約を復元 load()
        //    はインフラ層でイベントストリームから集約を再構築する
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

        // 2. 伝票番号を生成
        let entry_number =
            EntryNumber::new(format!("EN-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")))
                .map_err(ApplicationError::DomainError)?;

        // 3. 承認処理を実行（集約内部でイベントが生成される）
        let user_id = UserId::new(request.approver_id.clone());
        journal_entry
            .approve(entry_number.clone(), user_id)
            .map_err(ApplicationError::DomainError)?;

        // 4. Repository の save() で永続化 save() はインフラ層で集約の uncommitted_events
        //    をイベントストアに保存する
        self.journal_entry_repository
            .save(&journal_entry)
            .await
            .map_err(ApplicationError::DomainError)?;

        // 5. レスポンスを作成
        let response = ApproveJournalEntryResponse {
            entry_id: request.entry_id,
            entry_number: entry_number.value().to_string(),
            status: journal_entry.status().as_str().to_string(),
            approved_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_approve_result(response).await;

        Ok(())
    }
}
