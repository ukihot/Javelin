// 仕訳伝票ドメインイベント
// すべての状態変更をイベントとして記録

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::entities::JournalEntryLine;

/// 仕訳伝票ドメインイベント
///
/// 仕訳伝票のライフサイクルにおけるすべての状態変更を表現する。
/// Event Sourcingパターンに基づき、これらのイベントから
/// 仕訳伝票の現在の状態を復元できる。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum JournalEntryEvent {
    /// 下書き作成
    ///
    /// 新しい仕訳伝票が下書き状態で作成された。
    DraftCreated {
        entry_id: String,
        transaction_date: String,
        voucher_number: String,
        lines: Vec<JournalEntryLineDto>,
        created_by: String,
        created_at: DateTime<Utc>,
    },

    /// 下書き更新
    ///
    /// 下書き状態の仕訳伝票が更新された。
    DraftUpdated {
        entry_id: String,
        transaction_date: Option<String>,
        voucher_number: Option<String>,
        lines: Option<Vec<JournalEntryLineDto>>,
        updated_by: String,
        updated_at: DateTime<Utc>,
    },

    /// 承認申請
    ///
    /// 下書き状態の仕訳伝票が承認申請された。
    /// Draft → PendingApproval への遷移。
    ApprovalRequested { entry_id: String, requested_by: String, requested_at: DateTime<Utc> },

    /// 差戻し
    ///
    /// 承認待ち状態の仕訳伝票が差し戻された。
    /// PendingApproval → Draft への遷移。
    Rejected {
        entry_id: String,
        reason: String,
        rejected_by: String,
        rejected_at: DateTime<Utc>,
    },

    /// 記帳
    ///
    /// 承認待ち状態の仕訳伝票が承認され、記帳された。
    /// PendingApproval → Posted への遷移。
    /// 正式な伝票番号が採番される。
    Posted {
        entry_id: String,
        entry_number: String,
        posted_by: String,
        posted_at: DateTime<Utc>,
    },

    /// 取消
    ///
    /// 記帳済の仕訳伝票が取り消された。
    /// Posted → Reversed への遷移。
    /// 取消仕訳が自動生成される。
    Reversed {
        entry_id: String,
        original_id: String,
        reason: String,
        reversed_by: String,
        reversed_at: DateTime<Utc>,
    },

    /// 修正
    ///
    /// 取消済の仕訳伝票に対する修正仕訳が作成された。
    /// Reversed → Corrected への遷移。
    Corrected {
        entry_id: String,
        reversed_id: String,
        reason: String,
        corrected_by: String,
        corrected_at: DateTime<Utc>,
    },

    /// 締め
    ///
    /// 記帳済の仕訳伝票が会計期間の締め処理により締められた。
    /// Posted → Closed への遷移。
    Closed { entry_id: String, closed_by: String, closed_at: DateTime<Utc> },

    /// 再オープン
    ///
    /// 締め済の仕訳伝票が再オープンされた。
    /// Closed → Posted への遷移。
    /// 管理者権限が必要。
    Reopened {
        entry_id: String,
        reason: String,
        reopened_by: String,
        reopened_at: DateTime<Utc>,
    },

    /// 削除
    ///
    /// 下書き状態の仕訳伝票が削除された。
    /// Draft状態のみ削除可能。
    Deleted { entry_id: String, deleted_by: String, deleted_at: DateTime<Utc> },
}

/// 仕訳明細DTO
///
/// イベントペイロードとして使用される仕訳明細の表現。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JournalEntryLineDto {
    pub line_number: u32,
    pub side: String, // "Debit" or "Credit"
    pub account_code: String,
    pub sub_account_code: Option<String>,
    pub department_code: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub tax_type: String,
    pub tax_amount: f64,
    pub description: Option<String>,
}

impl JournalEntryEvent {
    /// イベントタイプを取得
    pub fn event_type(&self) -> &str {
        match self {
            JournalEntryEvent::DraftCreated { .. } => "DraftCreated",
            JournalEntryEvent::DraftUpdated { .. } => "DraftUpdated",
            JournalEntryEvent::ApprovalRequested { .. } => "ApprovalRequested",
            JournalEntryEvent::Rejected { .. } => "Rejected",
            JournalEntryEvent::Posted { .. } => "Posted",
            JournalEntryEvent::Reversed { .. } => "Reversed",
            JournalEntryEvent::Corrected { .. } => "Corrected",
            JournalEntryEvent::Closed { .. } => "Closed",
            JournalEntryEvent::Reopened { .. } => "Reopened",
            JournalEntryEvent::Deleted { .. } => "Deleted",
        }
    }

    /// 集約IDを取得
    pub fn aggregate_id(&self) -> &str {
        match self {
            JournalEntryEvent::DraftCreated { entry_id, .. }
            | JournalEntryEvent::DraftUpdated { entry_id, .. }
            | JournalEntryEvent::ApprovalRequested { entry_id, .. }
            | JournalEntryEvent::Rejected { entry_id, .. }
            | JournalEntryEvent::Posted { entry_id, .. }
            | JournalEntryEvent::Reversed { entry_id, .. }
            | JournalEntryEvent::Corrected { entry_id, .. }
            | JournalEntryEvent::Closed { entry_id, .. }
            | JournalEntryEvent::Reopened { entry_id, .. }
            | JournalEntryEvent::Deleted { entry_id, .. } => entry_id,
        }
    }

    /// イベント発生日時を取得
    pub fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            JournalEntryEvent::DraftCreated { created_at, .. } => *created_at,
            JournalEntryEvent::DraftUpdated { updated_at, .. } => *updated_at,
            JournalEntryEvent::ApprovalRequested { requested_at, .. } => *requested_at,
            JournalEntryEvent::Rejected { rejected_at, .. } => *rejected_at,
            JournalEntryEvent::Posted { posted_at, .. } => *posted_at,
            JournalEntryEvent::Reversed { reversed_at, .. } => *reversed_at,
            JournalEntryEvent::Corrected { corrected_at, .. } => *corrected_at,
            JournalEntryEvent::Closed { closed_at, .. } => *closed_at,
            JournalEntryEvent::Reopened { reopened_at, .. } => *reopened_at,
            JournalEntryEvent::Deleted { deleted_at, .. } => *deleted_at,
        }
    }

    /// 実行者を取得
    pub fn actor(&self) -> &str {
        match self {
            JournalEntryEvent::DraftCreated { created_by, .. } => created_by,
            JournalEntryEvent::DraftUpdated { updated_by, .. } => updated_by,
            JournalEntryEvent::ApprovalRequested { requested_by, .. } => requested_by,
            JournalEntryEvent::Rejected { rejected_by, .. } => rejected_by,
            JournalEntryEvent::Posted { posted_by, .. } => posted_by,
            JournalEntryEvent::Reversed { reversed_by, .. } => reversed_by,
            JournalEntryEvent::Corrected { corrected_by, .. } => corrected_by,
            JournalEntryEvent::Closed { closed_by, .. } => closed_by,
            JournalEntryEvent::Reopened { reopened_by, .. } => reopened_by,
            JournalEntryEvent::Deleted { deleted_by, .. } => deleted_by,
        }
    }
}

impl JournalEntryLineDto {
    /// JournalEntryLineからDTOを作成
    pub fn from_entity(line: &JournalEntryLine) -> Self {
        Self {
            line_number: line.line_number().value(),
            side: line.side().as_str().to_string(),
            account_code: line.account_code().code().to_string(),
            sub_account_code: line.sub_account_code().map(|c| c.value().to_string()),
            department_code: line.department_code().map(|c| c.value().to_string()),
            amount: line.amount().value(),
            currency: line.amount().currency().as_str().to_string(),
            tax_type: line.tax_type().as_str().to_string(),
            tax_amount: line.tax_amount().value(),
            description: line.description().map(|d| d.value().to_string()),
        }
    }
}

// DomainEventトレイト実装
impl crate::event::DomainEvent for JournalEntryEvent {
    fn event_type(&self) -> &str {
        self.event_type()
    }

    fn aggregate_id(&self) -> &str {
        self.aggregate_id()
    }

    fn version(&self) -> u64 {
        // イベント自体にはバージョン情報がないため、0を返す
        // 実際のバージョン管理はリポジトリ層で行う
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_created_event() {
        let event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        assert_eq!(event.event_type(), "DraftCreated");
        assert_eq!(event.aggregate_id(), "JE001");
        assert_eq!(event.actor(), "user1");
    }

    #[test]
    fn test_approval_requested_event() {
        let event = JournalEntryEvent::ApprovalRequested {
            entry_id: "JE002".to_string(),
            requested_by: "user2".to_string(),
            requested_at: Utc::now(),
        };

        assert_eq!(event.event_type(), "ApprovalRequested");
        assert_eq!(event.aggregate_id(), "JE002");
        assert_eq!(event.actor(), "user2");
    }

    #[test]
    fn test_posted_event() {
        let event = JournalEntryEvent::Posted {
            entry_id: "JE003".to_string(),
            entry_number: "EN-2024-001".to_string(),
            posted_by: "approver1".to_string(),
            posted_at: Utc::now(),
        };

        assert_eq!(event.event_type(), "Posted");
        assert_eq!(event.aggregate_id(), "JE003");
        assert_eq!(event.actor(), "approver1");
    }

    #[test]
    fn test_reversed_event() {
        let event = JournalEntryEvent::Reversed {
            entry_id: "JE004".to_string(),
            original_id: "JE003".to_string(),
            reason: "Incorrect entry".to_string(),
            reversed_by: "user1".to_string(),
            reversed_at: Utc::now(),
        };

        assert_eq!(event.event_type(), "Reversed");
        assert_eq!(event.aggregate_id(), "JE004");
        assert_eq!(event.actor(), "user1");
    }

    #[test]
    fn test_event_serialization() {
        let event = JournalEntryEvent::DraftCreated {
            entry_id: "JE005".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V005".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: JournalEntryEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_journal_entry_line_dto() {
        let dto = JournalEntryLineDto {
            line_number: 1,
            side: "Debit".to_string(),
            account_code: "1000".to_string(),
            sub_account_code: None,
            department_code: Some("D001".to_string()),
            amount: 100000.0,
            currency: "JPY".to_string(),
            tax_type: "NonTaxable".to_string(),
            tax_amount: 0.0,
            description: Some("Test description".to_string()),
        };

        let json = serde_json::to_string(&dto).unwrap();
        let deserialized: JournalEntryLineDto = serde_json::from_str(&json).unwrap();

        assert_eq!(dto, deserialized);
    }
}
