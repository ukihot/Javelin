// AuditAction - 監査アクション値オブジェクト

use serde::{Deserialize, Serialize};

use crate::value_object::ValueObject;

/// 監査アクション
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditAction {
    Created,
    SubmittedForApproval,
    Rejected,
    Approved,
    Reversed,
    Corrected,
    Closed,
    Reopened,
    Modified,
    ModifiedWithReason,
    Deleted,
}

impl ValueObject for AuditAction {
    fn validate(&self) -> crate::error::DomainResult<()> {
        Ok(())
    }
}

impl AuditAction {
    pub fn as_str(&self) -> &str {
        match self {
            AuditAction::Created => "Created",
            AuditAction::SubmittedForApproval => "SubmittedForApproval",
            AuditAction::Rejected => "Rejected",
            AuditAction::Approved => "Approved",
            AuditAction::Reversed => "Reversed",
            AuditAction::Corrected => "Corrected",
            AuditAction::Closed => "Closed",
            AuditAction::Reopened => "Reopened",
            AuditAction::Modified => "Modified",
            AuditAction::ModifiedWithReason => "ModifiedWithReason",
            AuditAction::Deleted => "Deleted",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            AuditAction::Created => "作成",
            AuditAction::SubmittedForApproval => "承認依頼",
            AuditAction::Rejected => "却下",
            AuditAction::Approved => "承認",
            AuditAction::Reversed => "取消",
            AuditAction::Corrected => "修正",
            AuditAction::Closed => "締め",
            AuditAction::Reopened => "再開",
            AuditAction::Modified => "修正",
            AuditAction::ModifiedWithReason => "理由付き修正",
            AuditAction::Deleted => "削除",
        }
    }
}
