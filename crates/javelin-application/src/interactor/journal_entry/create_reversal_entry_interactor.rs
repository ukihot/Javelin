// CreateReversalEntryInteractor - 反対仕訳登録ユースケース実装
// 責務: 既存残高または期間帰属を反転させる反対仕訳を登録する

use std::sync::Arc;

use chrono::NaiveDate;
use javelin_domain::{
    entity::EntityId,
    financial_close::journal_entry::{
        entities::{JournalEntry, JournalEntryId},
        services::JournalEntryDomainService,
        values::{TransactionDate, UserId, VoucherNumber},
    },
    repositories::JournalEntryRepository,
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
    event_repository: Arc<R>,
    output_port: Arc<O>,
    finder_service: Arc<F>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort, F: JournalEntryFinderService>
    CreateReversalEntryInteractor<R, O, F>
{
    pub fn new(event_repository: Arc<R>, output_port: Arc<O>, finder_service: Arc<F>) -> Self {
        Self { event_repository, output_port, finder_service }
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

        // 2. 参照元伝票のイベントストリームから明細を取得
        let reference_events = self
            .event_repository
            .get_events(&reference_entry.entry_id)
            .await
            .map_err(ApplicationError::DomainError)?;

        if reference_events.is_empty() {
            return Err(ApplicationError::ValidationFailed(vec![format!(
                "参照元伝票のイベントが見つかりません: {}",
                reference_entry.entry_id
            )]));
        }

        // 3. 参照元伝票の明細を取得（DraftCreatedイベントから）
        use javelin_domain::financial_close::journal_entry::events::JournalEntryEvent;

        let reference_lines = reference_events
            .iter()
            .find_map(|event_json| {
                let event: JournalEntryEvent = serde_json::from_value(event_json.clone()).ok()?;
                match event {
                    JournalEntryEvent::DraftCreated { lines, .. } => Some(lines),
                    _ => None,
                }
            })
            .ok_or_else(|| {
                ApplicationError::ValidationFailed(vec![
                    "参照元伝票の明細が見つかりません".to_string(),
                ])
            })?;

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

        // 7. 参照元伝票の明細を反転させた反対仕訳明細を作成（ドメインサービスを使用）
        // まず参照元の明細をアプリケーション層DTOに変換してからドメインオブジェクトに変換
        use crate::dtos::JournalEntryLineDto;
        let app_lines: Result<Vec<JournalEntryLineDto>, _> =
            reference_lines.iter().map(|dto| dto.try_into()).collect();
        let app_lines = app_lines?;
        let reference_domain_lines: Result<Vec<_>, _> =
            app_lines.iter().map(|dto| dto.try_into()).collect();
        let reference_domain_lines = reference_domain_lines?;

        // ドメインサービスで反転処理
        let lines = JournalEntryDomainService::create_reversal_lines(&reference_domain_lines)
            .map_err(ApplicationError::DomainError)?;

        // 8. 借貸バランスチェック
        JournalEntryDomainService::validate_balance(&lines)
            .map_err(ApplicationError::DomainError)?;

        // 9. 仕訳IDの生成
        let entry_id = JournalEntryId::new(uuid::Uuid::new_v4().to_string());

        // 10. 仕訳エンティティの作成（Draft状態）
        let journal_entry =
            JournalEntry::new(entry_id.clone(), transaction_date, voucher_number, lines, user_id)
                .map_err(ApplicationError::DomainError)?;

        // 11. イベントの取得
        let events = journal_entry.events();

        // 12. イベントストアへの保存
        self.event_repository
            .append_events(entry_id.value(), events.to_vec())
            .await
            .map_err(ApplicationError::DomainError)?;

        // 13. レスポンスDTOを作成してOutput Portへ送信
        let response = RegisterJournalEntryResponse {
            entry_id: entry_id.value().to_string(),
            status: journal_entry.status().as_str().to_string(),
        };
        self.output_port.present_register_result(response).await;

        Ok(())
    }
}
