// JournalStatus - 仕訳ステータス値オブジェクト

/// 仕訳ステータス
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JournalStatus {
    /// 下書き
    Draft,
    /// 承認待ち
    PendingApproval,
    /// 記帳済
    Posted,
    /// 取消済
    Reversed { reason: String, original_id: String },
    /// 修正済
    Corrected { reason: String, reversed_id: String },
    /// 締め済
    Closed,
    /// 削除済
    Deleted,
}

impl JournalStatus {
    pub fn can_transition_to(&self, target: &JournalStatus) -> bool {
        use JournalStatus::*;

        matches!(
            (self, target),
            (Draft, PendingApproval)
                | (Draft, Draft)
                | (PendingApproval, Posted)
                | (PendingApproval, Draft)
                | (Posted, Reversed { .. })
                | (Posted, Closed)
                | (Reversed { .. }, Corrected { .. })
                | (Closed, Posted)
                | (Draft, Deleted)
        )
    }

    pub fn is_editable(&self) -> bool {
        matches!(self, JournalStatus::Draft)
    }

    pub fn is_deletable(&self) -> bool {
        matches!(self, JournalStatus::Draft)
    }

    pub fn requires_approval(&self) -> bool {
        matches!(self, JournalStatus::PendingApproval)
    }

    pub fn is_posted(&self) -> bool {
        matches!(self, JournalStatus::Posted)
    }

    pub fn as_str(&self) -> &str {
        match self {
            JournalStatus::Draft => "Draft",
            JournalStatus::PendingApproval => "PendingApproval",
            JournalStatus::Posted => "Posted",
            JournalStatus::Reversed { .. } => "Reversed",
            JournalStatus::Corrected { .. } => "Corrected",
            JournalStatus::Closed => "Closed",
            JournalStatus::Deleted => "Deleted",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            JournalStatus::Draft => "下書き",
            JournalStatus::PendingApproval => "承認待ち",
            JournalStatus::Posted => "記帳済",
            JournalStatus::Reversed { .. } => "取消済",
            JournalStatus::Corrected { .. } => "修正済",
            JournalStatus::Closed => "締め済",
            JournalStatus::Deleted => "削除済",
        }
    }
}

/// 期間ステータス
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeriodStatus {
    /// オープン（入力可能）
    Open,
    /// ソフトロック（制限付き修正可能）
    SoftLocked,
    /// ハードロック（不変）
    HardLocked,
}

impl PeriodStatus {
    pub fn can_post_journal(&self) -> bool {
        matches!(self, PeriodStatus::Open)
    }

    pub fn can_modify_journal(&self, is_admin: bool) -> bool {
        match self {
            PeriodStatus::Open => true,
            PeriodStatus::SoftLocked => is_admin, // 管理者のみ修正可能
            PeriodStatus::HardLocked => false,    // 修正不可
        }
    }
}
