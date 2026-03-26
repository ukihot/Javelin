// JournalEntryProjector - 仕訳一覧Projection更新
// 仕訳関連イベントを購読し、仕訳一覧Read Modelを更新

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::Projector;
use crate::{
    error::{InfrastructureError, InfrastructureResult},
    event_stream::StoredEvent,
    read::infrastructure::db::ProjectionDb,
};

/// 仕訳一覧Projector
///
/// 仕訳関連イベント（DraftCreated, Approved, Rejected等）を購読し、
/// 仕訳一覧Projectionを更新する。
pub struct JournalEntryProjector {
    projection_db: Arc<ProjectionDb>,
}

impl JournalEntryProjector {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection_db }
    }

    /// 仕訳エントリキーを生成
    fn entry_key(entry_id: &str) -> String {
        format!("journal_entry:{}", entry_id)
    }
}

impl Projector for JournalEntryProjector {
    fn event_types(&self) -> Vec<&'static str> {
        vec![
            "DraftCreated",
            "SubmittedForApproval",
            "Approved",
            "Rejected",
            "Updated",
            "Deleted",
            "Corrected",
            "Reversed",
        ]
    }

    async fn project(&self, event: &StoredEvent) -> InfrastructureResult<()> {
        use serde_json::Value;

        println!(
            "✓ JournalEntryProjector: Processing {} (seq: {})",
            event.event_type, event.global_sequence
        );

        // イベントペイロードをデシリアライズ
        let event_data: Value = serde_json::from_slice(&event.payload)
            .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;

        let entry_id = &event.aggregate_id;
        let key = Self::entry_key(entry_id);

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
                                    description: line["description"]
                                        .as_str()
                                        .map(|s| s.to_string()),
                                    partner_id: line["partner_id"].as_str().map(|s| s.to_string()),
                                    external_name: line["external_name"]
                                        .as_str()
                                        .map(|s| s.to_string()),
                                    tracking_number: line["tracking_number"]
                                        .as_str()
                                        .map(|s| s.to_string()),
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                };

                let data = serde_json::to_vec(&stored_entry)
                    .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

                self.projection_db.update_projection(&key, &data, event.global_sequence).await?;
            }
            "SubmittedForApproval" => {
                // ステータスを更新
                if let Some(existing_data) = self.projection_db.get_projection(&key).await? {
                    let mut stored_entry: StoredJournalEntry =
                        serde_json::from_slice(&existing_data).map_err(|e| {
                            InfrastructureError::DeserializationFailed(e.to_string())
                        })?;

                    stored_entry.status = "PendingApproval".to_string();

                    let data = serde_json::to_vec(&stored_entry)
                        .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

                    self.projection_db
                        .update_projection(&key, &data, event.global_sequence)
                        .await?;
                }
            }
            "Approved" => {
                // ステータスを更新
                if let Some(existing_data) = self.projection_db.get_projection(&key).await? {
                    let mut stored_entry: StoredJournalEntry =
                        serde_json::from_slice(&existing_data).map_err(|e| {
                            InfrastructureError::DeserializationFailed(e.to_string())
                        })?;

                    stored_entry.status = "Approved".to_string();
                    stored_entry.approved_by =
                        event_data["approved_by"].as_str().map(|s| s.to_string());
                    stored_entry.approved_at = Some(event.timestamp.clone());
                    stored_entry.entry_number =
                        event_data["entry_number"].as_str().map(|s| s.to_string());

                    let data = serde_json::to_vec(&stored_entry)
                        .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

                    self.projection_db
                        .update_projection(&key, &data, event.global_sequence)
                        .await?;
                }
            }
            "Rejected" => {
                // ステータスを更新
                if let Some(existing_data) = self.projection_db.get_projection(&key).await? {
                    let mut stored_entry: StoredJournalEntry =
                        serde_json::from_slice(&existing_data).map_err(|e| {
                            InfrastructureError::DeserializationFailed(e.to_string())
                        })?;

                    stored_entry.status = "Rejected".to_string();

                    let data = serde_json::to_vec(&stored_entry)
                        .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

                    self.projection_db
                        .update_projection(&key, &data, event.global_sequence)
                        .await?;
                }
            }
            "Updated" => {
                // エントリを更新
                if let Some(existing_data) = self.projection_db.get_projection(&key).await? {
                    let mut stored_entry: StoredJournalEntry =
                        serde_json::from_slice(&existing_data).map_err(|e| {
                            InfrastructureError::DeserializationFailed(e.to_string())
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
                        .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

                    self.projection_db
                        .update_projection(&key, &data, event.global_sequence)
                        .await?;
                }
            }
            "Deleted" => {
                // エントリを削除
                self.projection_db.delete_projection(&key).await?;
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
    description: Option<String>,
    partner_id: Option<String>,
    external_name: Option<String>,
    tracking_number: Option<String>,
}
