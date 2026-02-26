// 現代Rust設計によるSnapshotDB実装
// - const generics によるポリシー型化
// - 型安全なSnapshot戦略

use std::{marker::PhantomData, path::Path, sync::Arc};

use chrono::{DateTime, Utc};
use lmdb::{Database, DatabaseFlags, Environment, Transaction, WriteFlags};
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    types::{AggregateId, Sequence},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub aggregate_id: String,
    pub state: Vec<u8>,
    pub last_sequence: u64,
    pub created_at: String,
}

/// Snapshot戦略Trait - const genericsで型化
pub trait SnapshotPolicyTrait {
    fn should_snapshot(event_count: u64, last_snapshot_time: Option<DateTime<Utc>>) -> bool;
}

/// N回ごとにSnapshot
pub struct EveryNEvents<const N: u64>;

impl<const N: u64> SnapshotPolicyTrait for EveryNEvents<N> {
    fn should_snapshot(event_count: u64, _last_snapshot_time: Option<DateTime<Utc>>) -> bool {
        event_count.is_multiple_of(N)
    }
}

/// 時間ベースSnapshot
pub struct EveryNMinutes<const N: i64>;

impl<const N: i64> SnapshotPolicyTrait for EveryNMinutes<N> {
    fn should_snapshot(_event_count: u64, last_snapshot_time: Option<DateTime<Utc>>) -> bool {
        if let Some(last_time) = last_snapshot_time {
            let elapsed = Utc::now().signed_duration_since(last_time);
            elapsed.num_minutes() >= N
        } else {
            true // 初回Snapshot
        }
    }
}

/// SnapshotDB実装
pub struct SnapshotDb<P: SnapshotPolicyTrait> {
    env: Arc<Environment>,
    snapshot_db: Database,
    _policy: PhantomData<P>,
}

impl<P: SnapshotPolicyTrait> SnapshotDb<P> {
    pub async fn new(path: &Path) -> InfrastructureResult<Self> {
        if !path.exists() {
            tokio::fs::create_dir_all(path).await.map_err(|e| {
                InfrastructureError::ProjectionDbInitFailed {
                    path: path.display().to_string(),
                    source: e,
                }
            })?;
        }

        let env = Environment::new()
            .set_max_dbs(1)
            .set_map_size(500 * 1024 * 1024)
            .open(path)
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        let snapshot_db = env
            .create_db(Some("snapshots"), DatabaseFlags::empty())
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        Ok(Self { env: Arc::new(env), snapshot_db, _policy: PhantomData })
    }

    /// Snapshotを保存すべきか判定
    pub fn should_snapshot(
        &self,
        event_count: u64,
        last_snapshot_time: Option<DateTime<Utc>>,
    ) -> bool {
        P::should_snapshot(event_count, last_snapshot_time)
    }

    /// Snapshotを保存
    pub async fn save_snapshot(
        &self,
        aggregate_id: AggregateId,
        state: &[u8],
        last_sequence: Sequence,
    ) -> InfrastructureResult<()> {
        let snapshot = Snapshot {
            aggregate_id: aggregate_id.to_string(),
            state: state.to_vec(),
            last_sequence: last_sequence.as_u64(),
            created_at: Utc::now().to_rfc3339(),
        };

        let key = aggregate_id.to_string();
        let value = serde_json::to_vec(&snapshot)
            .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

        let env = Arc::clone(&self.env);
        let snapshot_db = self.snapshot_db;

        tokio::task::spawn_blocking(move || {
            let mut txn =
                env.begin_rw_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
            txn.put(snapshot_db, &key.as_bytes(), &value, WriteFlags::empty())
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
            txn.commit().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
            Ok::<_, InfrastructureError>(())
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(())
    }

    /// Snapshotを読み込み
    pub async fn load_snapshot(
        &self,
        aggregate_id: AggregateId,
    ) -> InfrastructureResult<Option<Snapshot>> {
        let env = Arc::clone(&self.env);
        let snapshot_db = self.snapshot_db;
        let key = aggregate_id.to_string();

        let result = tokio::task::spawn_blocking(move || {
            let txn =
                env.begin_ro_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

            match txn.get(snapshot_db, &key.as_bytes()) {
                Ok(bytes) => {
                    let snapshot: Snapshot = serde_json::from_slice(bytes)
                        .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;
                    Ok(Some(snapshot))
                }
                Err(lmdb::Error::NotFound) => Ok(None),
                Err(e) => Err(InfrastructureError::LmdbError(e.to_string())),
            }
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(result)
    }

    /// Snapshotを削除
    pub async fn delete_snapshot(&self, aggregate_id: AggregateId) -> InfrastructureResult<()> {
        let env = Arc::clone(&self.env);
        let snapshot_db = self.snapshot_db;
        let key = aggregate_id.to_string();

        tokio::task::spawn_blocking(move || {
            let mut txn =
                env.begin_rw_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
            txn.del(snapshot_db, &key.as_bytes(), None)
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
            txn.commit().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
            Ok::<_, InfrastructureError>(())
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(())
    }
}

// 型エイリアス - 使いやすさのため
pub type SnapshotEvery100 = SnapshotDb<EveryNEvents<100>>;
pub type SnapshotEvery1000 = SnapshotDb<EveryNEvents<1000>>;
pub type SnapshotEvery60Min = SnapshotDb<EveryNMinutes<60>>;

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_snapshot_every_100() {
        let temp_dir = TempDir::new().unwrap();
        let db = SnapshotEvery100::new(temp_dir.path()).await.unwrap();

        assert!(db.should_snapshot(100, None));
        assert!(db.should_snapshot(200, None));
        assert!(!db.should_snapshot(150, None));
    }

    #[tokio::test]
    async fn test_snapshot_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let db = SnapshotEvery100::new(temp_dir.path()).await.unwrap();

        let agg_id = AggregateId::parse("agg-001").unwrap();
        let seq = Sequence::new(100);

        db.save_snapshot(agg_id, b"state data", seq).await.unwrap();

        let snapshot = db.load_snapshot(agg_id).await.unwrap();
        assert!(snapshot.is_some());

        let snap = snapshot.unwrap();
        assert_eq!(snap.aggregate_id, "agg-001");
        assert_eq!(snap.last_sequence, 100);
    }
}
