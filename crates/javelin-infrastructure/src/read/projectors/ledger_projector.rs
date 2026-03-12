// LedgerProjector - 元帳Projection更新
// 承認済み仕訳イベントを購読し、元帳Read Modelを更新

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Projector;
use crate::{
    error::{InfrastructureError, InfrastructureResult},
    event_stream::StoredEvent,
    read::infrastructure::db::ProjectionDb,
};

/// 元帳Projector
///
/// Approvedイベントを購読し、元帳Projectionを更新する。
/// 承認済み仕訳のみが元帳に転記される。
pub struct LedgerProjector {
    projection_db: Arc<ProjectionDb>,
}

impl LedgerProjector {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection_db }
    }

    /// 元帳キーを生成（勘定科目コード + 年 + 月）
    fn ledger_key(account_code: &str, year: u32, month: u8) -> String {
        format!("ledger:{}:{}:{}", account_code, year, month)
    }
}

impl Projector for LedgerProjector {
    fn event_types(&self) -> Vec<&'static str> {
        vec!["Approved"] // 承認済み仕訳のみを元帳に転記
    }

    async fn project(&self, event: &StoredEvent) -> InfrastructureResult<()> {
        println!(
            "✓ LedgerProjector: Processing {} (seq: {})",
            event.event_type, event.global_sequence
        );

        let event_data: Value = serde_json::from_slice(&event.payload)
            .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;

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

                let ledger_key = Self::ledger_key(account_code, year, month);

                // 既存の元帳データを取得
                let mut ledger_data = if let Some(data) =
                    self.projection_db.get_projection(&ledger_key).await?
                {
                    serde_json::from_slice::<StoredLedgerData>(&data)
                        .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?
                } else {
                    StoredLedgerData {
                        account_name: account_name.to_string(),
                        opening_balance: 0.0,
                        entries: vec![],
                    }
                };

                // 新しいエントリを追加
                use javelin_domain::journal_entry::values::DebitCredit;
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
                    .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

                self.projection_db
                    .update_projection(&ledger_key, &data, event.global_sequence)
                    .await?;
            }
        }

        Ok(())
    }
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
