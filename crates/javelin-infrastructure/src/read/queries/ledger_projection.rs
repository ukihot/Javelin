// LedgerProjection実装
// 元帳表示用のReadModel

use javelin_domain::financial_close::journal_entry::events::JournalEntryEvent;
use serde::{Deserialize, Serialize};

use crate::{
    error::InfrastructureResult,
    event_stream::StoredEvent,
    projection_trait::{Apply, ProjectionStrategy, ToReadModel},
};

/// 元帳エントリReadModel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerEntryReadModel {
    pub account_code: String,
    pub transaction_date: String,
    pub entry_number: String,
    pub description: String,
    pub debit_amount: f64,
    pub credit_amount: f64,
    pub balance: f64,
}

/// 元帳Projection
///
/// JournalEntryEventを受け取り、元帳表示用の
/// ReadModelを構築する。
#[derive(Debug, Clone)]
pub struct LedgerProjection {
    entries: Vec<LedgerEntryReadModel>,
    balances: std::collections::HashMap<String, f64>,
    // 仕訳明細をキャッシュ（entry_id -> lines）
    entry_lines_cache: std::collections::HashMap<
        String,
        Vec<javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto>,
    >,
    // 仕訳の取引日をキャッシュ（entry_id -> transaction_date）
    entry_transaction_date_cache: std::collections::HashMap<String, String>,
    // 仕訳の摘要をキャッシュ（entry_id -> description）
    entry_description_cache: std::collections::HashMap<String, String>,
}

impl LedgerProjection {
    /// 新しいProjectionインスタンスを作成
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            balances: std::collections::HashMap::new(),
            entry_lines_cache: std::collections::HashMap::new(),
            entry_transaction_date_cache: std::collections::HashMap::new(),
            entry_description_cache: std::collections::HashMap::new(),
        }
    }

    /// 勘定科目の残高を更新
    fn update_balance(&mut self, account_code: &str, debit: f64, credit: f64) -> f64 {
        let balance = self.balances.entry(account_code.to_string()).or_insert(0.0);
        *balance += debit - credit;
        *balance
    }

    /// 記帳済イベントから元帳エントリを作成
    fn create_ledger_entries(
        &mut self,
        entry_number: &str,
        transaction_date: &str,
        description: &str,
        lines: &[javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto],
    ) {
        use javelin_domain::financial_close::journal_entry::values::DebitCredit;

        for line in lines {
            let side = line.side.parse::<DebitCredit>().ok();
            let (debit, credit) = match side {
                Some(DebitCredit::Debit) => (line.amount, 0.0),
                Some(DebitCredit::Credit) => (0.0, line.amount),
                None => (0.0, 0.0),
            };

            let balance = self.update_balance(&line.account_code, debit, credit);

            self.entries.push(LedgerEntryReadModel {
                account_code: line.account_code.clone(),
                transaction_date: transaction_date.to_string(),
                entry_number: entry_number.to_string(),
                description: description.to_string(),
                debit_amount: debit,
                credit_amount: credit,
                balance,
            });
        }
    }

    /// 取消仕訳から元帳エントリを作成（逆仕訳）
    fn create_reversal_entries(
        &mut self,
        entry_number: &str,
        transaction_date: &str,
        description: &str,
        lines: &[javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto],
    ) {
        use javelin_domain::financial_close::journal_entry::values::DebitCredit;

        for line in lines {
            let side = line.side.parse::<DebitCredit>().ok();
            // 取消なので借方と貸方を逆転
            let (debit, credit) = match side {
                Some(DebitCredit::Debit) => (0.0, line.amount),
                Some(DebitCredit::Credit) => (line.amount, 0.0),
                None => (0.0, 0.0),
            };

            let balance = self.update_balance(&line.account_code, debit, credit);

            self.entries.push(LedgerEntryReadModel {
                account_code: line.account_code.clone(),
                transaction_date: transaction_date.to_string(),
                entry_number: entry_number.to_string(),
                description: format!("取消: {}", description),
                debit_amount: debit,
                credit_amount: credit,
                balance,
            });
        }
    }

    /// エントリ一覧を取得
    pub fn entries(&self) -> &[LedgerEntryReadModel] {
        &self.entries
    }

    /// 勘定科目別の残高を取得
    pub fn balance(&self, account_code: &str) -> f64 {
        *self.balances.get(account_code).unwrap_or(&0.0)
    }
}

impl Default for LedgerProjection {
    fn default() -> Self {
        Self::new()
    }
}

impl Apply<JournalEntryEvent> for LedgerProjection {
    fn apply(&mut self, event: JournalEntryEvent) -> InfrastructureResult<()> {
        match event {
            // DraftCreatedで明細、取引日、摘要をキャッシュ
            JournalEntryEvent::DraftCreated { entry_id, transaction_date, lines, .. } => {
                self.entry_lines_cache.insert(entry_id.clone(), lines);
                self.entry_transaction_date_cache.insert(entry_id.clone(), transaction_date);
            }
            // DraftUpdatedで明細を更新
            JournalEntryEvent::DraftUpdated { entry_id, lines: Some(lines), .. } => {
                self.entry_lines_cache.insert(entry_id, lines);
            }
            // 記帳時に元帳に反映
            JournalEntryEvent::Posted { entry_id, entry_number, .. } => {
                if let Some(lines) = self.entry_lines_cache.get(&entry_id).cloned() {
                    let transaction_date = self
                        .entry_transaction_date_cache
                        .get(&entry_id)
                        .cloned()
                        .unwrap_or_else(|| "1900-01-01".to_string());
                    let description = self
                        .entry_description_cache
                        .get(&entry_id)
                        .cloned()
                        .unwrap_or_else(|| "記帳済".to_string());

                    self.create_ledger_entries(
                        &entry_number,
                        &transaction_date,
                        &description,
                        &lines,
                    );
                }
            }
            // 取消時は逆仕訳を元帳に反映
            JournalEntryEvent::Reversed { entry_id, original_id, reversed_at, reason, .. } => {
                if let Some(lines) = self.entry_lines_cache.get(&original_id).cloned() {
                    // 元の仕訳の明細を使って逆仕訳を作成
                    self.create_reversal_entries(
                        &entry_id,
                        &reversed_at.format("%Y-%m-%d").to_string(),
                        &reason,
                        &lines,
                    );
                }
            }
            // Deletedでキャッシュをクリア
            JournalEntryEvent::Deleted { entry_id, .. } => {
                self.entry_lines_cache.remove(&entry_id);
                self.entry_transaction_date_cache.remove(&entry_id);
                self.entry_description_cache.remove(&entry_id);
            }
            _ => {
                // その他のイベントは元帳に影響しない
            }
        }

        Ok(())
    }
}

impl ToReadModel for LedgerProjection {
    type ReadModel = Vec<LedgerEntryReadModel>;

    fn to_read_model(&self) -> Self::ReadModel {
        self.entries.clone()
    }
}

/// LedgerProjection戦略
pub struct LedgerProjectionStrategy;

impl ProjectionStrategy for LedgerProjectionStrategy {
    fn should_update(&self, event: &StoredEvent) -> bool {
        // 記帳済と取消のみ元帳に反映
        event.event_type == "Posted" || event.event_type == "Reversed"
    }

    fn batch_size(&self) -> usize {
        100
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn test_ledger_projection_new() {
        let projection = LedgerProjection::new();
        assert_eq!(projection.entries().len(), 0);
    }

    #[test]
    fn test_update_balance() {
        let mut projection = LedgerProjection::new();

        // 借方100,000
        let balance1 = projection.update_balance("1000", 100000.0, 0.0);
        assert_eq!(balance1, 100000.0);

        // 貸方50,000
        let balance2 = projection.update_balance("1000", 0.0, 50000.0);
        assert_eq!(balance2, 50000.0);

        // 残高確認
        assert_eq!(projection.balance("1000"), 50000.0);
    }

    #[test]
    fn test_create_ledger_entries() {
        use javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto;

        let mut projection = LedgerProjection::new();

        let lines = vec![
            JournalEntryLineDto {
                line_number: 1,
                side: "Debit".to_string(),
                account_code: "1000".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 100000.0,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
            JournalEntryLineDto {
                line_number: 2,
                side: "Credit".to_string(),
                account_code: "2000".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 100000.0,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
        ];

        projection.create_ledger_entries("EN-2024-001", "2024-01-01", "Test entry", &lines);

        assert_eq!(projection.entries().len(), 2);
        assert_eq!(projection.balance("1000"), 100000.0);
        assert_eq!(projection.balance("2000"), -100000.0);
    }

    #[test]
    fn test_create_reversal_entries() {
        use javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto;

        let mut projection = LedgerProjection::new();

        let lines = vec![JournalEntryLineDto {
            line_number: 1,
            side: "Debit".to_string(),
            account_code: "1000".to_string(),
            sub_account_code: None,
            department_code: None,
            amount: 100000.0,
            currency: "JPY".to_string(),
            tax_type: "NonTaxable".to_string(),
            tax_amount: 0.0,
            description: None,
        }];

        // 元仕訳
        projection.create_ledger_entries("EN-2024-001", "2024-01-01", "Original", &lines);
        assert_eq!(projection.balance("1000"), 100000.0);

        // 取消仕訳
        projection.create_reversal_entries("EN-2024-002", "2024-01-02", "Original", &lines);
        assert_eq!(projection.balance("1000"), 0.0);
    }

    #[test]
    fn test_apply_draft_created() {
        use javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto;

        let mut projection = LedgerProjection::new();

        let lines = vec![JournalEntryLineDto {
            line_number: 1,
            side: "Debit".to_string(),
            account_code: "1000".to_string(),
            sub_account_code: None,
            department_code: None,
            amount: 100000.0,
            currency: "JPY".to_string(),
            tax_type: "NonTaxable".to_string(),
            tax_amount: 0.0,
            description: None,
        }];

        let event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines: lines.clone(),
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        projection.apply(event).unwrap();

        // DraftCreatedでは元帳に反映されない（キャッシュのみ）
        assert_eq!(projection.entries().len(), 0);
        assert!(projection.entry_lines_cache.contains_key("JE001"));
    }

    #[test]
    fn test_apply_posted() {
        use javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto;

        let mut projection = LedgerProjection::new();

        let lines = vec![
            JournalEntryLineDto {
                line_number: 1,
                side: "Debit".to_string(),
                account_code: "1000".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 100000.0,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
            JournalEntryLineDto {
                line_number: 2,
                side: "Credit".to_string(),
                account_code: "2000".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 100000.0,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
        ];

        // DraftCreatedでキャッシュ
        let draft_event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines,
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };
        projection.apply(draft_event).unwrap();

        // Postedで元帳に反映
        let posted_event = JournalEntryEvent::Posted {
            entry_id: "JE001".to_string(),
            entry_number: "EN-2024-001".to_string(),
            posted_by: "approver1".to_string(),
            posted_at: Utc::now(),
        };
        projection.apply(posted_event).unwrap();

        assert_eq!(projection.entries().len(), 2);
        assert_eq!(projection.balance("1000"), 100000.0);
        assert_eq!(projection.balance("2000"), -100000.0);
    }

    #[test]
    fn test_apply_reversed() {
        use javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto;

        let mut projection = LedgerProjection::new();

        let lines = vec![JournalEntryLineDto {
            line_number: 1,
            side: "Debit".to_string(),
            account_code: "1000".to_string(),
            sub_account_code: None,
            department_code: None,
            amount: 100000.0,
            currency: "JPY".to_string(),
            tax_type: "NonTaxable".to_string(),
            tax_amount: 0.0,
            description: None,
        }];

        // 元仕訳をキャッシュ
        let draft_event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines,
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };
        projection.apply(draft_event).unwrap();

        // 記帳
        let posted_event = JournalEntryEvent::Posted {
            entry_id: "JE001".to_string(),
            entry_number: "EN-2024-001".to_string(),
            posted_by: "approver1".to_string(),
            posted_at: Utc::now(),
        };
        projection.apply(posted_event).unwrap();
        assert_eq!(projection.balance("1000"), 100000.0);

        // 取消
        let reversed_event = JournalEntryEvent::Reversed {
            entry_id: "JE002".to_string(),
            original_id: "JE001".to_string(),
            reason: "Error".to_string(),
            reversed_by: "user1".to_string(),
            reversed_at: Utc::now(),
        };
        projection.apply(reversed_event).unwrap();

        // 取消仕訳が追加され、残高が0になる
        assert_eq!(projection.entries().len(), 2);
        assert_eq!(projection.balance("1000"), 0.0);
    }

    #[test]
    fn test_apply_deleted() {
        use javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto;

        let mut projection = LedgerProjection::new();

        let lines = vec![JournalEntryLineDto {
            line_number: 1,
            side: "Debit".to_string(),
            account_code: "1000".to_string(),
            sub_account_code: None,
            department_code: None,
            amount: 100000.0,
            currency: "JPY".to_string(),
            tax_type: "NonTaxable".to_string(),
            tax_amount: 0.0,
            description: None,
        }];

        // DraftCreatedでキャッシュ
        let draft_event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines,
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };
        projection.apply(draft_event).unwrap();
        assert!(projection.entry_lines_cache.contains_key("JE001"));

        // Deletedでキャッシュクリア
        let deleted_event = JournalEntryEvent::Deleted {
            entry_id: "JE001".to_string(),
            deleted_by: "user1".to_string(),
            deleted_at: Utc::now(),
        };
        projection.apply(deleted_event).unwrap();
        assert!(!projection.entry_lines_cache.contains_key("JE001"));
    }
}
