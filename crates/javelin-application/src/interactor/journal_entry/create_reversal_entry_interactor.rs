// CreateReversalEntryInteractor - 反対仕訳登録ユースケース実装
// 責務: 既存残高または期間帰属を反転させる反対仕訳を登録する

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
    dtos::{CreateReversalEntryRequest, RegisterJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::CreateReversalEntryUseCase,
    output_ports::JournalEntryOutputPort,
    query_service::JournalEntryFinderService,
};

pub struct CreateReversalEntryInteractor<
    R: JournalEntryRepository,
    O: JournalEntryOutputPort,
    F: JournalEntryFinderService,
> {
    journal_entry_repository: Arc<R>,
    output_port: Arc<O>,
    finder_service: Arc<F>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort, F: JournalEntryFinderService>
    CreateReversalEntryInteractor<R, O, F>
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
    CreateReversalEntryUseCase for CreateReversalEntryInteractor<R, O, F>
{
    async fn execute(&self, request: CreateReversalEntryRequest) -> ApplicationResult<()> {
        // 1. 参照元伝票の検索
        let reference_entry = self
            .finder_service
            .find_by_entry_number(&request.reference_entry_id)
            .await?
            .ok_or_else(|| {
                ApplicationError::ValidationFailed(vec![format!(
                    "参照元伝票が見つかりません: {}",
                    request.reference_entry_id
                )])
            })?;

        // 2. 参照元伝票を load() で取得
        let reference_journal = self
            .journal_entry_repository
            .load(&reference_entry.entry_id)
            .await
            .map_err(ApplicationError::DomainError)?
            .ok_or_else(|| {
                ApplicationError::ValidationFailed(vec![format!(
                    "参照元伝票が見つかりません: {}",
                    reference_entry.entry_id
                )])
            })?;

        // 3. 参照元伝票の明細を取得
        let reference_lines = reference_journal.lines();

        // TODO: ドメインサービスで反転処理を実装する必要がある
        // 現在は元の明細をそのまま使用（実際には借方/貸方を反転する必要がある）
        let lines: Vec<_> = reference_lines.iter().cloned().collect();

        // 4. 取引日付のパース
        let transaction_date = NaiveDate::parse_from_str(&request.transaction_date, "%Y-%m-%d")
            .map_err(|_| {
                ApplicationError::ValidationFailed(vec![format!(
                    "Invalid date format: {}",
                    request.transaction_date
                )])
            })?;
        let transaction_date =
            TransactionDate::new(transaction_date).map_err(ApplicationError::DomainError)?;

        // 5. 証憑番号の作成
        let voucher_number = VoucherNumber::new(request.voucher_number.clone())
            .map_err(ApplicationError::DomainError)?;

        // 6. ユーザーIDの作成
        let user_id = UserId::new(request.user_id.clone());

        // 7. 仕訳IDの生成
        let entry_id = JournalEntryId::new(uuid::Uuid::new_v4().to_string());

        // 8. 仕訳エンティティの作成（Draft状態） JournalEntry::new()
        //    内部でバランスチェックが行われる
        let journal_entry =
            JournalEntry::new(entry_id.clone(), transaction_date, voucher_number, lines, user_id)
                .map_err(ApplicationError::DomainError)?;

        // 9. Repository の save() で永続化
        self.journal_entry_repository
            .save(&journal_entry)
            .await
            .map_err(ApplicationError::DomainError)?;

        // 10. レスポンスDTOを作成してOutput Portへ送信
        let response = RegisterJournalEntryResponse {
            entry_id: entry_id.value().to_string(),
            status: journal_entry.status().as_str().to_string(),
        };
        self.output_port.present_register_result(response).await;

        Ok(())
    }
}
