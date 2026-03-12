// JournalEntry Entity - 仕訳伝票エンティティ

use chrono::{DateTime, Utc};

use crate::{
    entity::{Entity, EntityId},
    error::{DomainError, DomainResult},
    journal_entry::{
        entities::{JournalEntryId, JournalEntryLine},
        events::{JournalEntryEvent, JournalEntryLineDto},
        values::{DebitCredit, EntryNumber, JournalStatus, TransactionDate, UserId, VoucherNumber},
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
    // イベントソーシング: 未コミットイベント
    uncommitted_events: Vec<JournalEntryEvent>,
}

impl Entity for JournalEntry {
    type Id = JournalEntryId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl JournalEntry {
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
            uncommitted_events: Vec::new(),
        };

        entry.validate_balance()?;
        entry.audit_trail.add_entry("Created".to_string(), created_by.clone(), None);

        // DraftCreatedイベントを生成
        let event = JournalEntryEvent::DraftCreated {
            entry_id: id.value().to_string(),
            transaction_date: transaction_date.value().format("%Y-%m-%d").to_string(),
            voucher_number: voucher_number.value().to_string(),
            lines: lines.iter().map(JournalEntryLineDto::from_entity).collect(),
            created_by: created_by.value().to_string(),
            created_at: Utc::now(),
        };
        entry.uncommitted_events.push(event);

        Ok(entry)
    }

    pub fn entry_id(&self) -> &JournalEntryId {
        &self.id
    }

    pub fn entry_number(&self) -> Option<&EntryNumber> {
        self.entry_number.as_ref()
    }

    pub fn status(&self) -> &JournalStatus {
        &self.status
    }

    pub fn transaction_date(&self) -> &TransactionDate {
        &self.transaction_date
    }

    pub fn voucher_number(&self) -> &VoucherNumber {
        &self.voucher_number
    }

    pub fn lines(&self) -> &[JournalEntryLine] {
        &self.lines
    }

    pub fn metadata(&self) -> &JournalMetadata {
        &self.metadata
    }

    pub fn audit_trail(&self) -> &AuditTrail {
        &self.audit_trail
    }

    pub fn validate_balance(&self) -> DomainResult<()> {
        if self.lines.is_empty() {
            return Err(DomainError::JournalEntryValidationFailed);
        }

        // 通貨ごとに集計
        use std::collections::HashMap;
        let mut debit_by_currency: HashMap<crate::common::Currency, crate::common::Money> =
            HashMap::new();
        let mut credit_by_currency: HashMap<crate::common::Currency, crate::common::Money> =
            HashMap::new();

        for line in &self.lines {
            let currency = line.amount().currency();
            match line.side() {
                DebitCredit::Debit => {
                    let total = debit_by_currency
                        .entry(currency)
                        .or_insert_with(|| crate::common::Money::zero(currency));
                    *total = total.add(line.amount())?;
                }
                DebitCredit::Credit => {
                    let total = credit_by_currency
                        .entry(currency)
                        .or_insert_with(|| crate::common::Money::zero(currency));
                    *total = total.add(line.amount())?;
                }
            }
        }

        // 各通貨で借方と貸方が一致するか確認
        for (currency, debit_total) in &debit_by_currency {
            let credit_total = credit_by_currency
                .get(currency)
                .ok_or(DomainError::JournalEntryValidationFailed)?;

            if debit_total != credit_total {
                return Err(DomainError::JournalEntryValidationFailed);
            }
        }

        // 貸方にあって借方にない通貨がないか確認
        for currency in credit_by_currency.keys() {
            if !debit_by_currency.contains_key(currency) {
                return Err(DomainError::JournalEntryValidationFailed);
            }
        }

        Ok(())
    }

    pub fn is_editable(&self) -> bool {
        self.status.is_editable()
    }

    pub fn is_deletable(&self) -> bool {
        self.status.is_deletable()
    }

    pub fn is_posted(&self) -> bool {
        self.status.is_posted()
    }

    pub fn submit_for_approval(&mut self, user_id: UserId) -> DomainResult<()> {
        let target_status = JournalStatus::PendingApproval;

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = target_status;
        self.metadata.update(user_id.clone());
        self.audit_trail
            .add_entry("SubmittedForApproval".to_string(), user_id.clone(), None);

        // ApprovalRequestedイベントを生成
        let event = JournalEntryEvent::ApprovalRequested {
            entry_id: self.id.value().to_string(),
            requested_by: user_id.value().to_string(),
            requested_at: Utc::now(),
        };
        self.uncommitted_events.push(event);

        Ok(())
    }

    pub fn reject(&mut self, user_id: UserId, reason: String) -> DomainResult<()> {
        let target_status = JournalStatus::Draft;

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = target_status;
        self.metadata.update(user_id.clone());
        self.audit_trail
            .add_entry("Rejected".to_string(), user_id.clone(), Some(reason.clone()));

        // Rejectedイベントを生成
        let event = JournalEntryEvent::Rejected {
            entry_id: self.id.value().to_string(),
            reason: reason.clone(),
            rejected_by: user_id.value().to_string(),
            rejected_at: Utc::now(),
        };
        self.uncommitted_events.push(event);

        Ok(())
    }

    pub fn approve(&mut self, entry_number: EntryNumber, user_id: UserId) -> DomainResult<()> {
        let target_status = JournalStatus::Posted;

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.entry_number = Some(entry_number.clone());
        self.status = target_status;
        self.metadata.approve(user_id.clone());
        self.audit_trail.add_entry("Approved".to_string(), user_id.clone(), None);

        // Postedイベントを生成
        let event = JournalEntryEvent::Posted {
            entry_id: self.id.value().to_string(),
            entry_number: entry_number.value().to_string(),
            posted_by: user_id.value().to_string(),
            posted_at: Utc::now(),
        };
        self.uncommitted_events.push(event);

        Ok(())
    }

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

        // Reversedイベントを生成
        let event = JournalEntryEvent::Reversed {
            entry_id: self.id.value().to_string(),
            original_id: original_id.clone(),
            reason: reason.clone(),
            reversed_by: user_id.value().to_string(),
            reversed_at: Utc::now(),
        };
        self.uncommitted_events.push(event);

        Ok(())
    }

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

        // Correctedイベントを生成
        let event = JournalEntryEvent::Corrected {
            entry_id: self.id.value().to_string(),
            reversed_id: reversed_id.clone(),
            reason: reason.clone(),
            corrected_by: user_id.value().to_string(),
            corrected_at: Utc::now(),
        };
        self.uncommitted_events.push(event);

        Ok(())
    }

    pub fn close(&mut self, user_id: UserId) -> DomainResult<()> {
        let target_status = JournalStatus::Closed;

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = target_status;
        self.metadata.update(user_id.clone());
        self.audit_trail.add_entry("Closed".to_string(), user_id.clone(), None);

        // Closedイベントを生成
        let event = JournalEntryEvent::Closed {
            entry_id: self.id.value().to_string(),
            closed_by: user_id.value().to_string(),
            closed_at: Utc::now(),
        };
        self.uncommitted_events.push(event);

        Ok(())
    }

    pub fn reopen(&mut self, user_id: UserId, reason: String) -> DomainResult<()> {
        let target_status = JournalStatus::Posted;

        if !self.status.can_transition_to(&target_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = target_status;
        self.metadata.update(user_id.clone());
        self.audit_trail
            .add_entry("Reopened".to_string(), user_id.clone(), Some(reason.clone()));

        // Reopenedイベントを生成
        let event = JournalEntryEvent::Reopened {
            entry_id: self.id.value().to_string(),
            reason: reason.clone(),
            reopened_by: user_id.value().to_string(),
            reopened_at: Utc::now(),
        };
        self.uncommitted_events.push(event);

        Ok(())
    }

    /// 未コミットイベントを取得
    pub fn uncommitted_events(&self) -> &[JournalEntryEvent] {
        &self.uncommitted_events
    }

    /// イベントをクリア（リポジトリが保存後に呼び出す）
    pub fn clear_uncommitted_events(&mut self) {
        self.uncommitted_events.clear();
    }

    /// 明細を更新（Draft状態のみ）
    pub fn update_lines(
        &mut self,
        lines: Vec<JournalEntryLine>,
        user_id: UserId,
    ) -> DomainResult<()> {
        // Draft状態のみ更新可能
        if !matches!(self.status, JournalStatus::Draft) {
            return Err(DomainError::InvalidStatusTransition);
        }

        // 借貸バランスチェック（新しい明細で）
        Self::validate_lines_balance(&lines)?;

        // 明細を更新
        self.lines = lines;
        self.metadata.update(user_id.clone());

        // DraftUpdatedイベントを生成
        let line_dtos: Vec<JournalEntryLineDto> =
            self.lines.iter().map(JournalEntryLineDto::from_entity).collect();

        let event = JournalEntryEvent::DraftUpdated {
            entry_id: self.id.value().to_string(),
            transaction_date: Some(self.transaction_date.value().format("%Y-%m-%d").to_string()),
            voucher_number: Some(self.voucher_number.value().to_string()),
            lines: Some(line_dtos),
            updated_by: user_id.value().to_string(),
            updated_at: Utc::now(),
        };

        self.uncommitted_events.push(event);

        Ok(())
    }

    /// 明細の借貸バランスを検証
    fn validate_lines_balance(lines: &[JournalEntryLine]) -> DomainResult<()> {
        if lines.is_empty() {
            return Err(DomainError::JournalEntryValidationFailed);
        }

        // 通貨ごとに集計
        use std::collections::HashMap;
        let mut debit_by_currency: HashMap<crate::common::Currency, crate::common::Money> =
            HashMap::new();
        let mut credit_by_currency: HashMap<crate::common::Currency, crate::common::Money> =
            HashMap::new();

        for line in lines {
            let currency = line.amount().currency();
            match line.side() {
                DebitCredit::Debit => {
                    let total = debit_by_currency
                        .entry(currency)
                        .or_insert_with(|| crate::common::Money::zero(currency));
                    *total = total.add(line.amount())?;
                }
                DebitCredit::Credit => {
                    let total = credit_by_currency
                        .entry(currency)
                        .or_insert_with(|| crate::common::Money::zero(currency));
                    *total = total.add(line.amount())?;
                }
            }
        }

        // 各通貨で借方と貸方が一致するか確認
        for (currency, debit_total) in &debit_by_currency {
            let credit_total = credit_by_currency
                .get(currency)
                .ok_or(DomainError::JournalEntryValidationFailed)?;

            if debit_total != credit_total {
                return Err(DomainError::JournalEntryValidationFailed);
            }
        }

        // 貸方にあって借方にない通貨がないか確認
        for currency in credit_by_currency.keys() {
            if !debit_by_currency.contains_key(currency) {
                return Err(DomainError::JournalEntryValidationFailed);
            }
        }

        Ok(())
    }

    /// 取引日付を更新（Draft状態のみ）
    pub fn update_transaction_date(
        &mut self,
        transaction_date: TransactionDate,
        user_id: UserId,
    ) -> DomainResult<()> {
        // Draft状態のみ更新可能
        if !matches!(self.status, JournalStatus::Draft) {
            return Err(DomainError::InvalidStatusTransition);
        }

        // 取引日付を更新
        self.transaction_date = transaction_date;
        self.metadata.update(user_id.clone());

        // DraftUpdatedイベントを生成
        let event = JournalEntryEvent::DraftUpdated {
            entry_id: self.id.value().to_string(),
            transaction_date: Some(self.transaction_date.value().format("%Y-%m-%d").to_string()),
            voucher_number: None,
            lines: None,
            updated_by: user_id.value().to_string(),
            updated_at: Utc::now(),
        };

        self.uncommitted_events.push(event);

        Ok(())
    }

    /// 証憑番号を更新（Draft状態のみ）
    pub fn update_voucher_number(
        &mut self,
        voucher_number: VoucherNumber,
        user_id: UserId,
    ) -> DomainResult<()> {
        // Draft状態のみ更新可能
        if !matches!(self.status, JournalStatus::Draft) {
            return Err(DomainError::InvalidStatusTransition);
        }

        // 証憑番号を更新
        self.voucher_number = voucher_number;
        self.metadata.update(user_id.clone());

        // DraftUpdatedイベントを生成
        let event = JournalEntryEvent::DraftUpdated {
            entry_id: self.id.value().to_string(),
            transaction_date: None,
            voucher_number: Some(self.voucher_number.value().to_string()),
            lines: None,
            updated_by: user_id.value().to_string(),
            updated_at: Utc::now(),
        };

        self.uncommitted_events.push(event);

        Ok(())
    }

    /// 下書きを削除（Draft状態のみ）
    pub fn delete(&mut self, user_id: UserId) -> DomainResult<()> {
        // Draft状態のみ削除可能
        if !matches!(self.status, JournalStatus::Draft) {
            return Err(DomainError::InvalidStatusTransition);
        }

        // Deletedイベントを生成
        let event = JournalEntryEvent::Deleted {
            entry_id: self.id.value().to_string(),
            deleted_by: user_id.value().to_string(),
            deleted_at: Utc::now(),
        };

        self.uncommitted_events.push(event);

        Ok(())
    }
}
