// CreateAdditionalEntryInteractor - 追加仕訳登録ユースケース実装
// 責務: 計上不足または後日判明事項を補正する追加仕訳を登録する

use std::sync::Arc;

use chrono::NaiveDate;
use javelin_domain::{
    entity::EntityId,
    journal_entry::{
        entities::{JournalEntry, JournalEntryId},
        repositories::JournalEntryRepository,
        values::{TransactionDate, UserId, VoucherNumber},
    },
};

use crate::{
    dtos::{CreateAdditionalEntryRequest, RegisterJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::CreateAdditionalEntryUseCase,
    output_ports::JournalEntryOutputPort,
    query_service::JournalEntryFinderService,
};

pub struct CreateAdditionalEntryInteractor<
    R: JournalEntryRepository,
    O: JournalEntryOutputPort,
    F: JournalEntryFinderService,
> {
    journal_entry_repository: Arc<R>,
    output_port: Arc<O>,
    finder_service: Arc<F>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort, F: JournalEntryFinderService>
    CreateAdditionalEntryInteractor<R, O, F>
{
    pub fn new(
        journal_entry_repository: Arc<R>,
        output_port: Arc<O>,
        finder_service: Arc<F>,
    ) -> Self {
        Self { journal_entry_repository, output_port, finder_service }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort, F: JournalEntryFinderService>
    CreateAdditionalEntryUseCase for CreateAdditionalEntryInteractor<R, O, F>
{
    async fn execute(&self, request: CreateAdditionalEntryRequest) -> ApplicationResult<()> {
        let _reference_entry = self
            .finder_service
            .find_by_entry_number(&request.reference_entry_id)
            .await?
            .ok_or_else(|| {
                ApplicationError::ValidationFailed(vec![format!(
                    "参照元伝票が見つかりません: {}",
                    request.reference_entry_id
                )])
            })?;

        let transaction_date = NaiveDate::parse_from_str(&request.transaction_date, "%Y-%m-%d")
            .map_err(|_| {
                ApplicationError::ValidationFailed(vec![format!(
                    "Invalid date format: {}",
                    request.transaction_date
                )])
            })?;
        let transaction_date =
            TransactionDate::new(transaction_date).map_err(ApplicationError::DomainError)?;

        let voucher_number = VoucherNumber::new(request.voucher_number.clone())
            .map_err(ApplicationError::DomainError)?;
        let user_id = UserId::new(request.user_id.clone());

        let lines: Result<Vec<_>, _> = request.lines.iter().map(|dto| dto.try_into()).collect();
        let lines = lines?;

        // JournalEntry::new() 内部でバランスチェックが行われる

        let entry_id = JournalEntryId::new(uuid::Uuid::new_v4().to_string());
        let journal_entry =
            JournalEntry::new(entry_id.clone(), transaction_date, voucher_number, lines, user_id)
                .map_err(ApplicationError::DomainError)?;

        self.journal_entry_repository
            .save(&journal_entry)
            .await
            .map_err(ApplicationError::DomainError)?;

        let response = RegisterJournalEntryResponse {
            entry_id: entry_id.value().to_string(),
            status: journal_entry.status().as_str().to_string(),
        };
        self.output_port.present_register_result(response).await;

        Ok(())
    }
}
