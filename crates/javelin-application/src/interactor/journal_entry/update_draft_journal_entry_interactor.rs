// UpdateDraftJournalEntryInteractor - 下書き仕訳更新ユースケース実装

use std::sync::Arc;

use chrono::NaiveDate;
use javelin_domain::{
    entity::EntityId,
    financial_close::journal_entry::{
        entities::JournalEntryLine,
        events::{JournalEntryEvent, JournalEntryLineDto},
        values::UserId,
    },
    repositories::JournalEntryRepository,
};

use crate::{
    dtos::{UpdateDraftJournalEntryRequest, UpdateDraftJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::UpdateDraftJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct UpdateDraftJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    event_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> UpdateDraftJournalEntryInteractor<R, O> {
    pub fn new(event_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { event_repository, output_port }
    }

    /// DTOからイベント用のJournalEntryLineDtoを作成
    fn convert_to_event_line_dto(&self, line: &JournalEntryLine) -> JournalEntryLineDto {
        JournalEntryLineDto::from_entity(line)
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> UpdateDraftJournalEntryUseCase
    for UpdateDraftJournalEntryInteractor<R, O>
{
    async fn execute(&self, request: UpdateDraftJournalEntryRequest) -> ApplicationResult<()> {
        // 1. 明細が指定されている場合は変換とバリデーション
        let event_lines = if let Some(ref lines_dto) = request.lines {
            use javelin_domain::financial_close::journal_entry::services::JournalEntryDomainService;

            // DTOからエンティティに変換（TryFromを使用）
            let lines: Result<Vec<_>, _> = lines_dto.iter().map(|dto| dto.try_into()).collect();
            let lines = lines?;

            // 借貸バランスチェック
            JournalEntryDomainService::validate_balance(&lines)
                .map_err(ApplicationError::DomainError)?;

            // イベント用のDTOに変換
            let event_lines: Vec<JournalEntryLineDto> =
                lines.iter().map(|line| self.convert_to_event_line_dto(line)).collect();

            Some(event_lines)
        } else {
            None
        };

        // 2. 取引日付のバリデーション（指定されている場合）
        if let Some(ref date_str) = request.transaction_date {
            use javelin_domain::financial_close::journal_entry::values::TransactionDate;

            let transaction_date =
                NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
                    ApplicationError::ValidationFailed(vec![format!(
                        "Invalid date format: {}",
                        date_str
                    )])
                })?;
            TransactionDate::new(transaction_date).map_err(ApplicationError::DomainError)?;
        }

        // 3. 証憑番号のバリデーション（指定されている場合）
        if let Some(ref voucher) = request.voucher_number {
            use javelin_domain::financial_close::journal_entry::values::VoucherNumber;
            VoucherNumber::new(voucher.clone()).map_err(ApplicationError::DomainError)?;
        }

        // 4. 更新イベントを生成
        let user_id = UserId::new(request.user_id.clone());

        let event = JournalEntryEvent::DraftUpdated {
            entry_id: request.entry_id.clone(),
            transaction_date: request.transaction_date.clone(),
            voucher_number: request.voucher_number.clone(),
            lines: event_lines,
            updated_by: user_id.value().to_string(),
            updated_at: chrono::Utc::now(),
        };

        // 5. イベントストアへの保存
        self.event_repository
            .append_events(&request.entry_id, vec![event])
            .await
            .map_err(ApplicationError::DomainError)?;

        // 7. レスポンスを作成
        let response = UpdateDraftJournalEntryResponse {
            entry_id: request.entry_id,
            status: "Draft".to_string(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_update_draft_result(response).await;

        Ok(())
    }
}
