// RegisterJournalEntryInteractor - 仕訳登録ユースケース実装
// 責務: 仕訳登録のビジネスロジック実行

use std::sync::Arc;

use chrono::{Datelike, NaiveDate};
use javelin_domain::{
    entity::EntityId,
    financial_close::journal_entry::{
        entities::{JournalEntry, JournalEntryId},
        services::{JournalEntryDomainService, VoucherNumberDomainService},
        values::{TransactionDate, UserId, VoucherNumber},
    },
    repositories::JournalEntryRepository,
};

use crate::{
    dtos::{RegisterJournalEntryRequest, RegisterJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::RegisterJournalEntryUseCase,
    output_ports::JournalEntryOutputPort,
    query_service::JournalEntrySearchQueryService,
};

pub struct RegisterJournalEntryInteractor<
    R: JournalEntryRepository,
    O: JournalEntryOutputPort,
    Q: JournalEntrySearchQueryService,
> {
    event_repository: Arc<R>,
    output_port: Arc<O>,
    search_query_service: Arc<Q>,
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort, Q: JournalEntrySearchQueryService>
    RegisterJournalEntryInteractor<R, O, Q>
{
    pub fn new(
        event_repository: Arc<R>,
        output_port: Arc<O>,
        search_query_service: Arc<Q>,
    ) -> Self {
        Self { event_repository, output_port, search_query_service }
    }
}

impl<R: JournalEntryRepository, O: JournalEntryOutputPort, Q: JournalEntrySearchQueryService>
    RegisterJournalEntryUseCase for RegisterJournalEntryInteractor<R, O, Q>
{
    async fn execute(&self, request: RegisterJournalEntryRequest) -> ApplicationResult<()> {
        // 1. 入力バリデーション - 取引日付のパース
        // YYYYMMDD形式とYYYY-MM-DD形式の両方に対応
        let date_str = &request.transaction_date;
        let transaction_date = if date_str.contains('-') {
            // YYYY-MM-DD形式
            match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                Ok(date) => date,
                Err(e) => {
                    let error_msg = format!("日付形式が不正です: {} (エラー: {})", date_str, e);
                    self.output_port.notify_error(error_msg.clone()).await;
                    return Err(ApplicationError::ValidationFailed(vec![error_msg]));
                }
            }
        } else if date_str.len() == 8 {
            // YYYYMMDD形式
            match NaiveDate::parse_from_str(date_str, "%Y%m%d") {
                Ok(date) => date,
                Err(e) => {
                    let error_msg = format!("日付形式が不正です: {} (エラー: {})", date_str, e);
                    self.output_port.notify_error(error_msg.clone()).await;
                    return Err(ApplicationError::ValidationFailed(vec![error_msg]));
                }
            }
        } else {
            let error_msg = format!(
                "日付形式が不正です: {} (YYYY-MM-DD または YYYYMMDD 形式で入力してください)",
                date_str
            );
            self.output_port.notify_error(error_msg.clone()).await;
            return Err(ApplicationError::ValidationFailed(vec![error_msg]));
        };

        let transaction_date = match TransactionDate::new(transaction_date) {
            Ok(date) => date,
            Err(e) => {
                let error_msg = format!("取引日付が無効です: {}", e);
                self.output_port.notify_error(error_msg.clone()).await;
                return Err(ApplicationError::DomainError(e));
            }
        };

        // 進捗通知: 入力検証完了
        self.output_port.notify_progress("入力データを検証しました".to_string()).await;

        // 2. 証憑番号の作成（空の場合は自動生成）
        let voucher_number_str = if request.voucher_number.is_empty() {
            // 取引日付から年度を取得（簡易的に年を使用）
            let fiscal_year = transaction_date.value().year() as u32;

            // QueryServiceを使って既存の伝票番号リストを取得
            let existing_voucher_numbers = self
                .search_query_service
                .get_voucher_numbers_by_fiscal_year(fiscal_year)
                .await
                .map_err(|e| {
                    let error_msg = format!("既存伝票番号の取得に失敗しました: {}", e);
                    ApplicationError::QueryExecutionFailed(error_msg)
                })?;

            // ドメインサービスを使って次の伝票番号を生成
            match VoucherNumberDomainService::generate_next(fiscal_year, &existing_voucher_numbers)
            {
                Ok(vn) => {
                    // 進捗通知: 伝票番号採番完了
                    self.output_port
                        .notify_progress(format!("伝票番号を採番しました: {}", vn))
                        .await;
                    vn
                }
                Err(e) => {
                    let error_msg = format!("伝票番号の採番に失敗しました: {}", e);
                    self.output_port.notify_error(error_msg.clone()).await;
                    return Err(ApplicationError::DomainError(e));
                }
            }
        } else {
            request.voucher_number.clone()
        };

        let voucher_number = match VoucherNumber::new(voucher_number_str) {
            Ok(vn) => vn,
            Err(e) => {
                let error_msg = format!("伝票番号が無効です: {}", e);
                self.output_port.notify_error(error_msg.clone()).await;
                return Err(ApplicationError::DomainError(e));
            }
        };

        // 3. ユーザーIDの作成
        let user_id = UserId::new(request.user_id.clone());

        // 4. 仕訳明細の作成
        let lines: Result<Vec<_>, _> = request.lines.iter().map(|dto| dto.try_into()).collect();
        let lines = match lines {
            Ok(l) => l,
            Err(e) => {
                let error_msg = format!("仕訳明細の作成に失敗しました: {}", e);
                self.output_port.notify_error(error_msg.clone()).await;
                return Err(e);
            }
        };

        // 進捗通知: 仕訳明細作成完了
        self.output_port.notify_progress("仕訳明細を作成しました".to_string()).await;

        // 5. 借貸バランスチェック
        if let Err(e) = JournalEntryDomainService::validate_balance(&lines) {
            let error_msg = format!("借貸バランスが一致しません: {}", e);
            self.output_port.notify_error(error_msg.clone()).await;
            return Err(ApplicationError::DomainError(e));
        }

        // 進捗通知: 借貸バランス検証完了
        self.output_port.notify_progress("借貸バランスを検証しました".to_string()).await;

        // 6. 仕訳IDの生成（UUIDを使用）
        let entry_id = JournalEntryId::new(uuid::Uuid::new_v4().to_string());

        // 7. 仕訳エンティティの作成（Draft状態）
        let journal_entry = match JournalEntry::new(
            entry_id.clone(),
            transaction_date,
            voucher_number,
            lines,
            user_id,
        ) {
            Ok(je) => je,
            Err(e) => {
                let error_msg = format!("仕訳エンティティの作成に失敗しました: {}", e);
                self.output_port.notify_error(error_msg.clone()).await;
                return Err(ApplicationError::DomainError(e));
            }
        };

        // 進捗通知: 仕訳エンティティ作成完了
        self.output_port
            .notify_progress("仕訳エンティティを作成しました".to_string())
            .await;

        // 8. イベントの取得（DraftCreatedイベントが含まれる）
        let events = journal_entry.events();

        // 9. イベントストアへの保存
        if let Err(e) = self.event_repository.append_events(entry_id.value(), events.to_vec()).await
        {
            let error_msg = format!("イベントストアへの保存に失敗しました: {}", e);
            self.output_port.notify_error(error_msg.clone()).await;
            return Err(ApplicationError::DomainError(e));
        }

        // 進捗通知: イベントストア保存完了
        self.output_port
            .notify_progress("イベントストアへ保存しました".to_string())
            .await;

        // 10. レスポンスDTOを作成してOutput Portへ送信
        let response = RegisterJournalEntryResponse {
            entry_id: entry_id.value().to_string(),
            status: journal_entry.status().as_str().to_string(),
        };
        self.output_port.present_register_result(response).await;

        Ok(())
    }
}
