// JournalEntryProjection実装
// 仕訳一覧表示用のReadModel

use javelin_domain::financial_close::journal_entry::events::JournalEntryEvent;
use serde::{Deserialize, Serialize};

use crate::{
    error::InfrastructureResult,
    event_stream::StoredEvent,
    projection_trait::{Apply, ProjectionStrategy, ToReadModel},
};

/// 仕訳一覧用ReadModel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JournalEntryReadModel {
    pub entry_id: String,
    pub entry_number: Option<String>,
    pub status: String,
    pub transaction_date: String,
    pub voucher_number: String,
    pub total_debit: f64,
    pub total_credit: f64,
    pub created_by: String,
    pub created_at: String,
    pub updated_by: Option<String>,
    pub updated_at: Option<String>,
}

/// 仕訳Projection
///
/// JournalEntryEventを受け取り、仕訳一覧表示用の
/// ReadModelを構築する。
#[derive(Debug, Clone)]
pub struct JournalEntryProjection {
    entry_id: String,
    entry_number: Option<String>,
    status: String,
    transaction_date: String,
    voucher_number: String,
    total_debit: f64,
    total_credit: f64,
    created_by: String,
    created_at: String,
    updated_by: Option<String>,
    updated_at: Option<String>,
}

impl JournalEntryProjection {
    /// 新しいProjectionインスタンスを作成
    pub fn new(entry_id: String) -> Self {
        Self {
            entry_id,
            entry_number: None,
            status: "Draft".to_string(),
            transaction_date: String::new(),
            voucher_number: String::new(),
            total_debit: 0.0,
            total_credit: 0.0,
            created_by: String::new(),
            created_at: String::new(),
            updated_by: None,
            updated_at: None,
        }
    }

    /// 明細から合計金額を計算
    fn calculate_totals(
        lines: &[javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto],
    ) -> (f64, f64) {
        let mut total_debit = 0.0;
        let mut total_credit = 0.0;

        for line in lines {
            match line.side.as_str() {
                "Debit" => total_debit += line.amount,
                "Credit" => total_credit += line.amount,
                _ => {}
            }
        }

        (total_debit, total_credit)
    }
}

impl Apply<JournalEntryEvent> for JournalEntryProjection {
    fn apply(&mut self, event: JournalEntryEvent) -> InfrastructureResult<()> {
        match event {
            JournalEntryEvent::DraftCreated {
                entry_id,
                transaction_date,
                voucher_number,
                lines,
                created_by,
                created_at,
            } => {
                self.entry_id = entry_id;
                self.status = "Draft".to_string();
                self.transaction_date = transaction_date;
                self.voucher_number = voucher_number;
                let (debit, credit) = Self::calculate_totals(&lines);
                self.total_debit = debit;
                self.total_credit = credit;
                self.created_by = created_by;
                self.created_at = created_at.to_rfc3339();
            }
            JournalEntryEvent::DraftUpdated {
                transaction_date,
                voucher_number,
                lines,
                updated_by,
                updated_at,
                ..
            } => {
                if let Some(date) = transaction_date {
                    self.transaction_date = date;
                }
                if let Some(voucher) = voucher_number {
                    self.voucher_number = voucher;
                }
                if let Some(lines) = lines {
                    let (debit, credit) = Self::calculate_totals(&lines);
                    self.total_debit = debit;
                    self.total_credit = credit;
                }
                self.updated_by = Some(updated_by);
                self.updated_at = Some(updated_at.to_rfc3339());
            }
            JournalEntryEvent::ApprovalRequested { .. } => {
                self.status = "PendingApproval".to_string();
            }
            JournalEntryEvent::Rejected { .. } => {
                self.status = "Draft".to_string();
            }
            JournalEntryEvent::Posted { entry_number, .. } => {
                self.status = "Posted".to_string();
                self.entry_number = Some(entry_number);
            }
            JournalEntryEvent::Reversed { .. } => {
                self.status = "Reversed".to_string();
            }
            JournalEntryEvent::Corrected { .. } => {
                self.status = "Corrected".to_string();
            }
            JournalEntryEvent::Closed { .. } => {
                self.status = "Closed".to_string();
            }
            JournalEntryEvent::Reopened { .. } => {
                self.status = "Posted".to_string();
            }
            JournalEntryEvent::Deleted { .. } => {
                self.status = "Deleted".to_string();
            }
        }

        Ok(())
    }
}

impl ToReadModel for JournalEntryProjection {
    type ReadModel = JournalEntryReadModel;

    fn to_read_model(&self) -> Self::ReadModel {
        JournalEntryReadModel {
            entry_id: self.entry_id.clone(),
            entry_number: self.entry_number.clone(),
            status: self.status.clone(),
            transaction_date: self.transaction_date.clone(),
            voucher_number: self.voucher_number.clone(),
            total_debit: self.total_debit,
            total_credit: self.total_credit,
            created_by: self.created_by.clone(),
            created_at: self.created_at.clone(),
            updated_by: self.updated_by.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

/// JournalEntryProjection戦略
pub struct JournalEntryProjectionStrategy;

impl ProjectionStrategy for JournalEntryProjectionStrategy {
    fn should_update(&self, event: &StoredEvent) -> bool {
        event.event_type.starts_with("Draft")
            || event.event_type.starts_with("Approval")
            || event.event_type == "Rejected"
            || event.event_type == "Posted"
            || event.event_type == "Reversed"
            || event.event_type == "Corrected"
            || event.event_type == "Closed"
            || event.event_type == "Reopened"
            || event.event_type == "Deleted"
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
    fn test_draft_created_projection() {
        let mut projection = JournalEntryProjection::new("JE001".to_string());

        let event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        projection.apply(event).unwrap();

        let read_model = projection.to_read_model();
        assert_eq!(read_model.entry_id, "JE001");
        assert_eq!(read_model.status, "Draft");
        assert_eq!(read_model.transaction_date, "2024-01-01");
    }

    #[test]
    fn test_status_transitions() {
        let mut projection = JournalEntryProjection::new("JE002".to_string());

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
        assert_eq!(projection.to_read_model().status, "Draft");

        // 承認申請
        let event2 = JournalEntryEvent::ApprovalRequested {
            entry_id: "JE002".to_string(),
            requested_by: "user1".to_string(),
            requested_at: Utc::now(),
        };
        projection.apply(event2).unwrap();
        assert_eq!(projection.to_read_model().status, "PendingApproval");

        // 記帳
        let event3 = JournalEntryEvent::Posted {
            entry_id: "JE002".to_string(),
            entry_number: "EN-2024-001".to_string(),
            posted_by: "approver1".to_string(),
            posted_at: Utc::now(),
        };
        projection.apply(event3).unwrap();
        assert_eq!(projection.to_read_model().status, "Posted");
        assert_eq!(projection.to_read_model().entry_number, Some("EN-2024-001".to_string()));
    }

    #[test]
    fn test_calculate_totals() {
        use javelin_domain::financial_close::journal_entry::events::JournalEntryLineDto;

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

        let (debit, credit) = JournalEntryProjection::calculate_totals(&lines);
        assert_eq!(debit, 100000.0);
        assert_eq!(credit, 100000.0);
    }
}
