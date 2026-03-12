// 仕訳検索用Projection
// JournalEntryEventから検索用ReadModelを構築

use javelin_domain::journal_entry::events::JournalEntryEvent;
use serde::{Deserialize, Serialize};

use crate::{error::InfrastructureResult, read::infrastructure::traits::Apply};

/// 仕訳明細ReadModel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JournalEntryLineReadModel {
    pub line_number: u32,
    pub side: String,
    pub account_code: String,
    pub account_name: String,
    pub amount: f64,
    pub description: Option<String>,
}

impl JournalEntryLineReadModel {
    pub fn new(
        line_number: u32,
        side: String,
        account_code: String,
        account_name: String,
        amount: f64,
        description: Option<String>,
    ) -> Self {
        Self { line_number, side, account_code, account_name, amount, description }
    }
}

/// 仕訳検索ReadModel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JournalEntrySearchReadModel {
    pub entry_id: String,
    pub entry_number: Option<String>,
    pub transaction_date: String,
    pub status: String,
    pub lines: Vec<JournalEntryLineReadModel>,
}

impl JournalEntrySearchReadModel {
    pub fn new(
        entry_id: String,
        entry_number: Option<String>,
        transaction_date: String,
        status: String,
        lines: Vec<JournalEntryLineReadModel>,
    ) -> Self {
        Self { entry_id, entry_number, transaction_date, status, lines }
    }

    /// 摘要に指定文字列が含まれるか（部分一致、大文字小文字非区別）
    pub fn contains_description(&self, keyword: &str) -> bool {
        let keyword_lower = keyword.to_lowercase();
        self.lines.iter().any(|line| {
            line.description
                .as_ref()
                .map(|desc| desc.to_lowercase().contains(&keyword_lower))
                .unwrap_or(false)
        })
    }

    /// 勘定科目コードが含まれるか
    pub fn contains_account(&self, account_code: &str) -> bool {
        self.lines.iter().any(|line| line.account_code == account_code)
    }

    /// 借方貸方区分が含まれるか
    pub fn contains_side(&self, side: &str) -> bool {
        self.lines.iter().any(|line| line.side == side)
    }

    /// 金額範囲に該当する明細があるか
    pub fn contains_amount_in_range(
        &self,
        min_amount: Option<f64>,
        max_amount: Option<f64>,
    ) -> bool {
        self.lines.iter().any(|line| {
            let min_ok = min_amount.map(|min| line.amount >= min).unwrap_or(true);
            let max_ok = max_amount.map(|max| line.amount <= max).unwrap_or(true);
            min_ok && max_ok
        })
    }
}

/// 仕訳検索用Projection
///
/// JournalEntryEventを受け取り、検索最適化されたReadModelを構築する。
/// 取引日付、勘定科目、摘要などでインデックス化される。
#[derive(Debug, Clone)]
pub struct JournalEntrySearchProjection {
    entries: Vec<JournalEntrySearchReadModel>,
}

impl JournalEntrySearchProjection {
    /// 新しいProjectionインスタンスを作成
    pub fn new() -> Self {
        // モダンプラクティス: 初期キャパシティを確保
        Self { entries: Vec::with_capacity(100) }
    }

    /// エントリーリストを取得
    pub fn entries(&self) -> &[JournalEntrySearchReadModel] {
        &self.entries
    }

    /// エントリーリストを可変参照で取得
    pub fn entries_mut(&mut self) -> &mut Vec<JournalEntrySearchReadModel> {
        &mut self.entries
    }

    /// 勘定科目名を取得（暫定実装）
    ///
    /// 注意: Projectionは読み取り最適化のため、勘定科目名を非正規化して保持する。
    /// 本来はマスタデータから取得すべきだが、Projection構築時にマスタデータへの
    /// 依存を避けるため、現時点では固定マッピングを使用。
    ///
    /// 将来的な改善案:
    /// 1. AccountMasterのイベントを購読してProjection内にマスタデータをキャッシュ
    /// 2. または、表示時にアプリケーション層でマスタデータと結合
    fn get_account_name(&self, account_code: &str) -> String {
        // 暫定実装: 主要な勘定科目のみマッピング
        match account_code {
            "1000" => "現金".to_string(),
            "1100" => "普通預金".to_string(),
            "1200" => "売掛金".to_string(),
            "2000" => "買掛金".to_string(),
            "2100" => "未払金".to_string(),
            "3000" => "資本金".to_string(),
            "4000" => "売上高".to_string(),
            "5000" => "仕入高".to_string(),
            "6000" => "給料手当".to_string(),
            "7000" => "地代家賃".to_string(),
            _ => format!("勘定科目{}", account_code),
        }
    }

    /// エントリーを検索
    fn find_entry_mut(&mut self, entry_id: &str) -> Option<&mut JournalEntrySearchReadModel> {
        self.entries.iter_mut().find(|e| e.entry_id == entry_id)
    }
}

impl Default for JournalEntrySearchProjection {
    fn default() -> Self {
        Self::new()
    }
}

impl Apply<JournalEntryEvent> for JournalEntrySearchProjection {
    fn apply(&mut self, event: JournalEntryEvent) -> InfrastructureResult<()> {
        match event {
            JournalEntryEvent::DraftCreated { entry_id, transaction_date, lines, .. } => {
                // 明細をReadModelに変換（アカウント名を先に収集）
                let line_models: Vec<JournalEntryLineReadModel> = lines
                    .iter()
                    .map(|line| {
                        let account_name = self.get_account_name(&line.account_code);
                        JournalEntryLineReadModel::new(
                            line.line_number,
                            line.side.clone(),
                            line.account_code.clone(),
                            account_name,
                            line.amount,
                            line.description.clone(),
                        )
                    })
                    .collect();

                // 新しいエントリーを追加
                let read_model = JournalEntrySearchReadModel::new(
                    entry_id,
                    None, // 下書き時点では伝票番号なし
                    transaction_date,
                    "Draft".to_string(),
                    line_models,
                );

                self.entries.push(read_model);
            }

            JournalEntryEvent::DraftUpdated { entry_id, transaction_date, lines, .. } => {
                // 先にアカウント名を収集（不変借用）
                let line_models_opt = lines.as_ref().map(|lines| {
                    lines
                        .iter()
                        .map(|line| {
                            let account_name = self.get_account_name(&line.account_code);
                            JournalEntryLineReadModel::new(
                                line.line_number,
                                line.side.clone(),
                                line.account_code.clone(),
                                account_name,
                                line.amount,
                                line.description.clone(),
                            )
                        })
                        .collect::<Vec<_>>()
                });

                // 既存エントリーを更新（可変借用）
                if let Some(entry) = self.find_entry_mut(&entry_id) {
                    if let Some(date) = transaction_date {
                        entry.transaction_date = date;
                    }
                    if let Some(line_models) = line_models_opt {
                        entry.lines = line_models;
                    }
                }
            }

            JournalEntryEvent::ApprovalRequested { entry_id, .. } => {
                if let Some(entry) = self.find_entry_mut(&entry_id) {
                    entry.status = "PendingApproval".to_string();
                }
            }

            JournalEntryEvent::Rejected { entry_id, .. } => {
                if let Some(entry) = self.find_entry_mut(&entry_id) {
                    entry.status = "Draft".to_string();
                }
            }

            JournalEntryEvent::Posted { entry_id, entry_number, .. } => {
                if let Some(entry) = self.find_entry_mut(&entry_id) {
                    entry.status = "Posted".to_string();
                    entry.entry_number = Some(entry_number);
                }
            }

            JournalEntryEvent::Reversed { entry_id, .. } => {
                if let Some(entry) = self.find_entry_mut(&entry_id) {
                    entry.status = "Reversed".to_string();
                }
            }

            JournalEntryEvent::Corrected { entry_id, .. } => {
                if let Some(entry) = self.find_entry_mut(&entry_id) {
                    entry.status = "Corrected".to_string();
                }
            }

            JournalEntryEvent::Closed { entry_id, .. } => {
                if let Some(entry) = self.find_entry_mut(&entry_id) {
                    entry.status = "Closed".to_string();
                }
            }

            JournalEntryEvent::Reopened { entry_id, .. } => {
                if let Some(entry) = self.find_entry_mut(&entry_id) {
                    entry.status = "Posted".to_string();
                }
            }

            JournalEntryEvent::Deleted { entry_id, .. } => {
                if let Some(entry) = self.find_entry_mut(&entry_id) {
                    entry.status = "Deleted".to_string();
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use javelin_domain::journal_entry::events::JournalEntryLineDto;

    use super::*;

    #[test]
    fn test_draft_created_projection() {
        let mut projection = JournalEntrySearchProjection::new();

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
                description: Some("売上入金".to_string()),
            },
            JournalEntryLineDto {
                line_number: 2,
                side: "Credit".to_string(),
                account_code: "4000".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 100000.0,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: Some("商品販売".to_string()),
            },
        ];

        let event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines,
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        projection.apply(event).unwrap();

        let entries = projection.entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].entry_id, "JE001");
        assert_eq!(entries[0].status, "Draft");
        assert_eq!(entries[0].transaction_date, "2024-01-01");
        assert_eq!(entries[0].lines.len(), 2);
        assert_eq!(entries[0].lines[0].account_code, "1000");
        assert_eq!(entries[0].lines[0].account_name, "現金");
        assert_eq!(entries[0].lines[1].account_code, "4000");
        assert_eq!(entries[0].lines[1].account_name, "売上高");
    }

    #[test]
    fn test_status_transitions() {
        let mut projection = JournalEntrySearchProjection::new();

        // Draft作成
        let event1 = JournalEntryEvent::DraftCreated {
            entry_id: "JE002".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V002".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };
        projection.apply(event1).unwrap();
        assert_eq!(projection.entries()[0].status, "Draft");

        // 承認申請
        let event2 = JournalEntryEvent::ApprovalRequested {
            entry_id: "JE002".to_string(),
            requested_by: "user1".to_string(),
            requested_at: Utc::now(),
        };
        projection.apply(event2).unwrap();
        assert_eq!(projection.entries()[0].status, "PendingApproval");

        // 記帳
        let event3 = JournalEntryEvent::Posted {
            entry_id: "JE002".to_string(),
            entry_number: "EN-2024-001".to_string(),
            posted_by: "approver1".to_string(),
            posted_at: Utc::now(),
        };
        projection.apply(event3).unwrap();
        assert_eq!(projection.entries()[0].status, "Posted");
        assert_eq!(projection.entries()[0].entry_number, Some("EN-2024-001".to_string()));
    }

    #[test]
    fn test_draft_updated_projection() {
        let mut projection = JournalEntrySearchProjection::new();

        // Draft作成
        let event1 = JournalEntryEvent::DraftCreated {
            entry_id: "JE003".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V003".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };
        projection.apply(event1).unwrap();

        // Draft更新
        let new_lines = vec![JournalEntryLineDto {
            line_number: 1,
            side: "Debit".to_string(),
            account_code: "1000".to_string(),
            sub_account_code: None,
            department_code: None,
            amount: 50000.0,
            currency: "JPY".to_string(),
            tax_type: "NonTaxable".to_string(),
            tax_amount: 0.0,
            description: Some("修正後".to_string()),
        }];

        let event2 = JournalEntryEvent::DraftUpdated {
            entry_id: "JE003".to_string(),
            transaction_date: Some("2024-01-02".to_string()),
            voucher_number: None,
            lines: Some(new_lines),
            updated_by: "user1".to_string(),
            updated_at: Utc::now(),
        };
        projection.apply(event2).unwrap();

        let entries = projection.entries();
        assert_eq!(entries[0].transaction_date, "2024-01-02");
        assert_eq!(entries[0].lines.len(), 1);
        assert_eq!(entries[0].lines[0].amount, 50000.0);
    }

    #[test]
    fn test_reversed_and_corrected() {
        let mut projection = JournalEntrySearchProjection::new();

        // Draft作成 → 記帳
        let event1 = JournalEntryEvent::DraftCreated {
            entry_id: "JE004".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V004".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };
        projection.apply(event1).unwrap();

        let event2 = JournalEntryEvent::Posted {
            entry_id: "JE004".to_string(),
            entry_number: "EN-2024-002".to_string(),
            posted_by: "approver1".to_string(),
            posted_at: Utc::now(),
        };
        projection.apply(event2).unwrap();

        // 取消
        let event3 = JournalEntryEvent::Reversed {
            entry_id: "JE004".to_string(),
            original_id: "JE004".to_string(),
            reason: "誤記".to_string(),
            reversed_by: "user1".to_string(),
            reversed_at: Utc::now(),
        };
        projection.apply(event3).unwrap();
        assert_eq!(projection.entries()[0].status, "Reversed");

        // 修正
        let event4 = JournalEntryEvent::Corrected {
            entry_id: "JE004".to_string(),
            reversed_id: "JE004".to_string(),
            reason: "修正".to_string(),
            corrected_by: "user1".to_string(),
            corrected_at: Utc::now(),
        };
        projection.apply(event4).unwrap();
        assert_eq!(projection.entries()[0].status, "Corrected");
    }

    #[test]
    fn test_closed_and_reopened() {
        let mut projection = JournalEntrySearchProjection::new();

        // Draft作成 → 記帳
        let event1 = JournalEntryEvent::DraftCreated {
            entry_id: "JE005".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V005".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };
        projection.apply(event1).unwrap();

        let event2 = JournalEntryEvent::Posted {
            entry_id: "JE005".to_string(),
            entry_number: "EN-2024-003".to_string(),
            posted_by: "approver1".to_string(),
            posted_at: Utc::now(),
        };
        projection.apply(event2).unwrap();

        // 締め
        let event3 = JournalEntryEvent::Closed {
            entry_id: "JE005".to_string(),
            closed_by: "admin1".to_string(),
            closed_at: Utc::now(),
        };
        projection.apply(event3).unwrap();
        assert_eq!(projection.entries()[0].status, "Closed");

        // 再オープン
        let event4 = JournalEntryEvent::Reopened {
            entry_id: "JE005".to_string(),
            reason: "修正必要".to_string(),
            reopened_by: "admin1".to_string(),
            reopened_at: Utc::now(),
        };
        projection.apply(event4).unwrap();
        assert_eq!(projection.entries()[0].status, "Posted");
    }

    #[test]
    fn test_deleted() {
        let mut projection = JournalEntrySearchProjection::new();

        // Draft作成
        let event1 = JournalEntryEvent::DraftCreated {
            entry_id: "JE006".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V006".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };
        projection.apply(event1).unwrap();

        // 削除
        let event2 = JournalEntryEvent::Deleted {
            entry_id: "JE006".to_string(),
            deleted_by: "user1".to_string(),
            deleted_at: Utc::now(),
        };
        projection.apply(event2).unwrap();
        assert_eq!(projection.entries()[0].status, "Deleted");
    }
}
