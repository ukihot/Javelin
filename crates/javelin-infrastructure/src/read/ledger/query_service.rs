// LedgerQueryServiceImpl - 元帳照会サービス実装（Infrastructure層）
// ProjectionDBから元帳データを取得（CQRS読み取り側）

use std::{collections::HashMap, sync::Arc};

use javelin_application::{
    error::{ApplicationError, ApplicationResult},
    query_service::ledger_query_service::{
        GetLedgerQuery, GetTrialBalanceQuery, LedgerEntry, LedgerQueryService, LedgerResult,
        TrialBalanceResult,
    },
};

use crate::read::{infrastructure::db::ProjectionDb, ledger::projection::LedgerEntryReadModel};

/// LedgerQueryService実装
///
/// ProjectionDBから元帳データを取得する。
/// CQRS原則: クエリサービスはProjectionDB（読み取り最適化）を使用
pub struct LedgerQueryServiceImpl {
    projection_db: Arc<ProjectionDb>,
    /// 勘定科目マスタのキャッシュ（code -> name）
    /// 注意: 本来はマスタデータローダーを使用すべきだが、
    /// 簡易実装として固定マッピングを使用
    account_names: HashMap<String, String>,
}

impl LedgerQueryServiceImpl {
    /// 新しいインスタンスを作成
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        // 主要な勘定科目のマッピングを初期化
        let mut account_names = HashMap::new();
        account_names.insert("1000".to_string(), "現金".to_string());
        account_names.insert("1100".to_string(), "普通預金".to_string());
        account_names.insert("1200".to_string(), "売掛金".to_string());
        account_names.insert("2000".to_string(), "買掛金".to_string());
        account_names.insert("2100".to_string(), "未払金".to_string());
        account_names.insert("3000".to_string(), "資本金".to_string());
        account_names.insert("4000".to_string(), "売上高".to_string());
        account_names.insert("5000".to_string(), "仕入高".to_string());
        account_names.insert("6000".to_string(), "給料手当".to_string());
        account_names.insert("7000".to_string(), "地代家賃".to_string());

        Self { projection_db, account_names }
    }

    /// 勘定科目名を取得
    fn get_account_name(&self, account_code: &str) -> String {
        self.account_names
            .get(account_code)
            .cloned()
            .unwrap_or_else(|| format!("勘定科目{}", account_code))
    }

    /// ProjectionDBから元帳エントリを取得
    async fn get_ledger_entries(&self) -> ApplicationResult<Vec<LedgerEntryReadModel>> {
        let mut entries = Vec::with_capacity(1000);

        // ProjectionDBから元帳エントリを取得
        // キー形式: "ledger:{account_code}:{sequence}"
        // 簡易実装: 全勘定科目・全シーケンスをスキャン
        for account_code in
            &["1000", "1100", "1200", "2000", "2100", "3000", "4000", "5000", "6000", "7000"]
        {
            for seq in 0..10000 {
                let key = format!("ledger:{}:{:08}", account_code, seq);

                if let Some(data) = self
                    .projection_db
                    .get_projection(&key)
                    .await
                    .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?
                {
                    let entry: LedgerEntryReadModel = serde_json::from_slice(&data)
                        .map_err(|e| ApplicationError::ProjectionDatabaseError(e.to_string()))?;

                    entries.push(entry);
                } else {
                    // この勘定科目のデータが終わった
                    break;
                }
            }
        }

        Ok(entries)
    }
}

impl LedgerQueryService for LedgerQueryServiceImpl {
    async fn get_ledger(&self, query: GetLedgerQuery) -> ApplicationResult<LedgerResult> {
        // ProjectionDBから元帳エントリを取得
        let all_entries = self.get_ledger_entries().await?;

        // 勘定科目でフィルタリング
        let mut filtered_entries: Vec<LedgerEntryReadModel> = all_entries
            .into_iter()
            .filter(|entry| entry.account_code == query.account_code)
            .collect();

        // 日付範囲でフィルタリング
        if let Some(ref from_date) = query.from_date {
            filtered_entries.retain(|entry| entry.transaction_date >= *from_date);
        }
        if let Some(ref to_date) = query.to_date {
            filtered_entries.retain(|entry| entry.transaction_date <= *to_date);
        }

        // 取引日付でソート
        filtered_entries.sort_by(|a, b| a.transaction_date.cmp(&b.transaction_date));

        // 期首残高を計算（フィルタ前の最初のエントリの残高 - その借方貸方差額）
        let opening_balance = if let Some(first_entry) = filtered_entries.first() {
            first_entry.balance - (first_entry.debit_amount - first_entry.credit_amount)
        } else {
            0.0
        };

        // ページネーション適用
        let offset = query.offset.unwrap_or(0) as usize;
        let limit = query.limit.unwrap_or(100) as usize;
        let paginated_entries: Vec<&LedgerEntryReadModel> =
            filtered_entries.iter().skip(offset).take(limit).collect();

        // 借方合計、貸方合計を計算
        let mut total_debit = 0.0;
        let mut total_credit = 0.0;
        for entry in &paginated_entries {
            total_debit += entry.debit_amount;
            total_credit += entry.credit_amount;
        }

        // 期末残高（最後のエントリの残高）
        let closing_balance =
            paginated_entries.last().map(|entry| entry.balance).unwrap_or(opening_balance);

        // LedgerEntryに変換
        let entries: Vec<LedgerEntry> = paginated_entries
            .iter()
            .map(|entry| LedgerEntry {
                transaction_date: entry.transaction_date.clone(),
                entry_number: entry.entry_number.clone(),
                entry_id: entry.entry_number.clone(), // entry_idがないのでentry_numberを使用
                description: entry.description.clone(),
                debit_amount: entry.debit_amount,
                credit_amount: entry.credit_amount,
                balance: entry.balance,
            })
            .collect();

        Ok(LedgerResult {
            account_code: query.account_code.clone(),
            account_name: self.get_account_name(&query.account_code),
            opening_balance,
            entries,
            closing_balance,
            total_debit,
            total_credit,
        })
    }

    async fn get_trial_balance(
        &self,
        query: GetTrialBalanceQuery,
    ) -> ApplicationResult<TrialBalanceResult> {
        use std::collections::HashMap;

        use javelin_application::query_service::TrialBalanceEntry;

        // ProjectionDBから元帳エントリを取得
        let all_entries = self.get_ledger_entries().await?;

        // 期間でフィルタリング（YYYY-MM形式）
        let period_str = format!("{:04}-{:02}", query.period_year, query.period_month);
        let filtered_entries: Vec<LedgerEntryReadModel> = all_entries
            .into_iter()
            .filter(|entry| entry.transaction_date.starts_with(&period_str))
            .collect();

        // 勘定科目ごとに集計
        let mut account_map: HashMap<String, (f64, f64, f64, f64)> = HashMap::new();
        for entry in filtered_entries {
            let (_opening, debit, credit, closing) =
                account_map.entry(entry.account_code.clone()).or_insert((0.0, 0.0, 0.0, 0.0));
            *debit += entry.debit_amount;
            *credit += entry.credit_amount;
            *closing = entry.balance; // 最後のエントリの残高が期末残高
        }

        // 期首残高を計算（期末残高 - 借方 + 貸方）
        for (_account_code, (opening, debit, credit, closing)) in account_map.iter_mut() {
            *opening = *closing - *debit + *credit;
        }

        // TrialBalanceEntryに変換
        let mut entries: Vec<TrialBalanceEntry> = account_map
            .into_iter()
            .map(
                |(
                    account_code,
                    (opening_balance, debit_amount, credit_amount, closing_balance),
                )| {
                    TrialBalanceEntry {
                        account_code: account_code.clone(),
                        account_name: self.get_account_name(&account_code),
                        opening_balance,
                        debit_amount,
                        credit_amount,
                        closing_balance,
                    }
                },
            )
            .collect();

        // 勘定科目コードでソート
        entries.sort_by(|a, b| a.account_code.cmp(&b.account_code));

        // 借貸合計を計算
        let mut total_debit = 0.0;
        let mut total_credit = 0.0;
        for entry in &entries {
            total_debit += entry.debit_amount;
            total_credit += entry.credit_amount;
        }

        Ok(TrialBalanceResult {
            period_year: query.period_year,
            period_month: query.period_month,
            entries,
            total_debit,
            total_credit,
        })
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_get_ledger() {
        let temp_dir = TempDir::new().unwrap();
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
        let service = LedgerQueryServiceImpl::new(projection_db);

        let query = GetLedgerQuery {
            account_code: "1001".to_string(),
            from_date: None,
            to_date: None,
            limit: None,
            offset: None,
        };

        let result = service.get_ledger(query).await.unwrap();
        assert_eq!(result.account_code, "1001");
        assert_eq!(result.entries.len(), 0);
    }

    #[tokio::test]
    async fn test_get_trial_balance() {
        let temp_dir = TempDir::new().unwrap();
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
        let service = LedgerQueryServiceImpl::new(projection_db);

        let query = GetTrialBalanceQuery { period_year: 2024, period_month: 1 };

        let result = service.get_trial_balance(query).await.unwrap();
        assert_eq!(result.period_year, 2024);
        assert_eq!(result.period_month, 1);
        assert_eq!(result.entries.len(), 0);
    }
}
