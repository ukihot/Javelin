// TrialBalanceProjector - 試算表Projection更新
// 承認済み仕訳イベントを購読し、試算表Read Modelを更新

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Projector;
use crate::{
    error::{InfrastructureError, InfrastructureResult},
    event_stream::StoredEvent,
    read::infrastructure::db::ProjectionDb,
};

/// 試算表Projector
///
/// Approvedイベントを購読し、試算表Projectionを更新する。
/// 勘定科目ごとの借方・貸方合計を集計する。
pub struct TrialBalanceProjector {
    projection_db: Arc<ProjectionDb>,
}

impl TrialBalanceProjector {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection_db }
    }

    /// 試算表キーを生成（年 + 月）
    fn trial_balance_key(year: u32, month: u8) -> String {
        format!("trial_balance:{}:{}", year, month)
    }
}

impl Projector for TrialBalanceProjector {
    fn event_types(&self) -> Vec<&'static str> {
        vec!["Approved"] // 承認済み仕訳のみを試算表に反映
    }

    async fn project(&self, event: &StoredEvent) -> InfrastructureResult<()> {
        println!(
            "✓ TrialBalanceProjector: Processing {} (seq: {})",
            event.event_type, event.global_sequence
        );

        let event_data: Value = serde_json::from_slice(&event.payload)
            .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;

        // 取引日から年月を抽出
        let transaction_date = event_data["transaction_date"].as_str().unwrap_or("2024-01-01");
        let parts: Vec<&str> = transaction_date.split('-').collect();
        let year = parts.first().and_then(|s| s.parse::<u32>().ok()).unwrap_or(2024);
        let month = parts.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(1);

        let trial_balance_key = Self::trial_balance_key(year, month);

        // 既存の試算表データを取得
        let mut trial_balance_data =
            if let Some(data) = self.projection_db.get_projection(&trial_balance_key).await? {
                serde_json::from_slice::<StoredTrialBalanceData>(&data)
                    .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?
            } else {
                StoredTrialBalanceData { entries: vec![] }
            };

        // 仕訳明細から勘定科目ごとに集計
        if let Some(lines) = event_data["lines"].as_array() {
            for line in lines {
                let account_code = line["account_code"].as_str().unwrap_or("");
                let account_name = line["account_name"].as_str().unwrap_or("");
                let side = line["side"].as_str().unwrap_or("");
                let amount = line["amount"].as_f64().unwrap_or(0.0);

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

                // 既存のエントリを検索
                if let Some(entry) =
                    trial_balance_data.entries.iter_mut().find(|e| e.account_code == account_code)
                {
                    // 既存エントリを更新
                    entry.debit_amount += debit_amount;
                    entry.credit_amount += credit_amount;
                } else {
                    // 新規エントリを追加
                    trial_balance_data.entries.push(StoredTrialBalanceEntry {
                        account_code: account_code.to_string(),
                        account_name: account_name.to_string(),
                        debit_amount,
                        credit_amount,
                    });
                }
            }
        }

        // 試算表データを保存
        let data = serde_json::to_vec(&trial_balance_data)
            .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

        self.projection_db
            .update_projection(&trial_balance_key, &data, event.global_sequence)
            .await?;

        Ok(())
    }
}

/// ProjectionDBに保存される試算表データ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredTrialBalanceData {
    entries: Vec<StoredTrialBalanceEntry>,
}

/// ProjectionDBに保存される試算表エントリデータ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredTrialBalanceEntry {
    account_code: String,
    account_name: String,
    debit_amount: f64,
    credit_amount: f64,
}
