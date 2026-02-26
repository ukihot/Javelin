// 仕訳帳エンティティ - 発生日基準による全取引記録
// 統制要件: 借貸一致・証憑必須

use chrono::{DateTime, Utc};

use super::{JournalEntryId, JournalEntryLine};
use crate::{
    entity::{Entity, EntityId},
    error::{DomainError, DomainResult},
    financial_close::journal_entry::{
        event_publisher::EventCollector,
        events::{JournalEntryEvent, JournalEntryLineDto},
        values::{EntryNumber, JournalStatus, TransactionDate, UserId, VoucherNumber},
    },
};

/// 仕訳メタデータ
#[derive(Debug, Clone)]
pub struct JournalMetadata {
    pub created_by: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<UserId>,
    pub updated_at: Option<DateTime<Utc>>,
    pub approved_by: Option<UserId>,
    pub approved_at: Option<DateTime<Utc>>,
}

impl JournalMetadata {
    pub fn new(created_by: UserId) -> Self {
        Self {
            created_by,
            created_at: Utc::now(),
            updated_by: None,
            updated_at: None,
            approved_by: None,
            approved_at: None,
        }
    }

    pub fn update(&mut self, updated_by: UserId) {
        self.updated_by = Some(updated_by);
        self.updated_at = Some(Utc::now());
    }

    pub fn approve(&mut self, approved_by: UserId) {
        self.approved_by = Some(approved_by);
        self.approved_at = Some(Utc::now());
    }
}

/// 監査証跡
#[derive(Debug, Clone)]
pub struct AuditTrail {
    entries: Vec<AuditEntry>,
}

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub action: String,
    pub user_id: UserId,
    pub timestamp: DateTime<Utc>,
    pub reason: Option<String>,
}

impl AuditTrail {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, action: String, user_id: UserId, reason: Option<String>) {
        self.entries.push(AuditEntry { action, user_id, timestamp: Utc::now(), reason });
    }

    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

/// 仕訳伝票エンティティ
///
/// 仕訳伝票のライフサイクルを管理するエンティティ。
/// 複数の仕訳明細を持ち、ステータス遷移を制御する。
#[derive(Debug, Clone)]
pub struct JournalEntry {
    id: JournalEntryId,
    entry_number: Option<EntryNumber>,
    status: JournalStatus,
    transaction_date: TransactionDate,
    voucher_number: VoucherNumber,
    lines: Vec<JournalEntryLine>,
    metadata: JournalMetadata,
    audit_trail: AuditTrail,
    event_collector: EventCollector,
}

impl Entity for JournalEntry {
    type Id = JournalEntryId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl JournalEntry {
    /// 新しい仕訳伝票を作成（Draft状態）
    pub fn new(
        id: JournalEntryId,
        transaction_date: TransactionDate,
        voucher_number: VoucherNumber,
        lines: Vec<JournalEntryLine>,
        created_by: UserId,
    ) -> DomainResult<Self> {
        let mut entry = Self {
            id: id.clone(),
            entry_number: None,
            status: JournalStatus::Draft,
            transaction_date: transaction_date.clone(),
            voucher_number: voucher_number.clone(),
            lines: lines.clone(),
            metadata: JournalMetadata::new(created_by.clone()),
            audit_trail: AuditTrail::new(),
            event_collector: EventCollector::new(),
        };

        // 借貸バランスチェック
        entry.validate_balance()?;

        // 監査証跡に記録
        entry.audit_trail.add_entry("Created".to_string(), created_by.clone(), None);

        // DraftCreatedイベントを発行
        let event = JournalEntryEvent::DraftCreated {
            entry_id: id.value().to_string(),
            transaction_date: format!("{}", transaction_date.value()),
            voucher_number: voucher_number.value().to_string(),
            lines: lines.iter().map(JournalEntryLineDto::from_entity).collect(),
            created_by: created_by.value().to_string(),
            created_at: Utc::now(),
        };
        entry.event_collector.add(event);

        Ok(entry)
    }

    /// 伝票IDを取得
    pub fn entry_id(&self) -> &JournalEntryId {
        &self.id
    }

    /// 伝票番号を取得
    pub fn entry_number(&self) -> Option<&EntryNumber> {
        self.entry_number.as_ref()
    }

    /// ステータスを取得
    pub fn status(&self) -> &JournalStatus {
        &self.status
    }

    /// 取引日付を取得
    pub fn transaction_date(&self) -> &TransactionDate {
        &self.transaction_date
    }

    /// 証憑番号を取得
    pub fn voucher_number(&self) -> &VoucherNumber {
        &self.voucher_number
    }

    /// 仕訳明細を取得
    pub fn lines(&self) -> &[JournalEntryLine] {
        &self.lines
    }

    /// メタデータを取得
    pub fn metadata(&self) -> &JournalMetadata {
        &self.metadata
    }

    /// 監査証跡を取得
    pub fn audit_trail(&self) -> &AuditTrail {
        &self.audit_trail
    }

    /// 収集したイベントを取得
    pub fn events(&self) -> &[JournalEntryEvent] {
        self.event_collector.events()
    }

    /// 収集したイベントを消費して取得
    pub fn drain_events(&mut self) -> Vec<JournalEntryEvent> {
        self.event_collector.drain()
    }

    /// 借貸バランスチェック
    pub fn validate_balance(&self) -> DomainResult<()> {
        use crate::financial_close::journal_entry::values::DebitCredit;

        if self.lines.is_empty() {
            return Err(DomainError::JournalEntryValidationFailed);
        }

        let mut debit_total = 0.0;
        let mut credit_total = 0.0;

        for line in &self.lines {
            match line.side() {
                DebitCredit::Debit => debit_total += line.amount().value(),
                DebitCredit::Credit => credit_total += line.amount().value(),
            }
        }

        // 浮動小数点の比較には許容誤差を使用
        const EPSILON: f64 = 0.01;
        if (debit_total - credit_total).abs() > EPSILON {
            return Err(DomainError::JournalEntryValidationFailed);
        }

        Ok(())
    }

    /// 編集可能かチェック
    pub fn is_editable(&self) -> bool {
        self.status.is_editable()
    }

    /// 削除可能かチェック
    pub fn is_deletable(&self) -> bool {
        self.status.is_deletable()
    }

    /// 記帳済かチェック
    pub fn is_posted(&self) -> bool {
        self.status.is_posted()
    }

    /// 承認申請（Draft → PendingApproval）
    pub fn submit_for_approval(&mut self, user_id: UserId) -> DomainResult<()> {
        let target_status = JournalStatus::PendingApproval;

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = target_status;
        self.metadata.update(user_id.clone());
        self.audit_trail
            .add_entry("SubmittedForApproval".to_string(), user_id.clone(), None);

        // ApprovalRequestedイベントを発行
        let event = JournalEntryEvent::ApprovalRequested {
            entry_id: self.id.value().to_string(),
            requested_by: user_id.value().to_string(),
            requested_at: Utc::now(),
        };
        self.event_collector.add(event);

        Ok(())
    }

    /// 差戻し（PendingApproval → Draft）
    pub fn reject(&mut self, user_id: UserId, reason: String) -> DomainResult<()> {
        let target_status = JournalStatus::Draft;

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = target_status;
        self.metadata.update(user_id.clone());
        self.audit_trail
            .add_entry("Rejected".to_string(), user_id.clone(), Some(reason.clone()));

        // Rejectedイベントを発行
        let event = JournalEntryEvent::Rejected {
            entry_id: self.id.value().to_string(),
            reason,
            rejected_by: user_id.value().to_string(),
            rejected_at: Utc::now(),
        };
        self.event_collector.add(event);

        Ok(())
    }

    /// 承認・記帳（PendingApproval → Posted）
    pub fn approve(&mut self, entry_number: EntryNumber, user_id: UserId) -> DomainResult<()> {
        let target_status = JournalStatus::Posted;

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.entry_number = Some(entry_number.clone());
        self.status = target_status;
        self.metadata.approve(user_id.clone());
        self.audit_trail.add_entry("Approved".to_string(), user_id.clone(), None);

        // Postedイベントを発行
        let event = JournalEntryEvent::Posted {
            entry_id: self.id.value().to_string(),
            entry_number: entry_number.value().to_string(),
            posted_by: user_id.value().to_string(),
            posted_at: Utc::now(),
        };
        self.event_collector.add(event);

        Ok(())
    }

    /// 取消（Posted → Reversed）
    pub fn reverse(&mut self, reason: String, user_id: UserId) -> DomainResult<()> {
        let original_id = self.id.value().to_string();
        let target_status =
            JournalStatus::Reversed { reason: reason.clone(), original_id: original_id.clone() };

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = target_status;
        self.metadata.update(user_id.clone());
        self.audit_trail
            .add_entry("Reversed".to_string(), user_id.clone(), Some(reason.clone()));

        // Reversedイベントを発行
        let event = JournalEntryEvent::Reversed {
            entry_id: self.id.value().to_string(),
            original_id,
            reason,
            reversed_by: user_id.value().to_string(),
            reversed_at: Utc::now(),
        };
        self.event_collector.add(event);

        Ok(())
    }

    /// 修正（Reversed → Corrected）
    pub fn correct(
        &mut self,
        reversed_id: String,
        reason: String,
        user_id: UserId,
    ) -> DomainResult<()> {
        let target_status =
            JournalStatus::Corrected { reason: reason.clone(), reversed_id: reversed_id.clone() };

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = target_status;
        self.metadata.update(user_id.clone());
        self.audit_trail
            .add_entry("Corrected".to_string(), user_id.clone(), Some(reason.clone()));

        // Correctedイベントを発行
        let event = JournalEntryEvent::Corrected {
            entry_id: self.id.value().to_string(),
            reversed_id,
            reason,
            corrected_by: user_id.value().to_string(),
            corrected_at: Utc::now(),
        };
        self.event_collector.add(event);

        Ok(())
    }

    /// 締め（Posted → Closed）
    pub fn close(&mut self, user_id: UserId) -> DomainResult<()> {
        let target_status = JournalStatus::Closed;

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = target_status;
        self.metadata.update(user_id.clone());
        self.audit_trail.add_entry("Closed".to_string(), user_id.clone(), None);

        // Closedイベントを発行
        let event = JournalEntryEvent::Closed {
            entry_id: self.id.value().to_string(),
            closed_by: user_id.value().to_string(),
            closed_at: Utc::now(),
        };
        self.event_collector.add(event);

        Ok(())
    }

    /// 再オープン（Closed → Posted）
    pub fn reopen(&mut self, user_id: UserId, reason: String) -> DomainResult<()> {
        let target_status = JournalStatus::Posted;

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = target_status;
        self.metadata.update(user_id.clone());
        self.audit_trail
            .add_entry("Reopened".to_string(), user_id.clone(), Some(reason.clone()));

        // Reopenedイベントを発行
        let event = JournalEntryEvent::Reopened {
            entry_id: self.id.value().to_string(),
            reason,
            reopened_by: user_id.value().to_string(),
            reopened_at: Utc::now(),
        };
        self.event_collector.add(event);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::financial_close::{
        AccountCode,
        journal_entry::values::{Amount, Currency, DebitCredit, LineNumber, TaxType},
    };

    fn create_test_line(
        line_number: u32,
        side: DebitCredit,
        account: &str,
        amount: f64,
    ) -> JournalEntryLine {
        JournalEntryLine::new(
            LineNumber::new(line_number).unwrap(),
            side,
            AccountCode::new(account.to_string()).unwrap(),
            None,
            None,
            Amount::new(amount, Currency::JPY).unwrap(),
            TaxType::NonTaxable,
            Amount::zero(Currency::JPY),
            None,
        )
        .unwrap()
    }

    #[test]
    fn test_journal_entry_creation() {
        let id = JournalEntryId::new("JE001".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V001".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 100000.0),
            create_test_line(2, DebitCredit::Credit, "2000", 100000.0),
        ];

        let entry = JournalEntry::new(id, transaction_date, voucher_number, lines, user_id);

        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert_eq!(entry.status(), &JournalStatus::Draft);
        assert!(entry.entry_number().is_none());
        assert!(entry.is_editable());
        assert!(entry.is_deletable());
        assert!(!entry.is_posted());
        assert_eq!(entry.lines().len(), 2);
    }

    #[test]
    fn test_journal_entry_balance_validation() {
        let id = JournalEntryId::new("JE002".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V002".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        // 借貸不一致のケース
        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 100000.0),
            create_test_line(2, DebitCredit::Credit, "2000", 50000.0), // 不一致
        ];

        let entry = JournalEntry::new(id, transaction_date, voucher_number, lines, user_id);

        assert!(entry.is_err());
    }

    #[test]
    fn test_journal_entry_multiple_lines() {
        let id = JournalEntryId::new("JE003".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V003".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        // 複数明細のケース
        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 60000.0),
            create_test_line(2, DebitCredit::Debit, "1100", 40000.0),
            create_test_line(3, DebitCredit::Credit, "2000", 100000.0),
        ];

        let entry = JournalEntry::new(id, transaction_date, voucher_number, lines, user_id);

        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert_eq!(entry.lines().len(), 3);
        assert!(entry.validate_balance().is_ok());
    }

    #[test]
    fn test_journal_entry_empty_lines() {
        let id = JournalEntryId::new("JE004".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V004".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        let lines = vec![];

        let entry = JournalEntry::new(id, transaction_date, voucher_number, lines, user_id);

        assert!(entry.is_err());
    }

    #[test]
    fn test_journal_metadata() {
        let user_id = UserId::new("user1".to_string());
        let mut metadata = JournalMetadata::new(user_id.clone());

        assert_eq!(metadata.created_by, user_id);
        assert!(metadata.updated_by.is_none());
        assert!(metadata.approved_by.is_none());

        let updater = UserId::new("user2".to_string());
        metadata.update(updater.clone());
        assert_eq!(metadata.updated_by, Some(updater));
        assert!(metadata.updated_at.is_some());

        let approver = UserId::new("user3".to_string());
        metadata.approve(approver.clone());
        assert_eq!(metadata.approved_by, Some(approver));
        assert!(metadata.approved_at.is_some());
    }

    #[test]
    fn test_audit_trail() {
        let mut audit_trail = AuditTrail::new();
        let user_id = UserId::new("user1".to_string());

        audit_trail.add_entry("Created".to_string(), user_id.clone(), None);
        audit_trail.add_entry(
            "Updated".to_string(),
            user_id.clone(),
            Some("Fixed amount".to_string()),
        );

        assert_eq!(audit_trail.entries().len(), 2);
        assert_eq!(audit_trail.entries()[0].action, "Created");
        assert_eq!(audit_trail.entries()[1].action, "Updated");
        assert_eq!(audit_trail.entries()[1].reason, Some("Fixed amount".to_string()));
    }

    #[test]
    fn test_submit_for_approval() {
        let id = JournalEntryId::new("JE005".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V005".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 100000.0),
            create_test_line(2, DebitCredit::Credit, "2000", 100000.0),
        ];

        let mut entry =
            JournalEntry::new(id, transaction_date, voucher_number, lines, user_id.clone())
                .unwrap();

        assert_eq!(entry.status(), &JournalStatus::Draft);

        let approver = UserId::new("approver1".to_string());
        let result = entry.submit_for_approval(approver);

        assert!(result.is_ok());
        assert_eq!(entry.status(), &JournalStatus::PendingApproval);
        assert_eq!(entry.audit_trail().entries().len(), 2);
        assert_eq!(entry.audit_trail().entries()[1].action, "SubmittedForApproval");
    }

    #[test]
    fn test_reject() {
        let id = JournalEntryId::new("JE006".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V006".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 100000.0),
            create_test_line(2, DebitCredit::Credit, "2000", 100000.0),
        ];

        let mut entry =
            JournalEntry::new(id, transaction_date, voucher_number, lines, user_id.clone())
                .unwrap();

        entry.submit_for_approval(UserId::new("approver1".to_string())).unwrap();
        assert_eq!(entry.status(), &JournalStatus::PendingApproval);

        let result = entry.reject(user_id, "Incorrect amount".to_string());

        assert!(result.is_ok());
        assert_eq!(entry.status(), &JournalStatus::Draft);
        assert_eq!(entry.audit_trail().entries().len(), 3);
        assert_eq!(entry.audit_trail().entries()[2].action, "Rejected");
        assert_eq!(entry.audit_trail().entries()[2].reason, Some("Incorrect amount".to_string()));
    }

    #[test]
    fn test_approve() {
        let id = JournalEntryId::new("JE007".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V007".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 100000.0),
            create_test_line(2, DebitCredit::Credit, "2000", 100000.0),
        ];

        let mut entry =
            JournalEntry::new(id, transaction_date, voucher_number, lines, user_id.clone())
                .unwrap();

        entry.submit_for_approval(UserId::new("approver1".to_string())).unwrap();

        let entry_number = EntryNumber::new("EN-2024-001".to_string()).unwrap();
        let approver = UserId::new("approver1".to_string());
        let result = entry.approve(entry_number.clone(), approver);

        assert!(result.is_ok());
        assert_eq!(entry.status(), &JournalStatus::Posted);
        assert_eq!(entry.entry_number(), Some(&entry_number));
        assert!(entry.metadata().approved_by.is_some());
        assert!(entry.metadata().approved_at.is_some());
        assert_eq!(entry.audit_trail().entries().len(), 3);
        assert_eq!(entry.audit_trail().entries()[2].action, "Approved");
    }

    #[test]
    fn test_reverse() {
        let id = JournalEntryId::new("JE008".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V008".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 100000.0),
            create_test_line(2, DebitCredit::Credit, "2000", 100000.0),
        ];

        let mut entry =
            JournalEntry::new(id, transaction_date, voucher_number, lines, user_id.clone())
                .unwrap();

        entry.submit_for_approval(UserId::new("approver1".to_string())).unwrap();
        entry
            .approve(
                EntryNumber::new("EN-2024-002".to_string()).unwrap(),
                UserId::new("approver1".to_string()),
            )
            .unwrap();

        let result = entry.reverse("Incorrect entry".to_string(), user_id);

        assert!(result.is_ok());
        match entry.status() {
            JournalStatus::Reversed { reason, original_id } => {
                assert_eq!(reason, "Incorrect entry");
                assert_eq!(original_id, "JE008");
            }
            _ => panic!("Expected Reversed status"),
        }
        assert_eq!(entry.audit_trail().entries().len(), 4);
        assert_eq!(entry.audit_trail().entries()[3].action, "Reversed");
    }

    #[test]
    fn test_correct() {
        let id = JournalEntryId::new("JE009".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V009".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 100000.0),
            create_test_line(2, DebitCredit::Credit, "2000", 100000.0),
        ];

        let mut entry =
            JournalEntry::new(id, transaction_date, voucher_number, lines, user_id.clone())
                .unwrap();

        entry.submit_for_approval(UserId::new("approver1".to_string())).unwrap();
        entry
            .approve(
                EntryNumber::new("EN-2024-003".to_string()).unwrap(),
                UserId::new("approver1".to_string()),
            )
            .unwrap();
        entry.reverse("Incorrect entry".to_string(), user_id.clone()).unwrap();

        let result = entry.correct("JE009".to_string(), "Corrected amount".to_string(), user_id);

        assert!(result.is_ok());
        match entry.status() {
            JournalStatus::Corrected { reason, reversed_id } => {
                assert_eq!(reason, "Corrected amount");
                assert_eq!(reversed_id, "JE009");
            }
            _ => panic!("Expected Corrected status"),
        }
        assert_eq!(entry.audit_trail().entries().len(), 5);
        assert_eq!(entry.audit_trail().entries()[4].action, "Corrected");
    }

    #[test]
    fn test_close() {
        let id = JournalEntryId::new("JE010".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V010".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 100000.0),
            create_test_line(2, DebitCredit::Credit, "2000", 100000.0),
        ];

        let mut entry =
            JournalEntry::new(id, transaction_date, voucher_number, lines, user_id.clone())
                .unwrap();

        entry.submit_for_approval(UserId::new("approver1".to_string())).unwrap();
        entry
            .approve(
                EntryNumber::new("EN-2024-004".to_string()).unwrap(),
                UserId::new("approver1".to_string()),
            )
            .unwrap();

        let result = entry.close(user_id);

        assert!(result.is_ok());
        assert_eq!(entry.status(), &JournalStatus::Closed);
        assert_eq!(entry.audit_trail().entries().len(), 4);
        assert_eq!(entry.audit_trail().entries()[3].action, "Closed");
    }

    #[test]
    fn test_reopen() {
        let id = JournalEntryId::new("JE011".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V011".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 100000.0),
            create_test_line(2, DebitCredit::Credit, "2000", 100000.0),
        ];

        let mut entry =
            JournalEntry::new(id, transaction_date, voucher_number, lines, user_id.clone())
                .unwrap();

        entry.submit_for_approval(UserId::new("approver1".to_string())).unwrap();
        entry
            .approve(
                EntryNumber::new("EN-2024-005".to_string()).unwrap(),
                UserId::new("approver1".to_string()),
            )
            .unwrap();
        entry.close(user_id.clone()).unwrap();

        let admin = UserId::new("admin1".to_string());
        let result = entry.reopen(admin, "Period reopened for adjustment".to_string());

        assert!(result.is_ok());
        assert_eq!(entry.status(), &JournalStatus::Posted);
        assert_eq!(entry.audit_trail().entries().len(), 5);
        assert_eq!(entry.audit_trail().entries()[4].action, "Reopened");
        assert_eq!(
            entry.audit_trail().entries()[4].reason,
            Some("Period reopened for adjustment".to_string())
        );
    }

    #[test]
    fn test_invalid_status_transitions() {
        let id = JournalEntryId::new("JE012".to_string());
        let transaction_date =
            TransactionDate::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        let voucher_number = VoucherNumber::new("V012".to_string()).unwrap();
        let user_id = UserId::new("user1".to_string());

        let lines = vec![
            create_test_line(1, DebitCredit::Debit, "1000", 100000.0),
            create_test_line(2, DebitCredit::Credit, "2000", 100000.0),
        ];

        let mut entry =
            JournalEntry::new(id, transaction_date, voucher_number, lines, user_id.clone())
                .unwrap();

        // Draft → approve (should fail, needs to be PendingApproval first)
        let result =
            entry.approve(EntryNumber::new("EN-2024-006".to_string()).unwrap(), user_id.clone());
        assert!(result.is_err());

        // Draft → reverse (should fail, needs to be Posted first)
        let result = entry.reverse("Test".to_string(), user_id.clone());
        assert!(result.is_err());

        // Draft → close (should fail, needs to be Posted first)
        let result = entry.close(user_id.clone());
        assert!(result.is_err());
    }
}
