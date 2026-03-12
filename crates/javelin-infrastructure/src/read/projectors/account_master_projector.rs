// AccountMasterProjector - 勘定科目マスタProjection更新
// 勘定科目マスタ関連イベントを購読し、勘定科目マスタRead Modelを更新

use std::sync::Arc;

use serde_json::Value;

use super::Projector;
use crate::{
    error::{InfrastructureError, InfrastructureResult},
    event_stream::StoredEvent,
    read::infrastructure::db::ProjectionDb,
};

/// 勘定科目マスタProjector
///
/// 勘定科目マスタ関連イベント（AccountMasterCreated, AccountMasterUpdated等）を購読し、
/// 勘定科目マスタProjectionを更新する。
pub struct AccountMasterProjector {
    projection_db: Arc<ProjectionDb>,
}

impl AccountMasterProjector {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection_db }
    }

    /// 勘定科目マスタキーを生成
    fn account_key(code: &str) -> String {
        format!("account_master:{}", code)
    }
}

impl Projector for AccountMasterProjector {
    fn event_types(&self) -> Vec<&'static str> {
        vec!["AccountMasterCreated", "AccountMasterUpdated", "AccountMasterDeleted"]
    }

    async fn project(&self, event: &StoredEvent) -> InfrastructureResult<()> {
        println!(
            "✓ AccountMasterProjector: Processing {} (seq: {})",
            event.event_type, event.global_sequence
        );

        let event_data: Value = serde_json::from_slice(&event.payload)
            .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;

        match event.event_type.as_str() {
            "AccountMasterCreated" | "AccountMasterUpdated" => {
                let code = event_data["code"].as_str().ok_or_else(|| {
                    InfrastructureError::ValidationFailed("Missing code".to_string())
                })?;
                let name = event_data["name"].as_str().ok_or_else(|| {
                    InfrastructureError::ValidationFailed("Missing name".to_string())
                })?;
                let account_type = event_data["account_type"].as_str().ok_or_else(|| {
                    InfrastructureError::ValidationFailed("Missing account_type".to_string())
                })?;
                let is_active = event_data["is_active"].as_bool().unwrap_or(true);

                let key = Self::account_key(code);
                let stored_data = serde_json::json!({
                    "code": code,
                    "name": name,
                    "account_type": account_type,
                    "is_active": is_active,
                });

                let data = serde_json::to_vec(&stored_data)
                    .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

                self.projection_db.update_projection(&key, &data, event.global_sequence).await?;
            }
            "AccountMasterDeleted" => {
                let code = event_data["code"].as_str().ok_or_else(|| {
                    InfrastructureError::ValidationFailed("Missing code".to_string())
                })?;

                let key = Self::account_key(code);

                // 削除はis_activeをfalseにする（論理削除）
                if let Some(existing_data) = self.projection_db.get_projection(&key).await? {
                    let mut stored_data: Value = serde_json::from_slice(&existing_data)
                        .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;

                    stored_data["is_active"] = Value::Bool(false);

                    let data = serde_json::to_vec(&stored_data)
                        .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

                    self.projection_db
                        .update_projection(&key, &data, event.global_sequence)
                        .await?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}
