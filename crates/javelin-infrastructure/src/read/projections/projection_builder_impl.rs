// ProjectionBuilder具象実装 - Infrastructure層
// Application層のProjectionBuilderトレイトを実装

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use javelin_application::{
    error::{ApplicationError, ApplicationResult},
    projection_builder::ProjectionBuilder as ProjectionBuilderTrait,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{event_store::EventStore, event_stream::StoredEvent, projection_db::ProjectionDb};

/// 再試行キューエントリ
#[derive(Debug, Clone)]
struct RetryQueueEntry {
    event: StoredEvent,
    retry_count: u32,
    last_error: String,
}

/// ProjectionBuilder具象実装
///
/// イベントストリームからRead Modelを構築する。
/// EventStoreから取得したイベントを順次処理し、ProjectionDBを更新する。
///
/// 要件: 2.1, 2.2
pub struct ProjectionBuilderImpl {
    projection_db: Arc<ProjectionDb>,
    event_store: Arc<EventStore>,
    /// 再試行キュー（要件7.4）
    retry_queue: Arc<Mutex<VecDeque<RetryQueueEntry>>>,
    /// インフラエラー通知チャネル
    error_sender: Arc<Mutex<Option<mpsc::UnboundedSender<String>>>>,
}

impl ProjectionBuilderImpl {
    /// 新しいProjectionBuilderImplを作成
    ///
    /// # Arguments
    /// * `projection_db` - ProjectionDBへの参照
    /// * `event_store` - EventStoreへの参照
    pub fn new(projection_db: Arc<ProjectionDb>, event_store: Arc<EventStore>) -> Self {
        Self {
            projection_db,
            event_store,
            retry_queue: Arc::new(Mutex::new(VecDeque::new())),
            error_sender: Arc::new(Mutex::new(None)),
        }
    }

    /// 単一イベントからProjectionを更新（内部実装）
    ///
    /// イベント種別に応じて適切なProjection更新メソッドを呼び出す。
    ///
    /// # Arguments
    /// * `event` - 処理するイベント
    async fn process_event_internal(&self, event: &StoredEvent) -> ApplicationResult<()> {
        // イベント種別に応じて適切なProjection更新メソッドを呼び出す
        match event.event_type.as_str() {
            "DraftCreated"
            | "SubmittedForApproval"
            | "Approved"
            | "Rejected"
            | "Updated"
            | "Deleted"
            | "Corrected"
            | "Reversed" => {
                // 仕訳一覧Projectionを更新（Task 4.1で実装）
                self.update_journal_entry_list_projection(event).await?;
            }
            _ => {
                // 未知のイベント種別はログに記録して無視
                // 本番環境ではログ出力を追加すべき
            }
        }

        Ok(())
    }

    /// 仕訳一覧Projectionを更新
    ///
    /// Task 4.1で実装
    ///
    /// イベント種別に応じて仕訳一覧Projectionを更新：
    /// - DraftCreated: 新規エントリを追加
    /// - Approved: ステータスを更新
    /// - Deleted: エントリを削除
    ///
    /// 要件: 2.3, 2.4, 2.5
    async fn update_journal_entry_list_projection(
        &self,
        event: &StoredEvent,
    ) -> ApplicationResult<()> {
        use serde_json::Value;

        // イベントペイロードをデシリアライズ
        let event_data: Value = serde_json::from_slice(&event.payload)
            .map_err(|e| ApplicationError::ValidationFailed(vec![e.to_string()]))?;

        let entry_id = event.aggregate_id.clone();
        let key = format!("journal_entry:{}", entry_id);

        match event.event_type.as_str() {
            "DraftCreated" => {
                // 新規エントリを追加
                let stored_entry = StoredJournalEntry {
                    entry_id: entry_id.clone(),
                    entry_number: None,
                    status: "Draft".to_string(),
                    transaction_date: event_data["transaction_date"]
                        .as_str()
                        .unwrap_or("2024-01-01")
                        .to_string(),
                    voucher_number: event_data["voucher_number"].as_str().unwrap_or("").to_string(),
                    description: event_data["description"].as_str().unwrap_or("").to_string(),
                    total_debit: event_data["total_debit"].as_f64().unwrap_or(0.0),
                    total_credit: event_data["total_credit"].as_f64().unwrap_or(0.0),
                    created_by: event_data["created_by"].as_str().unwrap_or("").to_string(),
                    created_at: event.timestamp.clone(),
                    updated_by: None,
                    updated_at: None,
                    approved_by: None,
                    approved_at: None,
                    lines: event_data["lines"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .map(|line| StoredJournalEntryLine {
                                    line_number: line["line_number"].as_u64().unwrap_or(0) as u32,
                                    side: line["side"].as_str().unwrap_or("").to_string(),
                                    account_code: line["account_code"]
                                        .as_str()
                                        .unwrap_or("")
                                        .to_string(),
                                    account_name: line["account_name"]
                                        .as_str()
                                        .unwrap_or("")
                                        .to_string(),
                                    sub_account_code: line["sub_account_code"]
                                        .as_str()
                                        .map(|s| s.to_string()),
                                    department_code: line["department_code"]
                                        .as_str()
                                        .map(|s| s.to_string()),
                                    amount: line["amount"].as_f64().unwrap_or(0.0),
                                    currency: line["currency"]
                                        .as_str()
                                        .unwrap_or("JPY")
                                        .to_string(),
                                    tax_type: line["tax_type"].as_str().unwrap_or("").to_string(),
                                    tax_amount: line["tax_amount"].as_f64().unwrap_or(0.0),
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                };

                let data = serde_json::to_vec(&stored_entry)
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                self.projection_db
                    .update_projection(&key, &data, event.global_sequence)
                    .await
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;
            }
            "SubmittedForApproval" => {
                // ステータスを更新
                if let Some(existing_data) = self
                    .projection_db
                    .get_projection(&key)
                    .await
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
                {
                    let mut stored_entry: StoredJournalEntry =
                        serde_json::from_slice(&existing_data).map_err(|e| {
                            ApplicationError::ProjectionDatabaseError(e.to_string())
                        })?;

                    stored_entry.status = "PendingApproval".to_string();

                    let data = serde_json::to_vec(&stored_entry)
                        .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                    self.projection_db
                        .update_projection(&key, &data, event.global_sequence)
                        .await
                        .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;
                }
            }
            "Approved" => {
                // ステータスを更新
                if let Some(existing_data) = self
                    .projection_db
                    .get_projection(&key)
                    .await
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
                {
                    let mut stored_entry: StoredJournalEntry =
                        serde_json::from_slice(&existing_data).map_err(|e| {
                            ApplicationError::ProjectionDatabaseError(e.to_string())
                        })?;

                    stored_entry.status = "Approved".to_string();
                    stored_entry.approved_by =
                        event_data["approved_by"].as_str().map(|s| s.to_string());
                    stored_entry.approved_at = Some(event.timestamp.clone());
                    stored_entry.entry_number =
                        event_data["entry_number"].as_str().map(|s| s.to_string());

                    let data = serde_json::to_vec(&stored_entry)
                        .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                    self.projection_db
                        .update_projection(&key, &data, event.global_sequence)
                        .await
                        .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                    // 元帳Projectionも更新
                    self.update_ledger_projection(event).await?;
                }
            }
            "Rejected" => {
                // ステータスを更新
                if let Some(existing_data) = self
                    .projection_db
                    .get_projection(&key)
                    .await
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
                {
                    let mut stored_entry: StoredJournalEntry =
                        serde_json::from_slice(&existing_data).map_err(|e| {
                            ApplicationError::ProjectionDatabaseError(e.to_string())
                        })?;

                    stored_entry.status = "Rejected".to_string();

                    let data = serde_json::to_vec(&stored_entry)
                        .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                    self.projection_db
                        .update_projection(&key, &data, event.global_sequence)
                        .await
                        .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;
                }
            }
            "Updated" => {
                // エントリを更新
                if let Some(existing_data) = self
                    .projection_db
                    .get_projection(&key)
                    .await
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
                {
                    let mut stored_entry: StoredJournalEntry =
                        serde_json::from_slice(&existing_data).map_err(|e| {
                            ApplicationError::ProjectionDatabaseError(e.to_string())
                        })?;

                    stored_entry.transaction_date = event_data["transaction_date"]
                        .as_str()
                        .unwrap_or(&stored_entry.transaction_date)
                        .to_string();
                    stored_entry.voucher_number = event_data["voucher_number"]
                        .as_str()
                        .unwrap_or(&stored_entry.voucher_number)
                        .to_string();
                    stored_entry.description = event_data["description"]
                        .as_str()
                        .unwrap_or(&stored_entry.description)
                        .to_string();
                    stored_entry.updated_by =
                        event_data["updated_by"].as_str().map(|s| s.to_string());
                    stored_entry.updated_at = Some(event.timestamp.clone());

                    let data = serde_json::to_vec(&stored_entry)
                        .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                    self.projection_db
                        .update_projection(&key, &data, event.global_sequence)
                        .await
                        .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;
                }
            }
            "Deleted" => {
                // エントリを削除
                self.projection_db
                    .delete_projection(&key)
                    .await
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;
            }
            "Corrected" | "Reversed" => {
                // 訂正・取消の場合は新しいエントリとして扱う（元のエントリは残す）
                // 実装は必要に応じて追加
            }
            _ => {
                // 未知のイベント種別は無視
            }
        }

        Ok(())
    }

    /// 元帳Projectionを更新
    ///
    /// Task 4.2で実装
    ///
    /// Approvedイベント時に元帳に転記し、勘定科目別の残高を更新する。
    ///
    /// 要件: 2.6
    async fn update_ledger_projection(&self, event: &StoredEvent) -> ApplicationResult<()> {
        use serde_json::Value;

        // イベントペイロードをデシリアライズ
        let event_data: Value = serde_json::from_slice(&event.payload)
            .map_err(|e| ApplicationError::ValidationFailed(vec![e.to_string()]))?;

        // 仕訳明細から勘定科目ごとに元帳を更新
        if let Some(lines) = event_data["lines"].as_array() {
            for line in lines {
                let account_code = line["account_code"].as_str().unwrap_or("");
                let account_name = line["account_name"].as_str().unwrap_or("");
                let side = line["side"].as_str().unwrap_or("");
                let amount = line["amount"].as_f64().unwrap_or(0.0);

                // 取引日から年月を抽出
                let transaction_date =
                    event_data["transaction_date"].as_str().unwrap_or("2024-01-01");
                let parts: Vec<&str> = transaction_date.split('-').collect();
                let year = parts.first().and_then(|s| s.parse::<u32>().ok()).unwrap_or(2024);
                let month = parts.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(1);

                let ledger_key = format!("ledger:{}:{}:{}", account_code, year, month);

                // 既存の元帳データを取得
                let mut ledger_data = if let Some(data) = self
                    .projection_db
                    .get_projection(&ledger_key)
                    .await
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
                {
                    serde_json::from_slice::<StoredLedgerData>(&data)
                        .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
                } else {
                    StoredLedgerData {
                        account_name: account_name.to_string(),
                        opening_balance: 0.0,
                        entries: vec![],
                    }
                };

                // 新しいエントリを追加
                use javelin_domain::financial_close::journal_entry::values::DebitCredit;
                let side_enum = side.parse::<DebitCredit>().ok();
                let debit_amount = if matches!(side_enum, Some(DebitCredit::Debit)) {
                    amount
                } else {
                    0.0
                };
                let credit_amount = if matches!(side_enum, Some(DebitCredit::Credit)) {
                    amount
                } else {
                    0.0
                };

                ledger_data.entries.push(StoredLedgerEntry {
                    transaction_date: transaction_date.to_string(),
                    entry_number: event_data["entry_number"].as_str().unwrap_or("").to_string(),
                    entry_id: event.aggregate_id.clone(),
                    voucher_number: event_data["voucher_number"].as_str().unwrap_or("").to_string(),
                    description: event_data["description"].as_str().unwrap_or("").to_string(),
                    debit_amount,
                    credit_amount,
                    balance: 0.0, // 残高は照会時に計算
                });

                // 元帳データを保存
                let data = serde_json::to_vec(&ledger_data)
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                self.projection_db
                    .update_projection(&ledger_key, &data, event.global_sequence)
                    .await
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;
            }

            // 試算表Projectionも更新
            self.update_trial_balance_projection(event).await?;
        }

        Ok(())
    }

    /// 試算表Projectionを更新
    ///
    /// Task 4.3で実装
    ///
    /// 元帳Projectionから試算表を生成し、借貸合計を計算する。
    ///
    /// 要件: 2.7
    async fn update_trial_balance_projection(&self, event: &StoredEvent) -> ApplicationResult<()> {
        use serde_json::Value;

        // イベントペイロードをデシリアライズ
        let event_data: Value = serde_json::from_slice(&event.payload)
            .map_err(|e| ApplicationError::ValidationFailed(vec![e.to_string()]))?;

        // 取引日から年月を抽出
        let transaction_date = event_data["transaction_date"].as_str().unwrap_or("2024-01-01");
        let parts: Vec<&str> = transaction_date.split('-').collect();
        let year = parts.first().and_then(|s| s.parse::<u32>().ok()).unwrap_or(2024);
        let month = parts.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(1);

        let trial_balance_key = format!("trial_balance:{}:{}", year, month);

        // 既存の試算表データを取得
        let mut trial_balance_data = if let Some(data) = self
            .projection_db
            .get_projection(&trial_balance_key)
            .await
            .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
        {
            serde_json::from_slice::<StoredTrialBalanceData>(&data)
                .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
        } else {
            StoredTrialBalanceData { entries: vec![] }
        };

        // 仕訳明細から勘定科目ごとに集計
        if let Some(lines) = event_data["lines"].as_array() {
            for line in lines {
                let account_code = line["account_code"].as_str().unwrap_or("");
                let account_name = line["account_name"].as_str().unwrap_or("");
                let side = line["side"].as_str().unwrap_or("");
                let amount = line["amount"].as_f64().unwrap_or(0.0);

                use javelin_domain::financial_close::journal_entry::values::DebitCredit;
                let side_enum = side.parse::<DebitCredit>().ok();
                let debit_amount = if matches!(side_enum, Some(DebitCredit::Debit)) {
                    amount
                } else {
                    0.0
                };
                let credit_amount = if matches!(side_enum, Some(DebitCredit::Credit)) {
                    amount
                } else {
                    0.0
                };

                // 既存のエントリを検索
                if let Some(entry) =
                    trial_balance_data.entries.iter_mut().find(|e| e.account_code == account_code)
                {
                    // 既存エントリを更新
                    entry.debit_amount += debit_amount;
                    entry.credit_amount += credit_amount;
                } else {
                    // 新規エントリを追加
                    trial_balance_data.entries.push(StoredTrialBalanceEntry {
                        account_code: account_code.to_string(),
                        account_name: account_name.to_string(),
                        debit_amount,
                        credit_amount,
                    });
                }
            }
        }

        // 試算表データを保存
        let data = serde_json::to_vec(&trial_balance_data)
            .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

        self.projection_db
            .update_projection(&trial_balance_key, &data, event.global_sequence)
            .await
            .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

        Ok(())
    }

    /// イベント通知ハンドラを作成
    ///
    /// EventStoreに登録するコールバックを作成する。
    /// イベント保存時に自動的にこのハンドラが呼び出され、
    /// Projectionが更新される。
    ///
    /// # Arguments
    /// * `error_sender` - インフラエラー通知用チャネル
    ///
    /// # Returns
    /// イベント通知コールバック
    ///
    /// # Requirements
    /// 要件: 7.2
    pub fn create_event_notification_handler(
        self: Arc<Self>,
        error_sender: mpsc::UnboundedSender<String>,
    ) -> crate::event_store::EventNotificationCallback {
        // エラーチャネルを保存
        *self.error_sender.lock().unwrap() = Some(error_sender.clone());

        Arc::new(move |event| {
            let builder = Arc::clone(&self);
            let error_sender = error_sender.clone();
            Box::pin(async move {
                if let Err(e) = builder.process_event_internal(&event).await {
                    // エラーメッセージを作成
                    let error_message = format!(
                        "Projection更新エラー [seq={}, agg={}]: {:?}",
                        event.global_sequence, event.aggregate_id, e
                    );

                    // エラーチャネルに送信（UIのイベントログに表示される）
                    let _ = error_sender.send(error_message);

                    // 再試行キューへの追加（要件7.4）
                    builder.add_to_retry_queue(event, e.to_string());
                }
            })
        })
    }

    /// 再試行キューにイベントを追加
    ///
    /// Projection更新に失敗したイベントを再試行キューに追加する。
    ///
    /// # Arguments
    /// * `event` - 失敗したイベント
    /// * `error` - エラーメッセージ
    ///
    /// # Requirements
    /// 要件: 7.4
    fn add_to_retry_queue(&self, event: StoredEvent, error: String) {
        let mut queue = self.retry_queue.lock().unwrap();
        queue.push_back(RetryQueueEntry { event, retry_count: 0, last_error: error });
    }

    /// 再試行キューを処理
    ///
    /// 再試行キューに溜まったイベントを再処理する。
    /// 最大3回まで再試行し、それでも失敗した場合はログに記録する。
    ///
    /// # Requirements
    /// 要件: 7.4
    pub async fn process_retry_queue(&self) -> ApplicationResult<()> {
        const MAX_RETRIES: u32 = 3;

        loop {
            let entry = {
                let mut queue = self.retry_queue.lock().unwrap();
                queue.pop_front()
            };

            match entry {
                Some(mut entry) => {
                    entry.retry_count += 1;

                    match self.process_event_internal(&entry.event).await {
                        Ok(_) => {
                            // 成功 - イベントログに通知
                            let success_message = format!(
                                "Projection更新リトライ成功 [seq={}, retry={}]",
                                entry.event.global_sequence, entry.retry_count
                            );
                            if let Some(sender) = self.error_sender.lock().unwrap().as_ref() {
                                let _ = sender.send(success_message);
                            }
                        }
                        Err(e) => {
                            if entry.retry_count >= MAX_RETRIES {
                                // 最大リトライ回数に達した - イベントログに通知
                                let error_message = format!(
                                    "Projection更新リトライ失敗（最大回数到達） [seq={}, retry={}]: {:?}",
                                    entry.event.global_sequence, entry.retry_count, e
                                );
                                if let Some(sender) = self.error_sender.lock().unwrap().as_ref() {
                                    let _ = sender.send(error_message);
                                }
                            } else {
                                // 再度キューに追加
                                entry.last_error = e.to_string();
                                let mut queue = self.retry_queue.lock().unwrap();
                                queue.push_back(entry);
                            }
                        }
                    }
                }
                None => break, // キューが空
            }
        }

        Ok(())
    }

    /// 再試行キューのサイズを取得
    pub fn retry_queue_size(&self) -> usize {
        self.retry_queue.lock().unwrap().len()
    }
}

#[async_trait::async_trait]
impl ProjectionBuilderTrait for ProjectionBuilderImpl {
    async fn rebuild_all_projections(&self) -> ApplicationResult<()> {
        // EventStoreから全イベントを取得（シーケンス0から）
        let events = self.event_store.get_all_events(0).await.map_err(|e| {
            ApplicationError::EventStoreError(format!("Failed to get events: {}", e))
        })?;

        // 各イベントを順次処理
        for event in events.iter() {
            self.process_event_internal(event).await?;
        }

        // チェックポイントを更新
        if let Some(last_event) = events.last() {
            self.projection_db
                .update_projection_batch(
                    "main",
                    1,
                    vec![], // 空の更新（チェックポイントのみ更新）
                    last_event.global_sequence,
                )
                .await
                .map_err(|e| {
                    ApplicationError::ProjectionDatabaseError(format!(
                        "Failed to update checkpoint: {}",
                        e
                    ))
                })?;
        }

        Ok(())
    }

    async fn process_event(&self, event_data: &[u8]) -> ApplicationResult<()> {
        // イベントデータをデシリアライズ
        let event: StoredEvent = serde_json::from_slice(event_data)
            .map_err(|e| ApplicationError::ValidationFailed(vec![e.to_string()]))?;

        self.process_event_internal(&event).await
    }
}

/// ProjectionDBに保存される仕訳エントリデータ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredJournalEntry {
    entry_id: String,
    entry_number: Option<String>,
    status: String,
    transaction_date: String,
    voucher_number: String,
    description: String,
    total_debit: f64,
    total_credit: f64,
    created_by: String,
    created_at: String,
    updated_by: Option<String>,
    updated_at: Option<String>,
    approved_by: Option<String>,
    approved_at: Option<String>,
    lines: Vec<StoredJournalEntryLine>,
}

/// ProjectionDBに保存される仕訳明細データ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredJournalEntryLine {
    line_number: u32,
    side: String,
    account_code: String,
    account_name: String,
    sub_account_code: Option<String>,
    department_code: Option<String>,
    amount: f64,
    currency: String,
    tax_type: String,
    tax_amount: f64,
}

/// ProjectionDBに保存される元帳データ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredLedgerData {
    account_name: String,
    opening_balance: f64,
    entries: Vec<StoredLedgerEntry>,
}

/// ProjectionDBに保存される元帳エントリデータ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredLedgerEntry {
    transaction_date: String,
    entry_number: String,
    entry_id: String,
    voucher_number: String,
    description: String,
    debit_amount: f64,
    credit_amount: f64,
    balance: f64,
}

/// ProjectionDBに保存される試算表データ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredTrialBalanceData {
    entries: Vec<StoredTrialBalanceEntry>,
}

/// ProjectionDBに保存される試算表エントリデータ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredTrialBalanceEntry {
    account_code: String,
    account_name: String,
    debit_amount: f64,
    credit_amount: f64,
}
