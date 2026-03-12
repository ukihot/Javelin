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
use tokio::sync::mpsc;

use crate::{
    event_store::EventStore,
    event_stream::StoredEvent,
    read::{
        infrastructure::db::ProjectionDb,
        projectors::{
            AccountMasterProjector, JournalEntryProjector, LedgerProjector, Projector,
            ProjectorRegistry, TrialBalanceProjector,
        },
    },
};

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
pub struct ProjectionBuilderImpl<J, A, L, T>
where
    J: Projector + 'static,
    A: Projector + 'static,
    L: Projector + 'static,
    T: Projector + 'static,
{
    projection_db: Arc<ProjectionDb>,
    event_store: Arc<EventStore>,
    /// Projectorレジストリ（静的ディスパッチ）
    projector_registry: Arc<ProjectorRegistry<J, A, L, T>>,
    /// 再試行キュー（要件7.4）
    retry_queue: Arc<Mutex<VecDeque<RetryQueueEntry>>>,
    /// インフラエラー通知チャネル
    error_sender: Arc<Mutex<Option<mpsc::UnboundedSender<String>>>>,
}

impl
    ProjectionBuilderImpl<
        JournalEntryProjector,
        AccountMasterProjector,
        LedgerProjector,
        TrialBalanceProjector,
    >
{
    /// 新しいProjectionBuilderImplを作成
    ///
    /// # Arguments
    /// * `projection_db` - ProjectionDBへの参照
    /// * `event_store` - EventStoreへの参照
    pub fn new(projection_db: Arc<ProjectionDb>, event_store: Arc<EventStore>) -> Self {
        // Projectorを作成
        let journal_entry_projector =
            Arc::new(JournalEntryProjector::new(Arc::clone(&projection_db)));
        let account_master_projector =
            Arc::new(AccountMasterProjector::new(Arc::clone(&projection_db)));
        let ledger_projector = Arc::new(LedgerProjector::new(Arc::clone(&projection_db)));
        let trial_balance_projector =
            Arc::new(TrialBalanceProjector::new(Arc::clone(&projection_db)));

        // ProjectorRegistryを作成（静的ディスパッチ）
        let projector_registry = Arc::new(ProjectorRegistry::new(
            journal_entry_projector,
            account_master_projector,
            ledger_projector,
            trial_balance_projector,
        ));

        Self {
            projection_db,
            event_store,
            projector_registry,
            retry_queue: Arc::new(Mutex::new(VecDeque::new())),
            error_sender: Arc::new(Mutex::new(None)),
        }
    }
}

impl<J, A, L, T> ProjectionBuilderImpl<J, A, L, T>
where
    J: Projector + 'static,
    A: Projector + 'static,
    L: Projector + 'static,
    T: Projector + 'static,
{
    /// 単一イベントからProjectionを更新（内部実装）
    ///
    /// ProjectorRegistryを使用してイベントを処理する。
    ///
    /// # Arguments
    /// * `event` - 処理するイベント
    async fn process_event_internal(&self, event: &StoredEvent) -> ApplicationResult<()> {
        println!("✓ Processing event: {} (seq: {})", event.event_type, event.global_sequence);

        // ProjectorRegistryを使用してイベントを処理（静的ディスパッチ）
        self.projector_registry.process_event(event).await.map_err(|e| {
            ApplicationError::ProjectionDatabaseError(format!("Projector error: {}", e))
        })?;

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

impl ProjectionBuilderTrait
    for ProjectionBuilderImpl<
        JournalEntryProjector,
        AccountMasterProjector,
        LedgerProjector,
        TrialBalanceProjector,
    >
{
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
