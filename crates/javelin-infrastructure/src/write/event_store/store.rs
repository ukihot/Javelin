// 現代Rust設計によるEventStore実装
// - Iterator指向のストリーム
// - 型安全なキー設計
// - TryFrom/From による変換集約
// - std::fmt::from_fn によるログ出力

use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use lmdb::{
    Cursor, Database, DatabaseFlags, Environment, EnvironmentFlags, Transaction, WriteFlags,
};
use lmdb_sys as ffi;

use super::event_stream::{EventStream, EventStreamBuilder, StoredEvent};
use crate::{
    error::{InfrastructureError, InfrastructureResult},
    storage_metrics::{DurabilityPolicy, StorageMetrics},
    types::{AggregateId, ExpectedVersion, Sequence},
};

/// イベント通知コールバック型
pub type EventNotificationCallback = Arc<
    dyn Fn(StoredEvent) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

pub struct EventStore {
    env: Arc<Environment>,
    events_db: Database,
    meta_db: Database,
    #[allow(dead_code)]
    path: PathBuf,
    current_map_size: Arc<Mutex<usize>>,
    #[allow(dead_code)]
    durability_policy: DurabilityPolicy,
    /// イベント保存後の通知コールバック
    notification_callback: Arc<Mutex<Option<EventNotificationCallback>>>,
}

impl EventStore {
    pub async fn new(path: &Path) -> InfrastructureResult<Self> {
        Self::new_with_config(path, 100 * 1024 * 1024, DurabilityPolicy::default()).await
    }

    pub async fn new_with_config(
        path: &Path,
        initial_map_size: usize,
        durability_policy: DurabilityPolicy,
    ) -> InfrastructureResult<Self> {
        if !path.exists() {
            tokio::fs::create_dir_all(path).await.map_err(|e| {
                InfrastructureError::EventStoreInitFailed {
                    path: path.display().to_string(),
                    source: e,
                }
            })?;
        }

        let data_file = path.join("data.mdb");
        let existing_size = if data_file.exists() {
            tokio::fs::metadata(&data_file).await.map(|m| m.len() as usize).unwrap_or(0)
        } else {
            0
        };

        // 既存ファイルサイズに基づいて map_size を調整するが、過度な増加を防止する
        // - 既存ファイルが無ければ `initial_map_size` を使用
        // - 既存ファイルがある場合は既存サイズの +50% を試算するが上限を設ける
        // - 過去の運用で極端に大きなファイルが残っていると exponential に増える懸念があるため
        //   安全側に倒して上限を低めに設定する（現状 1GB）
        const MAX_MAP_SIZE: usize = 1024 * 1024 * 1024; // 1GB
        let map_size = if existing_size == 0 {
            initial_map_size
        } else {
            // candidate は existing_size の +50%（過度な拡大を抑える）
            let mut candidate = existing_size.saturating_add(existing_size / 2);
            if candidate > MAX_MAP_SIZE {
                // 既存ファイルが異常に大きい場合は上限に切り詰める（安全措置）
                eprintln!(
                    "EventStore: existing data.mdb size ({}) exceeds MAX_MAP_SIZE ({}). capping to MAX.",
                    existing_size, MAX_MAP_SIZE
                );
                candidate = MAX_MAP_SIZE;
            }
            std::cmp::max(initial_map_size, candidate)
        };

        let mut env_builder = Environment::new();
        env_builder.set_max_dbs(2).set_map_size(map_size);

        match durability_policy {
            DurabilityPolicy::MaxDurability => {}
            DurabilityPolicy::Balanced => {
                env_builder.set_flags(EnvironmentFlags::NO_META_SYNC);
            }
            DurabilityPolicy::MaxPerformance => {
                env_builder.set_flags(EnvironmentFlags::NO_SYNC | EnvironmentFlags::NO_META_SYNC);
            }
        }

        let env = env_builder
            .open(path)
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        let events_db = env
            .create_db(Some("events"), DatabaseFlags::empty())
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        let meta_db = env
            .create_db(Some("meta"), DatabaseFlags::empty())
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        Ok(Self {
            env: Arc::new(env),
            events_db,
            meta_db,
            path: path.to_path_buf(),
            current_map_size: Arc::new(Mutex::new(map_size)),
            durability_policy,
            notification_callback: Arc::new(Mutex::new(None)),
        })
    }

    /// 複数イベントを一括追記
    ///
    /// 指定された集約IDに対して複数のドメインイベントを一括で保存する。
    /// すべてのイベントは同一トランザクション内で処理され、
    /// シーケンス番号が自動採番され、タイムスタンプが記録される。
    ///
    /// # Arguments
    /// * `aggregate_id` - 集約ID
    /// * `events` - 保存するドメインイベントのリスト
    ///
    /// # Returns
    /// 最後に保存されたイベントのシーケンス番号
    ///
    /// # Errors
    /// - イベントのシリアライズに失敗した場合
    /// - LMDBへの書き込みに失敗した場合
    /// - トランザクションのコミットに失敗した場合
    pub async fn append<T>(&self, aggregate_id: &str, events: Vec<T>) -> InfrastructureResult<u64>
    where
        T: serde::Serialize + Send + 'static,
    {
        if events.is_empty() {
            return Err(InfrastructureError::ValidationFailed(
                "Cannot append empty event list".to_string(),
            ));
        }

        let aggregate_id = aggregate_id.to_string();
        let env = Arc::clone(&self.env);
        let events_db = self.events_db;
        let meta_db = self.meta_db;

        // イベントを事前にシリアライズ
        let serialized_events: Vec<Vec<u8>> = events
            .into_iter()
            .map(|event| {
                serde_json::to_vec(&event)
                    .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let (last_sequence, stored_events) = tokio::task::spawn_blocking(move || {
            let mut txn =
                env.begin_rw_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            // グローバルシーケンス取得
            let seq_key = b"next_sequence";
            let mut current_sequence = match txn.get(meta_db, &seq_key) {
                Ok(bytes) => {
                    let arr = bytes.as_array::<8>().ok_or_else(|| {
                        InfrastructureError::DeserializationFailed("Invalid sequence".to_string())
                    })?;
                    u64::from_be_bytes(*arr)
                }
                Err(lmdb::Error::NotFound) => 0,
                Err(e) => return Err(InfrastructureError::LmdbError(e.to_string())),
            };

            let timestamp = chrono::Utc::now().to_rfc3339();
            let mut last_seq = 0u64;
            // モダンプラクティス: 事前にキャパシティを確保
            let mut stored_events = Vec::with_capacity(serialized_events.len());

            // 各イベントを保存
            for event_data in serialized_events {
                current_sequence += 1;
                last_seq = current_sequence;

                // payloadからイベントタイプを抽出
                let event_type = if let Ok(json_value) =
                    serde_json::from_slice::<serde_json::Value>(&event_data)
                {
                    json_value.get("type").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string()
                } else {
                    "Unknown".to_string()
                };

                // StoredEvent構造を構築
                let stored_event = StoredEvent {
                    global_sequence: current_sequence,
                    event_type,
                    aggregate_id: aggregate_id.clone(),
                    version: current_sequence, // バージョンはシーケンスと同じ
                    timestamp: timestamp.clone(),
                    payload: event_data,
                };

                let event_key = current_sequence.to_be_bytes();
                let event_value = serde_json::to_vec(&stored_event)
                    .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

                txn.put(events_db, &event_key, &event_value, WriteFlags::empty())
                    .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

                stored_events.push(stored_event);
            }

            // 最新シーケンス番号を更新
            txn.put(meta_db, &seq_key, &current_sequence.to_be_bytes(), WriteFlags::empty())
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            txn.commit().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            Ok::<(u64, Vec<StoredEvent>), InfrastructureError>((last_seq, stored_events))
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        // イベント通知を送信（非同期で実行）
        let callback_opt = self.notification_callback.lock().unwrap().clone();
        if let Some(callback) = callback_opt {
            println!("✓ Sending {} event notifications", stored_events.len());
            for event in stored_events {
                println!(
                    "  - Notifying event: {} (seq: {})",
                    event.event_type, event.global_sequence
                );
                let callback = Arc::clone(&callback);
                tokio::spawn(async move {
                    callback(event).await;
                });
            }
        } else {
            println!("⚠ No notification callback registered");
        }

        Ok(last_sequence)
    }

    /// イベント追記 - 楽観的ロック対応
    pub async fn append_event(
        &self,
        event_type: &str,
        aggregate_id: &str,
        version: u64,
        expected_version: ExpectedVersion,
        payload: &[u8],
    ) -> InfrastructureResult<Sequence> {
        let event_type = event_type.to_string();
        let aggregate_id = aggregate_id.to_string();
        let payload = payload.to_vec();

        let env = Arc::clone(&self.env);
        let events_db = self.events_db;
        let meta_db = self.meta_db;

        let sequence = tokio::task::spawn_blocking(move || {
            let mut txn =
                env.begin_rw_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            // 楽観的ロックチェック（必要に応じて）
            if !expected_version.matches(version.saturating_sub(1)) {
                return Err(InfrastructureError::ConcurrencyConflict {
                    aggregate_id: aggregate_id.clone(),
                    expected: expected_version.0,
                    actual: version.saturating_sub(1),
                });
            }

            // グローバルシーケンス発番
            let seq_key = b"next_sequence";
            let current = match txn.get(meta_db, &seq_key) {
                Ok(bytes) => {
                    // 現代Rust: as_array による型安全な変換
                    let arr = bytes.as_array::<8>().ok_or_else(|| {
                        InfrastructureError::DeserializationFailed("Invalid sequence".to_string())
                    })?;
                    u64::from_be_bytes(*arr)
                }
                Err(lmdb::Error::NotFound) => 0,
                Err(e) => return Err(InfrastructureError::LmdbError(e.to_string())),
            };

            let global_sequence = Sequence::new(current + 1);
            txn.put(meta_db, &seq_key, &global_sequence.to_be_bytes(), WriteFlags::empty())
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            // イベント構築
            let stored_event = StoredEvent {
                global_sequence: global_sequence.as_u64(),
                event_type,
                aggregate_id,
                version,
                timestamp: chrono::Utc::now().to_rfc3339(),
                payload,
            };

            let event_key = global_sequence.to_be_bytes();
            let event_value = serde_json::to_vec(&stored_event)
                .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

            txn.put(events_db, &event_key, &event_value, WriteFlags::empty())
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            txn.commit().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            Ok::<Sequence, InfrastructureError>(global_sequence)
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(sequence)
    }

    /// イベントストリームを取得（Iterator指向）
    pub fn stream_events(&self, from_sequence: Sequence) -> EventStream {
        EventStreamBuilder::new(Arc::clone(&self.env), self.events_db)
            .from_sequence(from_sequence)
            .build()
    }

    /// Aggregate単位のイベントストリーム
    pub fn stream_aggregate_events(
        &self,
        aggregate_id: AggregateId,
        from_sequence: Sequence,
    ) -> EventStream {
        EventStreamBuilder::new(Arc::clone(&self.env), self.events_db)
            .from_sequence(from_sequence)
            .for_aggregate(aggregate_id)
            .build()
    }

    /// 指定された集約IDのイベントストリームを取得
    ///
    /// 指定された集約IDに関連するすべてのイベントをシーケンス順に取得する。
    /// このメソッドは状態再現性（要件10.5）を実現するために使用される。
    ///
    /// # Arguments
    /// * `aggregate_id` - 集約ID（文字列形式）
    ///
    /// # Returns
    /// イベントのベクタ（シーケンス順）
    ///
    /// # Errors
    /// - LMDBからの読み取りに失敗した場合
    /// - イベントのデシリアライズに失敗した場合
    pub async fn get_events(&self, aggregate_id: &str) -> InfrastructureResult<Vec<StoredEvent>> {
        let aggregate_id = aggregate_id.to_string();
        let env = Arc::clone(&self.env);
        let events_db = self.events_db;

        let events = tokio::task::spawn_blocking(move || {
            let txn =
                env.begin_ro_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            let cursor = txn
                .open_ro_cursor(events_db)
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            // モダンプラクティス: 初期キャパシティを確保（平均的なイベント数を想定）
            let mut events = Vec::with_capacity(16);

            // データベースが空かどうかを確認するため、最初のキーを取得を試みる
            // 空の場合はNotFoundエラーが返される
            match cursor.get(None, None, ffi::MDB_FIRST) {
                Ok((_, value)) => {
                    // 最初のイベントを処理
                    let event: StoredEvent = serde_json::from_slice(value)
                        .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;

                    if event.aggregate_id == aggregate_id {
                        events.push(event);
                    }

                    // 残りのイベントを処理
                    loop {
                        match cursor.get(None, None, ffi::MDB_NEXT) {
                            Ok((_, value)) => {
                                let event: StoredEvent =
                                    serde_json::from_slice(value).map_err(|e| {
                                        InfrastructureError::DeserializationFailed(e.to_string())
                                    })?;

                                if event.aggregate_id == aggregate_id {
                                    events.push(event);
                                }
                            }
                            Err(lmdb::Error::NotFound) => break,
                            Err(e) => return Err(InfrastructureError::LmdbError(e.to_string())),
                        }
                    }
                }
                Err(lmdb::Error::NotFound) => {
                    // データベースが空の場合は空のベクタを返す
                }
                Err(e) => return Err(InfrastructureError::LmdbError(e.to_string())),
            }

            // シーケンス順にソート（念のため）
            events.sort_by_key(|e| e.global_sequence);

            Ok::<Vec<StoredEvent>, InfrastructureError>(events)
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(events)
    }

    /// 指定されたシーケンス番号以降の全イベントを取得
    ///
    /// Projection再構築用のメソッド。指定されたシーケンス番号以降の
    /// すべてのイベントをシーケンス順に取得する。
    ///
    /// # Arguments
    /// * `from_sequence` - 開始シーケンス番号（この番号を含む）
    ///
    /// # Returns
    /// イベントのベクタ（シーケンス順）
    ///
    /// # Errors
    /// - LMDBからの読み取りに失敗した場合
    /// - イベントのデシリアライズに失敗した場合
    pub async fn get_all_events(
        &self,
        from_sequence: u64,
    ) -> InfrastructureResult<Vec<StoredEvent>> {
        let env = Arc::clone(&self.env);
        let events_db = self.events_db;

        let events = tokio::task::spawn_blocking(move || {
            let txn =
                env.begin_ro_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            let cursor = txn
                .open_ro_cursor(events_db)
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            // モダンプラクティス: 初期キャパシティを確保（バッチ処理を想定）
            let mut events = Vec::with_capacity(100);

            // 指定されたシーケンス番号から開始
            let start_key = from_sequence.to_be_bytes();

            // カーソルを指定されたキー以降に移動
            match cursor.get(Some(&start_key), None, ffi::MDB_SET_RANGE) {
                Ok((_, value)) => {
                    // 最初のイベントを処理
                    let event: StoredEvent = serde_json::from_slice(value)
                        .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;

                    if event.global_sequence >= from_sequence {
                        events.push(event);
                    }

                    // 残りのイベントを処理
                    loop {
                        match cursor.get(None, None, ffi::MDB_NEXT) {
                            Ok((_, value)) => {
                                let event: StoredEvent =
                                    serde_json::from_slice(value).map_err(|e| {
                                        InfrastructureError::DeserializationFailed(e.to_string())
                                    })?;

                                if event.global_sequence >= from_sequence {
                                    events.push(event);
                                }
                            }
                            Err(lmdb::Error::NotFound) => break,
                            Err(e) => return Err(InfrastructureError::LmdbError(e.to_string())),
                        }
                    }
                }
                Err(lmdb::Error::NotFound) => {
                    // 指定されたシーケンス以降のイベントがない場合は空のベクタを返す
                }
                Err(e) => return Err(InfrastructureError::LmdbError(e.to_string())),
            }

            // シーケンス順にソート（念のため）
            events.sort_by_key(|e| e.global_sequence);

            Ok::<Vec<StoredEvent>, InfrastructureError>(events)
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(events)
    }

    /// 最新シーケンス取得
    pub async fn get_latest_sequence(&self) -> InfrastructureResult<Sequence> {
        let env = Arc::clone(&self.env);
        let meta_db = self.meta_db;

        let result = tokio::task::spawn_blocking(move || {
            let txn =
                env.begin_ro_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            let key = b"next_sequence";
            match txn.get(meta_db, &key) {
                Ok(bytes) => {
                    let arr = bytes.as_array::<8>().ok_or_else(|| {
                        InfrastructureError::DeserializationFailed("Invalid sequence".to_string())
                    })?;
                    Ok(Sequence::from_be_bytes(*arr))
                }
                Err(lmdb::Error::NotFound) => Ok(Sequence::new(0)),
                Err(e) => Err(InfrastructureError::LmdbError(e.to_string())),
            }
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(result)
    }

    /// ストレージメトリクス取得
    pub async fn get_storage_metrics(&self) -> InfrastructureResult<StorageMetrics> {
        let env = Arc::clone(&self.env);
        let events_db = self.events_db;
        let current_map_size = *self.current_map_size.lock().unwrap();

        let metrics = tokio::task::spawn_blocking(move || {
            let txn =
                env.begin_ro_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            use lmdb_sys as ffi;

            let mut env_stat: ffi::MDB_stat = unsafe { std::mem::zeroed() };
            let mut env_info: ffi::MDB_envinfo = unsafe { std::mem::zeroed() };

            let env_ptr = env.as_ref() as *const lmdb::Environment as *mut ffi::MDB_env;

            unsafe {
                let ret = ffi::mdb_env_stat(env_ptr, &mut env_stat);
                if ret != 0 {
                    return Err(InfrastructureError::LmdbError(format!(
                        "mdb_env_stat failed: {}",
                        ret
                    )));
                }

                let ret = ffi::mdb_env_info(env_ptr, &mut env_info);
                if ret != 0 {
                    return Err(InfrastructureError::LmdbError(format!(
                        "mdb_env_info failed: {}",
                        ret
                    )));
                }
            }

            let mut db_stat: ffi::MDB_stat = unsafe { std::mem::zeroed() };
            unsafe {
                let ret = ffi::mdb_stat(txn.txn(), events_db.dbi(), &mut db_stat);
                if ret != 0 {
                    return Err(InfrastructureError::LmdbError(format!(
                        "mdb_stat failed: {}",
                        ret
                    )));
                }
            }

            let page_size = env_stat.ms_psize as usize;
            let last_page_no = env_info.me_last_pgno;
            let used_size = page_size * last_page_no;
            let usage_percent = (used_size as f64 * 100.0) / current_map_size as f64;

            Ok::<StorageMetrics, InfrastructureError>(StorageMetrics {
                map_size: current_map_size,
                used_size,
                usage_percent,
                page_size,
                last_page_no,
                entries: db_stat.ms_entries,
            })
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(metrics)
    }

    /// デバッグ用：std::fmt::from_fn によるイベントダンプ
    pub fn dump_event_info<'a>(&self, event: &'a StoredEvent) -> impl std::fmt::Display + 'a {
        let seq = event.global_sequence;
        let event_type = event.event_type.clone();
        let aggregate_id = event.aggregate_id.clone();
        let version = event.version;

        std::fmt::from_fn(move |f| {
            write!(
                f,
                "Event[seq={}, type={}, agg={}, ver={}]",
                seq, event_type, aggregate_id, version
            )
        })
    }

    /// イベント通知コールバックを設定
    ///
    /// イベント保存後に呼び出されるコールバックを設定する。
    /// ProjectionBuilderなどがこのコールバックを登録し、
    /// イベント保存時に自動的にProjectionを更新できる。
    ///
    /// # Arguments
    /// * `callback` - イベント通知コールバック
    ///
    /// # Requirements
    /// 要件: 7.1
    pub fn set_notification_callback(&self, callback: EventNotificationCallback) {
        *self.notification_callback.lock().unwrap() = Some(callback);
    }

    /// イベント通知コールバックをクリア
    pub fn clear_notification_callback(&self) {
        *self.notification_callback.lock().unwrap() = None;
    }
}
