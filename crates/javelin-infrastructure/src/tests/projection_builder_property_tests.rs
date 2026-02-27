// Property-based tests for ProjectionBuilderImpl

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use javelin_application::projection_builder::ProjectionBuilder;
    use proptest::prelude::*;
    use serde::{Deserialize, Serialize};
    use tempfile::TempDir;

    use crate::{
        read::projections::{ProjectionBuilderImpl, ProjectionDb},
        write::event_store::EventStore,
    };

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestEvent {
        id: String,
        data: String,
    }

    // テストイベント生成戦略
    fn test_event_strategy() -> impl Strategy<Value = TestEvent> {
        (any::<String>(), any::<String>()).prop_map(|(id, data)| TestEvent { id, data })
    }

    // イベントリスト生成戦略（1-5個のイベント）
    fn event_list_strategy() -> impl Strategy<Value = Vec<TestEvent>> {
        prop::collection::vec(test_event_strategy(), 1..5)
    }

    /// プロパティ3: Projection再構築の完全性
    ///
    /// Feature: cqrs-infrastructure-integration, Property 3: Projection再構築の完全性
    ///
    /// 任意のイベントストリームに対して、ProjectionBuilderがすべてのイベントを
    /// 順次処理し、完全なProjectionDBを構築すること
    ///
    /// **検証要件: 2.1**
    ///
    /// 検証内容:
    /// - 任意のイベントストリームを保存した後、Projection再構築が成功すること
    /// - 再構築後、チェックポイントが最新のシーケンス番号に更新されていること
    #[test]
    fn property_3_projection_rebuild_completeness() {
        proptest!(|(events in event_list_strategy(), aggregate_id in "[a-z]{5,10}")| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let temp_dir = TempDir::new().unwrap();
                let event_store_path = temp_dir.path().join("events");
                let projection_db_path = temp_dir.path().join("projections");

                // EventStoreとProjectionDBを作成
                let event_store = Arc::new(EventStore::new(&event_store_path).await.unwrap());
                let projection_db = Arc::new(ProjectionDb::new(&projection_db_path).await.unwrap());

                // イベントを保存
                let last_seq = event_store.append(&aggregate_id, events.clone()).await.unwrap();

                // ProjectionBuilderを作成
                let builder = ProjectionBuilderImpl::new(
                    Arc::clone(&projection_db),
                    Arc::clone(&event_store),
                );

                // Projection再構築
                let result = builder.rebuild_all_projections().await;
                prop_assert!(result.is_ok(), "Projection rebuild should succeed");

                // チェックポイントが更新されていることを確認
                let position = projection_db.get_position("main", 1).await.unwrap();
                prop_assert_eq!(position, last_seq, "Checkpoint should be updated to last sequence");

                Ok(())
            }).unwrap();
        });
    }

    /// プロパティ4: Projection増分更新
    ///
    /// Feature: cqrs-infrastructure-integration, Property 4: Projection増分更新
    ///
    /// 任意の単一イベントに対して、ProjectionBuilderがそのイベントに基づいて
    /// ProjectionDBを正しく増分更新すること
    ///
    /// **検証要件: 2.2**
    ///
    /// 検証内容:
    /// - 任意の単一イベントを処理した後、エラーが発生しないこと
    /// - process_eventメソッドが正常に完了すること
    #[test]
    fn property_4_projection_incremental_update() {
        proptest!(|(event in test_event_strategy(), aggregate_id in "[a-z]{5,10}")| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let temp_dir = TempDir::new().unwrap();
                let event_store_path = temp_dir.path().join("events");
                let projection_db_path = temp_dir.path().join("projections");

                // EventStoreとProjectionDBを作成
                let event_store = Arc::new(EventStore::new(&event_store_path).await.unwrap());
                let projection_db = Arc::new(ProjectionDb::new(&projection_db_path).await.unwrap());

                // イベントを保存
                event_store.append(&aggregate_id, vec![event.clone()]).await.unwrap();

                // イベントを取得
                let stored_events = event_store.get_events(&aggregate_id).await.unwrap();
                prop_assert_eq!(stored_events.len(), 1);

                let stored_event = &stored_events[0];

                // ProjectionBuilderを作成
                let builder = ProjectionBuilderImpl::new(
                    Arc::clone(&projection_db),
                    Arc::clone(&event_store),
                );

                // 単一イベントを処理（バイト列として）
                let event_data = serde_json::to_vec(stored_event).unwrap();
                let result = builder.process_event(&event_data).await;
                prop_assert!(result.is_ok(), "Process event should succeed");

                Ok(())
            }).unwrap();
        });
    }

    // シリアルテスト例: 同じDB を共有してプロジェクション更新が順序どおり行われる
    use serial_test::serial;

    fn proj_shared_path() -> std::path::PathBuf {
        std::env::temp_dir().join("javelin_proj_serial")
    }

    #[test]
    #[serial]
    fn serial_projection_builder_write() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let path = proj_shared_path();
            println!("[test] serial_projection_builder_write: removing dir: {:?}", &path);
            let _ = std::fs::remove_dir_all(&path);
            println!("[test] serial_projection_builder_write: removed dir");
            let event_store = Arc::new(EventStore::new(&path).await.unwrap());
            println!("[test] serial_projection_builder_write: created EventStore");
            let projection_db = Arc::new(ProjectionDb::new(&path).await.unwrap());
            println!("[test] serial_projection_builder_write: created ProjectionDb");

            let builder =
                ProjectionBuilderImpl::new(Arc::clone(&projection_db), Arc::clone(&event_store));
            println!("[test] serial_projection_builder_write: before rebuild");
            // just ensure we can rebuild even on empty store
            builder.rebuild_all_projections().await.unwrap();
            println!("[test] serial_projection_builder_write: after rebuild");
        });
    }

    #[test]
    #[serial]
    fn serial_projection_builder_read() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let path = proj_shared_path();
            // ensure a clean environment before opening LMDB to avoid leftover files from previous
            // runs
            println!(
                "[test] serial_projection_builder_read: removing dir before open: {:?}",
                &path
            );
            let _ = std::fs::remove_dir_all(&path);
            println!("[test] serial_projection_builder_read: path={:?}", &path);
            let event_store = Arc::new(EventStore::new(&path).await.unwrap());
            println!("[test] serial_projection_builder_read: created EventStore");
            println!("[test] serial_projection_builder_read: path exists={} ", path.exists());
            if path.exists() {
                match std::fs::read_dir(&path) {
                    Ok(entries) => {
                        for e in entries.flatten() {
                            println!("[test] serial_projection_builder_read: entry={:?}", e.path());
                        }
                    }
                    Err(err) => {
                        println!("[test] serial_projection_builder_read: read_dir error={}", err)
                    }
                }
            }
            let projection_db = Arc::new(ProjectionDb::new(&path).await.unwrap());
            println!("[test] serial_projection_builder_read: created ProjectionDb");
            let builder =
                ProjectionBuilderImpl::new(Arc::clone(&projection_db), Arc::clone(&event_store));
            println!("[test] serial_projection_builder_read: before rebuild");
            // ensure builder still works (reads checkpoint etc.)
            builder.rebuild_all_projections().await.unwrap();
            println!("[test] serial_projection_builder_read: after rebuild");
        });
    }
}
