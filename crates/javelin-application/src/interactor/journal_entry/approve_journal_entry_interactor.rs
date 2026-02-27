// ApproveJournalEntryInteractor - 承認ユースケース実装

use std::sync::Arc;

use javelin_domain::{
    financial_close::journal_entry::{
        events::JournalEntryEvent,
        values::{EntryNumber, UserId},
    },
    repositories::JournalEntryRepository,
};

use crate::{
    dtos::{ApproveJournalEntryRequest, ApproveJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::ApproveJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
};

pub struct ApproveJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    event_repository: Arc<R>,
    output_port: Arc<O>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> ApproveJournalEntryInteractor<R, O> {
    pub fn new(event_repository: Arc<R>, output_port: Arc<O>) -> Self {
        Self { event_repository, output_port }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort> ApproveJournalEntryUseCase
    for ApproveJournalEntryInteractor<R, O>
{
    async fn execute(&self, request: ApproveJournalEntryRequest) -> ApplicationResult<()> {
        // 1. イベントストリームから仕訳エンティティを再構築
        let events = self
            .event_repository
            .get_events(&request.entry_id)
            .await
            .map_err(ApplicationError::DomainError)?;

        if events.is_empty() {
            return Err(ApplicationError::ValidationFailed(vec![format!(
                "Journal entry not found: {}",
                request.entry_id
            )]));
        }

        // 2. イベントをJournalEntryEventに変換
        // モダンプラクティス: イベント数で初期キャパシティを確保
        let mut journal_events = Vec::with_capacity(events.len());
        for event_json in events {
            let event: JournalEntryEvent = serde_json::from_value(event_json).map_err(|e| {
                ApplicationError::ValidationFailed(vec![format!(
                    "Failed to deserialize event: {}",
                    e
                )])
            })?;
            journal_events.push(event);
        }

        // 3. 最初のイベント（DraftCreated）から仕訳エンティティを作成
        let first_event = journal_events.first().ok_or_else(|| {
            ApplicationError::ValidationFailed(vec!["No events found".to_string()])
        })?;

        let mut journal_entry = match first_event {
            JournalEntryEvent::DraftCreated {
                entry_id,
                transaction_date,
                voucher_number,
                lines,
                created_by,
                ..
            } => {
                use chrono::NaiveDate;
                use javelin_domain::financial_close::journal_entry::{
                    entities::{JournalEntry, JournalEntryId},
                    values::{TransactionDate, UserId, VoucherNumber},
                };

                let entry_id = JournalEntryId::new(entry_id.clone());
                let transaction_date = NaiveDate::parse_from_str(transaction_date, "%Y-%m-%d")
                    .map_err(|e| {
                        ApplicationError::ValidationFailed(vec![format!(
                            "Invalid transaction date: {}",
                            e
                        )])
                    })?;
                let transaction_date = TransactionDate::new(transaction_date)
                    .map_err(ApplicationError::DomainError)?;
                let voucher_number = VoucherNumber::new(voucher_number.clone())
                    .map_err(ApplicationError::DomainError)?;
                let user_id = UserId::new(created_by.clone());

                // 明細を作成
                use crate::dtos::JournalEntryLineDto as AppLineDto;
                let app_lines: Result<Vec<AppLineDto>, _> =
                    lines.iter().map(|dto| dto.try_into()).collect();
                let app_lines = app_lines?;
                let entry_lines: Result<Vec<_>, _> =
                    app_lines.iter().map(|dto| dto.try_into()).collect();
                let entry_lines = entry_lines?;

                JournalEntry::new(entry_id, transaction_date, voucher_number, entry_lines, user_id)
                    .map_err(ApplicationError::DomainError)?
            }
            _ => {
                return Err(ApplicationError::ValidationFailed(vec![
                    "First event must be DraftCreated".to_string(),
                ]));
            }
        };

        // 4. 残りのイベントを適用してエンティティの状態を復元
        for event in journal_events.iter().skip(1) {
            match event {
                JournalEntryEvent::DraftUpdated { .. } => {
                    // DraftUpdatedは状態に影響しないのでスキップ
                    // （実際の実装では明細の更新も処理する必要がある）
                }
                JournalEntryEvent::ApprovalRequested { requested_by, .. } => {
                    let user_id = UserId::new(requested_by.clone());
                    journal_entry
                        .submit_for_approval(user_id)
                        .map_err(ApplicationError::DomainError)?;
                }
                JournalEntryEvent::Rejected { rejected_by, reason, .. } => {
                    let user_id = UserId::new(rejected_by.clone());
                    journal_entry
                        .reject(user_id, reason.clone())
                        .map_err(ApplicationError::DomainError)?;
                }
                _ => {
                    // その他のイベントは承認前には発生しないはず
                }
            }
        }

        // 5. 伝票番号を生成
        let entry_number =
            EntryNumber::new(format!("EN-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")))
                .map_err(ApplicationError::DomainError)?;

        // 6. 承認処理を実行
        let user_id = UserId::new(request.approver_id.clone());
        journal_entry
            .approve(entry_number.clone(), user_id)
            .map_err(ApplicationError::DomainError)?;

        // 7. 新しいイベントを取得
        let new_events = journal_entry.drain_events();

        // 8. イベントストアへの保存
        self.event_repository
            .append_events(&request.entry_id, new_events)
            .await
            .map_err(ApplicationError::DomainError)?;

        // 9. レスポンスを作成
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
