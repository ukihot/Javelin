// ProjectionDB - ReadModel保存
// 保存内容: Query最適化構造
// 再構築: Event再生
// 独立性: Projection単位で管理
// 冪等性: event_sequence追跡

use std::{path::Path, sync::Arc};

use lmdb::{Database, DatabaseFlags, Environment, Transaction, WriteFlags};
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionPosition {
    pub projection_name: String,
    pub projection_version: u32, // プロジェクションロジックバージョン
    pub last_processed_sequence: u64,
    pub updated_at: String,
}

pub struct ProjectionDb {
    env: Arc<Environment>,
    state_db: Database, // Read Model本体
    meta_db: Database,  // チェックポイント・バージョン管理
}

impl ProjectionDb {
    pub async fn new(path: &Path) -> InfrastructureResult<Self> {
        // ディレクトリが存在しない場合は作成
        if !path.exists() {
            tokio::fs::create_dir_all(path).await.map_err(|e| {
                InfrastructureError::ProjectionDbInitFailed {
                    path: path.display().to_string(),
                    source: e,
                }
            })?;
        }

        // LMDB環境の初期化
        let env = Environment::new()
            .set_max_dbs(2) // state + meta
            .set_map_size(100 * 1024 * 1024) // 100MB
            .open(path)
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        let state_db = env
            .create_db(Some("state"), DatabaseFlags::empty())
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        let meta_db = env
            .create_db(Some("meta"), DatabaseFlags::empty())
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        Ok(Self { env: Arc::new(env), state_db, meta_db })
    }

    /// プロジェクション位置を取得
    pub async fn get_position(
        &self,
        projection_name: &str,
        projection_version: u32,
    ) -> InfrastructureResult<u64> {
        let env = Arc::clone(&self.env);
        let meta_db = self.meta_db;
        let key = format!("{}:v{}", projection_name, projection_version);

        let result = tokio::task::spawn_blocking(move || {
            let txn =
                env.begin_ro_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            match txn.get(meta_db, &key.as_bytes()) {
                Ok(bytes) => {
                    let position: ProjectionPosition = serde_json::from_slice(bytes)
                        .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;
                    Ok(position.last_processed_sequence)
                }
                Err(lmdb::Error::NotFound) => Ok(0),
                Err(e) => Err(InfrastructureError::LmdbError(e.to_string())),
            }
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(result)
    }

    /// プロジェクション更新（複数キー + チェックポイント、同一トランザクション）
    /// 重要: 途中クラッシュ対策のため、state更新とmeta更新を同一txnで実行
    pub async fn update_projection_batch(
        &self,
        projection_name: &str,
        projection_version: u32,
        updates: Vec<(String, Vec<u8>)>,
        event_sequence: u64,
    ) -> InfrastructureResult<()> {
        let env = Arc::clone(&self.env);
        let state_db = self.state_db;
        let meta_db = self.meta_db;
        let projection_name = projection_name.to_string(); // 所有権を取得
        let checkpoint_key = format!("{}:v{}", projection_name, projection_version);

        tokio::task::spawn_blocking(move || {
            // 単一RWトランザクション内で全更新を実行
            let mut txn =
                env.begin_rw_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            // 1. 全state更新
            for (key, value) in updates {
                // データを直接保存（メタデータなし）
                txn.put(state_db, &key.as_bytes(), &value, WriteFlags::empty())
                    .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
            }

            // 2. チェックポイント更新
            let position = ProjectionPosition {
                projection_name,
                projection_version,
                last_processed_sequence: event_sequence,
                updated_at: chrono::Utc::now().to_rfc3339(),
            };
            let position_bytes = serde_json::to_vec(&position)
                .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

            txn.put(meta_db, &checkpoint_key.as_bytes(), &position_bytes, WriteFlags::empty())
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            // 3. 単一コミット（アトミック性保証）
            txn.commit().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            Ok::<_, InfrastructureError>(())
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(())
    }

    /// Projectionを更新（単一キー、後方互換用）
    pub async fn update_projection(
        &self,
        key: &str,
        value: &[u8],
        event_sequence: u64,
    ) -> InfrastructureResult<()> {
        self.update_projection_batch(
            "default",
            1,
            vec![(key.to_string(), value.to_vec())],
            event_sequence,
        )
        .await
    }

    /// Projectionを取得
    pub async fn get_projection(&self, key: &str) -> InfrastructureResult<Option<Vec<u8>>> {
        let env = Arc::clone(&self.env);
        let state_db = self.state_db;
        let key = key.to_string();

        let result = tokio::task::spawn_blocking(move || {
            let txn =
                env.begin_ro_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            match txn.get(state_db, &key.as_bytes()) {
                Ok(bytes) => {
                    // データを直接返す（メタデータなし）
                    Ok(Some(bytes.to_vec()))
                }
                Err(lmdb::Error::NotFound) => Ok(None),
                Err(e) => Err(InfrastructureError::LmdbError(e.to_string())),
            }
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(result)
    }

    /// Projectionを削除
    pub async fn delete_projection(&self, key: &str) -> InfrastructureResult<()> {
        let env = Arc::clone(&self.env);
        let state_db = self.state_db;
        let key = key.to_string();

        tokio::task::spawn_blocking(move || {
            let mut txn =
                env.begin_rw_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            txn.del(state_db, &key.as_bytes(), None)
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            txn.commit().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            Ok::<_, InfrastructureError>(())
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_projection_db_initialization() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let projection_db_path = temp_dir.path().join("projections");

        let result = ProjectionDb::new(&projection_db_path).await;
        assert!(result.is_ok(), "ProjectionDB should initialize successfully");
    }

    #[tokio::test]
    async fn test_projection_db_update() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let projection_db_path = temp_dir.path().join("projections");

        let db = ProjectionDb::new(&projection_db_path).await.unwrap();
        let result = db.update_projection("key1", b"value1", 1).await;

        assert!(result.is_ok(), "Projection update should succeed");
    }

    #[tokio::test]
    async fn test_projection_db_get() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let projection_db_path = temp_dir.path().join("projections");

        let db = ProjectionDb::new(&projection_db_path).await.unwrap();
        let result = db.get_projection("key1").await;

        assert!(result.is_ok(), "Projection get should succeed");
    }
}
