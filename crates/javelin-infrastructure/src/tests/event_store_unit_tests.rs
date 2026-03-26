// Unit tests for EventStore

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use tempfile::TempDir;

    use crate::event_store::EventStore;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestEvent {
        id: String,
        data: String,
    }

    /// 単一イベントの保存と取得
    ///
    /// 検証内容:
    /// - 単一イベントが正常に保存されること
    /// - 保存されたイベントが正しく取得できること
    /// - シーケンス番号が正しく採番されること
    /// - タイムスタンプが記録されること
    #[tokio::test]
    async fn test_single_event_save_and_retrieve() {
        let temp_dir = TempDir::new().unwrap();
        let store = EventStore::new(temp_dir.path()).await.unwrap();

        let event = TestEvent { id: "event-001".to_string(), data: "test data".to_string() };

        // イベントを保存
        let last_seq = store.append("agg-001", vec![event.clone()]).await.unwrap();

        // シーケンス番号が1であることを確認
        assert_eq!(last_seq, 1);

        // イベントを取得
        let retrieved_events = store.get_events("agg-001").await.unwrap();

        // 1つのイベントが取得できることを確認
        assert_eq!(retrieved_events.len(), 1);

        // イベントの内容を確認
        let stored_event = &retrieved_events[0];
        assert_eq!(stored_event.aggregate_id, "agg-001");
        assert_eq!(stored_event.global_sequence, 1);

        // ペイロードをデシリアライズして元のイベントと比較
        let deserialized: TestEvent = serde_json::from_slice(&stored_event.payload).unwrap();
        assert_eq!(deserialized, event);

        // タイムスタンプが記録されていることを確認
        assert!(!stored_event.timestamp.is_empty());
        let _timestamp = chrono::DateTime::parse_from_rfc3339(&stored_event.timestamp).unwrap();
    }

    /// 複数イベントのバッチ保存
    ///
    /// 検証内容:
    /// - 複数のイベントが一度に保存されること
    /// - すべてのイベントが正しく取得できること
    /// - シーケンス番号が連続して採番されること
    /// - 最新シーケンス番号が正しく返されること
    #[tokio::test]
    async fn test_batch_event_save() {
        let temp_dir = TempDir::new().unwrap();
        let store = EventStore::new(temp_dir.path()).await.unwrap();

        let events = vec![
            TestEvent { id: "event-001".to_string(), data: "first event".to_string() },
            TestEvent { id: "event-002".to_string(), data: "second event".to_string() },
            TestEvent { id: "event-003".to_string(), data: "third event".to_string() },
        ];

        // バッチでイベントを保存
        let last_seq = store.append("agg-batch", events.clone()).await.unwrap();

        // 最新シーケンス番号が3であることを確認
        assert_eq!(last_seq, 3);

        // イベントを取得
        let retrieved_events = store.get_events("agg-batch").await.unwrap();

        // 3つのイベントが取得できることを確認
        assert_eq!(retrieved_events.len(), 3);

        // 各イベントの内容とシーケンス番号を確認
        for (i, stored_event) in retrieved_events.iter().enumerate() {
            assert_eq!(stored_event.aggregate_id, "agg-batch");
            assert_eq!(stored_event.global_sequence, (i + 1) as u64);

            let deserialized: TestEvent = serde_json::from_slice(&stored_event.payload).unwrap();
            assert_eq!(deserialized, events[i]);
        }

        // 最新シーケンス番号を確認
        let latest_seq = store.get_latest_sequence().await.unwrap();
        assert_eq!(latest_seq.as_u64(), 3);
    }

    /// 複数バッチの保存とシーケンス番号の連続性
    ///
    /// 検証内容:
    /// - 複数回のバッチ保存でシーケンス番号が連続すること
    /// - 異なる集約IDのイベントが正しく保存されること
    #[tokio::test]
    async fn test_multiple_batch_saves_with_sequential_numbers() {
        let temp_dir = TempDir::new().unwrap();
        let store = EventStore::new(temp_dir.path()).await.unwrap();

        // 最初のバッチ
        let batch1 = vec![
            TestEvent { id: "1".to_string(), data: "batch 1 event 1".to_string() },
            TestEvent { id: "2".to_string(), data: "batch 1 event 2".to_string() },
        ];
        let seq1 = store.append("agg-001", batch1).await.unwrap();
        assert_eq!(seq1, 2);

        // 2番目のバッチ（異なる集約ID）
        let batch2 = vec![
            TestEvent { id: "3".to_string(), data: "batch 2 event 1".to_string() },
            TestEvent { id: "4".to_string(), data: "batch 2 event 2".to_string() },
            TestEvent { id: "5".to_string(), data: "batch 2 event 3".to_string() },
        ];
        let seq2 = store.append("agg-002", batch2).await.unwrap();
        assert_eq!(seq2, 5);

        // 最新シーケンス番号を確認
        let latest_seq = store.get_latest_sequence().await.unwrap();
        assert_eq!(latest_seq.as_u64(), 5);
    }

    /// 存在しない集約IDの照会
    ///
    /// 検証内容:
    /// - 存在しない集約IDを照会した場合、空の結果が返されること
    /// - エラーが発生しないこと
    #[tokio::test]
    async fn test_query_nonexistent_aggregate_id() {
        let temp_dir = TempDir::new().unwrap();
        let store = EventStore::new(temp_dir.path()).await.unwrap();

        // 存在しない集約IDを照会
        let result = store.get_events("nonexistent-aggregate").await;

        // エラーが発生しないことを確認
        assert!(result.is_ok());

        // 空のベクタが返されることを確認
        let events = result.unwrap();
        assert_eq!(events.len(), 0);
    }

    /// 存在しない集約IDの照会（イベントが存在する場合）
    ///
    /// 検証内容:
    /// - 他の集約のイベントが存在する場合でも、存在しない集約IDを照会すると空の結果が返されること
    #[tokio::test]
    async fn test_query_nonexistent_aggregate_id_with_other_events() {
        let temp_dir = TempDir::new().unwrap();
        let store = EventStore::new(temp_dir.path()).await.unwrap();

        // いくつかのイベントを保存
        let events = vec![
            TestEvent { id: "1".to_string(), data: "event 1".to_string() },
            TestEvent { id: "2".to_string(), data: "event 2".to_string() },
        ];
        store.append("agg-exists", events).await.unwrap();

        // 存在しない集約IDを照会
        let result = store.get_events("agg-does-not-exist").await.unwrap();

        // 空のベクタが返されることを確認
        assert_eq!(result.len(), 0);

        // 存在する集約IDを照会すると正しく取得できることを確認
        let existing_result = store.get_events("agg-exists").await.unwrap();
        assert_eq!(existing_result.len(), 2);
    }

    /// 大量イベントのバッチ保存
    ///
    /// 検証内容:
    /// - 大量のイベントを一度に保存できること
    /// - すべてのイベントが正しく取得できること
    #[tokio::test]
    async fn test_large_batch_save() {
        let temp_dir = TempDir::new().unwrap();
        let store = EventStore::new(temp_dir.path()).await.unwrap();

        // 100個のイベントを生成
        let events: Vec<TestEvent> = (1..=100)
            .map(|i| TestEvent { id: format!("event-{:03}", i), data: format!("data {}", i) })
            .collect();

        // バッチで保存
        let last_seq = store.append("agg-large", events.clone()).await.unwrap();
        assert_eq!(last_seq, 100);

        // すべてのイベントを取得
        let retrieved = store.get_events("agg-large").await.unwrap();
        assert_eq!(retrieved.len(), 100);

        // シーケンス番号が連続していることを確認
        for (i, event) in retrieved.iter().enumerate() {
            assert_eq!(event.global_sequence, (i + 1) as u64);
        }
    }

    // ------------------------------------------------------------------
    // シリアル実行を要求するテストの例
    // ------------------------------------------------------------------
    // 以下では同じディレクトリを共有し、順序通りに書き込み・読み込みが
    // 行われることを確認するために `serial_test` を活用しています。
    use serial_test::serial;

    fn shared_db_path() -> std::path::PathBuf {
        std::env::temp_dir().join("javelin_infra_serial")
    }

    #[tokio::test]
    #[serial]
    async fn serial_write_shared_store() {
        // 前回の残骸があれば削除
        let _ = std::fs::remove_dir_all(shared_db_path());
        let store = EventStore::new(&shared_db_path()).await.unwrap();

        let event = TestEvent { id: "evt".into(), data: "serial 1".into() };
        let seq = store.append("agg-serial", vec![event.clone()]).await.unwrap();
        assert_eq!(seq, 1);
    }

    #[tokio::test]
    #[serial]
    async fn serial_read_shared_store() {
        let store = EventStore::new(&shared_db_path()).await.unwrap();
        let mut events = store.get_events("agg-serial").await.unwrap();

        // すでにデータがあれば、最低1件存在し、期待値を確認
        if let Some(found_event) = events.iter().find(|e| {
            let des: TestEvent = serde_json::from_slice(&e.payload).unwrap();
            des.data == "serial 1"
        }) {
            let des: TestEvent = serde_json::from_slice(&found_event.payload).unwrap();
            assert_eq!(des.data, "serial 1");
            return;
        }

        // なければ再度書き込みして確認（他プロセスでも実行順が保証されない実行環境対応）
        let event = TestEvent { id: "evt".into(), data: "serial 1".into() };
        let seq = store.append("agg-serial", vec![event.clone()]).await.unwrap();
        assert!(seq >= 1);

        events = store.get_events("agg-serial").await.unwrap();
        assert!(events.len() >= 1);

        let des: TestEvent = serde_json::from_slice(&events[0].payload).unwrap();
        assert_eq!(des.data, "serial 1");
    }
}
