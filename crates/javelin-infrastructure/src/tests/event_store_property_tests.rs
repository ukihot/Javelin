// Property-based tests for EventStore

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use serde::{Deserialize, Serialize};
    use tempfile::TempDir;

    use crate::event_store::EventStore;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestEvent {
        id: String,
        data: String,
    }

    // テストイベント生成戦略
    fn test_event_strategy() -> impl Strategy<Value = TestEvent> {
        (any::<String>(), any::<String>()).prop_map(|(id, data)| TestEvent { id, data })
    }

    // イベントリスト生成戦略（1-10個のイベント）
    fn event_list_strategy() -> impl Strategy<Value = Vec<TestEvent>> {
        prop::collection::vec(test_event_strategy(), 1..10)
    }

    /// プロパティ1: イベント永続化の完全性
    ///
    /// Feature: cqrs-infrastructure-integration, Property 1: イベント永続化の完全性
    ///
    /// 任意の仕訳操作（登録、承認、差戻し、更新、削除、訂正、取消、承認申請）に対して、
    /// 操作実行後にEventStoreに対応するドメインイベントが保存されていること
    ///
    /// **検証要件: 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8**
    ///
    /// 検証内容:
    /// - 任意のイベントリストを保存した後、すべてのイベントが取得できること
    /// - 保存されたイベントの内容が元のイベントと一致すること
    /// - イベントの順序が保持されること
    #[test]
    fn property_1_event_persistence_completeness() {
        proptest!(|(events in event_list_strategy(), aggregate_id in "[a-z]{5,10}")| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let temp_dir = TempDir::new().unwrap();
                let store = EventStore::new(temp_dir.path()).await.unwrap();

                // イベントを保存
                let _last_seq = store.append(&aggregate_id, events.clone()).await.unwrap();

                // イベントを取得
                let retrieved_events = store.get_events(&aggregate_id).await.unwrap();

                // すべてのイベントが取得できることを確認
                prop_assert_eq!(retrieved_events.len(), events.len());

                // 各イベントの内容が一致することを確認
                for (i, stored_event) in retrieved_events.iter().enumerate() {
                    let deserialized: TestEvent = serde_json::from_slice(&stored_event.payload).unwrap();
                    prop_assert_eq!(deserialized, events[i].clone());
                    prop_assert_eq!(&stored_event.aggregate_id, &aggregate_id);
                }

                Ok(())
            }).unwrap();
        });
    }

    /// プロパティ19: イベントメタデータの完全性
    ///
    /// Feature: cqrs-infrastructure-integration, Property 19: イベントメタデータの完全性
    ///
    /// 任意のEventStoreに保存されたイベントは、シーケンス番号とタイムスタンプを持つこと
    ///
    /// **検証要件: 10.1, 10.2**
    ///
    /// 検証内容:
    /// - すべてのイベントにシーケンス番号が付与されていること
    /// - シーケンス番号が連続していること
    /// - すべてのイベントにタイムスタンプが記録されていること
    /// - タイムスタンプが有効なRFC3339形式であること
    #[test]
    fn property_19_event_metadata_completeness() {
        proptest!(|(events in event_list_strategy(), aggregate_id in "[a-z]{5,10}")| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let temp_dir = TempDir::new().unwrap();
                let store = EventStore::new(temp_dir.path()).await.unwrap();

                // イベントを保存
                store.append(&aggregate_id, events.clone()).await.unwrap();

                // イベントを取得
                let retrieved_events = store.get_events(&aggregate_id).await.unwrap();

                // すべてのイベントにシーケンス番号とタイムスタンプがあることを確認
                for (i, stored_event) in retrieved_events.iter().enumerate() {
                    // シーケンス番号が付与されていること
                    prop_assert!(stored_event.global_sequence > 0);

                    // シーケンス番号が連続していること（最初のイベントから）
                    if i > 0 {
                        prop_assert_eq!(
                            stored_event.global_sequence,
                            retrieved_events[i - 1].global_sequence + 1
                        );
                    }

                    // タイムスタンプが記録されていること
                    prop_assert!(!stored_event.timestamp.is_empty());

                    // タイムスタンプが有効なRFC3339形式であること
                    prop_assert!(chrono::DateTime::parse_from_rfc3339(&stored_event.timestamp).is_ok());
                }

                Ok(())
            }).unwrap();
        });
    }

    /// プロパティ20: EventStoreの不変性
    ///
    /// Feature: cqrs-infrastructure-integration, Property 20: EventStoreの不変性
    ///
    /// 任意のEventStoreに保存されたイベントは、一度保存されたら変更されないこと（追記専用）
    ///
    /// **検証要件: 10.3**
    ///
    /// 検証内容:
    /// - イベントを保存した後、同じイベントを再度取得しても内容が変わらないこと
    /// - 新しいイベントを追加しても、既存のイベントが変更されないこと
    #[test]
    fn property_20_event_store_immutability() {
        proptest!(|(
            first_batch in event_list_strategy(),
            second_batch in event_list_strategy(),
            aggregate_id in "[a-z]{5,10}"
        )| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let temp_dir = TempDir::new().unwrap();
                let store = EventStore::new(temp_dir.path()).await.unwrap();

                // 最初のバッチを保存
                store.append(&aggregate_id, first_batch.clone()).await.unwrap();

                // 最初のバッチを取得
                let first_retrieval = store.get_events(&aggregate_id).await.unwrap();

                // 2番目のバッチを保存
                store.append(&aggregate_id, second_batch.clone()).await.unwrap();

                // 再度取得
                let second_retrieval = store.get_events(&aggregate_id).await.unwrap();

                // 最初のバッチのイベント数を確認
                prop_assert!(second_retrieval.len() >= first_retrieval.len());

                // 最初のバッチのイベントが変更されていないことを確認
                for (i, original_event) in first_retrieval.iter().enumerate() {
                    let current_event = &second_retrieval[i];

                    // シーケンス番号が同じ
                    prop_assert_eq!(current_event.global_sequence, original_event.global_sequence);

                    // ペイロードが同じ
                    prop_assert_eq!(&current_event.payload, &original_event.payload);

                    // タイムスタンプが同じ
                    prop_assert_eq!(&current_event.timestamp, &original_event.timestamp);

                    // 集約IDが同じ
                    prop_assert_eq!(&current_event.aggregate_id, &original_event.aggregate_id);
                }

                Ok(())
            }).unwrap();
        });
    }
}
