// UpdateDraftJournalEntryInteractor - 下書き仕訳更新ユースケース実装

use std::sync::Arc;

use chrono::NaiveDate;
use javelin_domain::journal_entry::{repositories::JournalEntryRepository, values::UserId};

use crate::{
    dtos::{UpdateDraftJournalEntryRequest, UpdateDraftJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::UpdateDraftJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct UpdateDraftJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    journal_entry_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> UpdateDraftJournalEntryInteractor<R, O> {
    pub fn new(journal_entry_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { journal_entry_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> UpdateDraftJournalEntryUseCase
    for UpdateDraftJournalEntryInteractor<R, O>
{
    async fn execute(&self, request: UpdateDraftJournalEntryRequest) -> ApplicationResult<()> {
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

        let user_id = UserId::new(request.user_id.clone());

        // 2. 明細が指定されている場合は更新
        if let Some(ref lines_dto) = request.lines {
            // DTOからエンティティに変換
            let lines: Result<Vec<javelin_domain::journal_entry::entities::JournalEntryLine>, _> =
                lines_dto.iter().map(|dto| dto.try_into()).collect();
            let lines = lines?;

            // 集約のupdate_linesメソッドを使用
            journal_entry
                .update_lines(lines, user_id.clone())
                .map_err(ApplicationError::DomainError)?;
        }

        // 3. 取引日付の更新（指定されている場合）
        if let Some(ref date_str) = request.transaction_date {
            use javelin_domain::journal_entry::values::TransactionDate;

            let transaction_date =
                NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
                    ApplicationError::ValidationFailed(vec![format!(
                        "Invalid date format: {}",
                        date_str
                    )])
                })?;
            let transaction_date =
                TransactionDate::new(transaction_date).map_err(ApplicationError::DomainError)?;

            // 集約のupdate_transaction_dateメソッドを使用
            journal_entry
                .update_transaction_date(transaction_date, user_id.clone())
                .map_err(ApplicationError::DomainError)?;
        }

        // 4. 証憑番号の更新（指定されている場合）
        if let Some(ref voucher) = request.voucher_number {
            use javelin_domain::journal_entry::values::VoucherNumber;
            let voucher_number =
                VoucherNumber::new(voucher.clone()).map_err(ApplicationError::DomainError)?;

            // 集約のupdate_voucher_numberメソッドを使用
            journal_entry
                .update_voucher_number(voucher_number, user_id)
                .map_err(ApplicationError::DomainError)?;
        }

        // 5. Repository の save() で永続化
        self.journal_entry_repository
            .save(&journal_entry)
            .await
            .map_err(ApplicationError::DomainError)?;

        // 6. レスポンスを作成
        let response = UpdateDraftJournalEntryResponse {
            entry_id: request.entry_id,
            status: journal_entry.status().as_str().to_string(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_update_draft_result(response).await;

        Ok(())
    }
}
