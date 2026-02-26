// 仕訳ステータス関連の値オブジェクト

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
}

impl JournalStatus {
    /// 指定されたステータスへの遷移が可能かチェック
    pub fn can_transition_to(&self, target: &JournalStatus) -> bool {
        use JournalStatus::*;

        matches!(
            (self, target),
            // Draft → PendingApproval (承認申請)
            (Draft, PendingApproval) |
            // Draft → Draft (編集)
            (Draft, Draft) |
            // PendingApproval → Posted (承認)
            (PendingApproval, Posted) |
            // PendingApproval → Draft (差戻し)
            (PendingApproval, Draft) |
            // Posted → Reversed (取消)
            (Posted, Reversed { .. }) |
            // Posted → Closed (締め)
            (Posted, Closed) |
            // Reversed → Corrected (修正)
            (Reversed { .. }, Corrected { .. }) |
            // Closed → Posted (再オープン、管理者のみ)
            (Closed, Posted)
        )
    }

    /// 編集可能かチェック
    pub fn is_editable(&self) -> bool {
        matches!(self, JournalStatus::Draft)
    }

    /// 削除可能かチェック
    pub fn is_deletable(&self) -> bool {
        matches!(self, JournalStatus::Draft)
    }

    /// 承認が必要かチェック
    pub fn requires_approval(&self) -> bool {
        matches!(self, JournalStatus::PendingApproval)
    }

    /// 記帳済かチェック
    pub fn is_posted(&self) -> bool {
        matches!(self, JournalStatus::Posted)
    }

    /// 文字列に変換
    pub fn as_str(&self) -> &str {
        match self {
            JournalStatus::Draft => "Draft",
            JournalStatus::PendingApproval => "PendingApproval",
            JournalStatus::Posted => "Posted",
            JournalStatus::Reversed { .. } => "Reversed",
            JournalStatus::Corrected { .. } => "Corrected",
            JournalStatus::Closed => "Closed",
        }
    }

    /// 表示名を取得
    pub fn display_name(&self) -> &str {
        match self {
            JournalStatus::Draft => "下書き",
            JournalStatus::PendingApproval => "承認待ち",
            JournalStatus::Posted => "記帳済",
            JournalStatus::Reversed { .. } => "取消済",
            JournalStatus::Corrected { .. } => "修正済",
            JournalStatus::Closed => "締め済",
        }
    }
}

/// 期間ステータス
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeriodStatus {
    /// オープン（入力可能）
    Open,
    /// クローズ（締め済）
    Closed,
    /// ロック（完全ロック）
    Locked,
}

impl PeriodStatus {
    /// 仕訳登録が可能かチェック
    pub fn can_post_journal(&self) -> bool {
        matches!(self, PeriodStatus::Open)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_status_transitions() {
        let draft = JournalStatus::Draft;
        let pending = JournalStatus::PendingApproval;
        let posted = JournalStatus::Posted;
        let closed = JournalStatus::Closed;

        // Draft → PendingApproval
        assert!(draft.can_transition_to(&pending));

        // PendingApproval → Posted
        assert!(pending.can_transition_to(&posted));

        // Posted → Closed
        assert!(posted.can_transition_to(&closed));

        // Closed → Posted (再オープン)
        assert!(closed.can_transition_to(&posted));

        // Posted → Draft (不可)
        assert!(!posted.can_transition_to(&draft));

        // Closed → Draft (不可)
        assert!(!closed.can_transition_to(&draft));
    }

    #[test]
    fn test_journal_status_editable() {
        assert!(JournalStatus::Draft.is_editable());
        assert!(!JournalStatus::PendingApproval.is_editable());
        assert!(!JournalStatus::Posted.is_editable());
        assert!(!JournalStatus::Closed.is_editable());
    }

    #[test]
    fn test_period_status_can_post() {
        assert!(PeriodStatus::Open.can_post_journal());
        assert!(!PeriodStatus::Closed.can_post_journal());
        assert!(!PeriodStatus::Locked.can_post_journal());
    }
}
