// 仕訳伝票ドメインイベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::journal_entry::entities::JournalEntryLine;

/// 仕訳伝票ドメインイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum JournalEntryEvent {
    DraftCreated {
        entry_id: String,
        transaction_date: String,
        voucher_number: String,
        lines: Vec<JournalEntryLineDto>,
        created_by: String,
        created_at: DateTime<Utc>,
    },
    DraftUpdated {
        entry_id: String,
        transaction_date: Option<String>,
        voucher_number: Option<String>,
        lines: Option<Vec<JournalEntryLineDto>>,
        updated_by: String,
        updated_at: DateTime<Utc>,
    },
    ApprovalRequested {
        entry_id: String,
        requested_by: String,
        requested_at: DateTime<Utc>,
    },
    Rejected {
        entry_id: String,
        reason: String,
        rejected_by: String,
        rejected_at: DateTime<Utc>,
    },
    Posted {
        entry_id: String,
        entry_number: String,
        posted_by: String,
        posted_at: DateTime<Utc>,
    },
    Reversed {
        entry_id: String,
        original_id: String,
        reason: String,
        reversed_by: String,
        reversed_at: DateTime<Utc>,
    },
    Corrected {
        entry_id: String,
        reversed_id: String,
        reason: String,
        corrected_by: String,
        corrected_at: DateTime<Utc>,
    },
    Closed {
        entry_id: String,
        closed_by: String,
        closed_at: DateTime<Utc>,
    },
    Reopened {
        entry_id: String,
        reason: String,
        reopened_by: String,
        reopened_at: DateTime<Utc>,
    },
    Deleted {
        entry_id: String,
        deleted_by: String,
        deleted_at: DateTime<Utc>,
    },
}

/// 仕訳明細DTO
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JournalEntryLineDto {
    pub line_number: u32,
    pub side: String,
    pub account_code: String,
    pub sub_account_code: Option<String>,
    pub department_code: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub tax_type: String,
    pub tax_amount: f64,
    pub description: Option<String>,
    pub partner_id: Option<String>,
    pub external_name: Option<String>,
    pub tracking_number: Option<String>,
}

impl JournalEntryEvent {
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
    pub fn from_entity(line: &JournalEntryLine) -> Self {
        use std::str::FromStr;
        Self {
            line_number: line.line_number().value(),
            side: line.side().as_str().to_string(),
            account_code: line.account_code().value().to_string(),
            sub_account_code: line.sub_account_code().map(|c| c.value().to_string()),
            department_code: line.department_code().map(|c| c.value().to_string()),
            amount: f64::from_str(&line.amount().amount().to_string()).unwrap_or(0.0),
            currency: line.amount().currency().as_str().to_string(),
            tax_type: line.tax_type().as_str().to_string(),
            tax_amount: f64::from_str(&line.tax_amount().amount().to_string()).unwrap_or(0.0),
            description: line.description().map(|d| d.value().to_string()),
            partner_id: line.partner_id().map(|id| id.value().to_string()),
            external_name: line.external_name().map(|n| n.value().to_string()),
            tracking_number: line.tracking_number().map(|t| t.value().to_string()),
        }
    }
}

impl crate::event::DomainEvent for JournalEntryEvent {
    fn event_type(&self) -> &str {
        self.event_type()
    }

    fn aggregate_id(&self) -> &str {
        self.aggregate_id()
    }

    fn version(&self) -> u64 {
        0
    }
}
