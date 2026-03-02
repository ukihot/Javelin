// 補助元帳エンティティ

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::Amount,
    entity::Entity,
    error::{DomainError, DomainResult},
};

/// 補助元帳ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubsidiaryLedgerId(Uuid);

impl SubsidiaryLedgerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SubsidiaryLedgerId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SubsidiaryLedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for SubsidiaryLedgerId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// 補助元帳タイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubsidiaryLedgerType {
    /// 損益補助元帳（収益・費用明細）
    ProfitAndLoss,
    /// 財政補助元帳（資産・負債明細）
    BalanceSheet,
}

impl SubsidiaryLedgerType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ProfitAndLoss => "ProfitAndLoss",
            Self::BalanceSheet => "BalanceSheet",
        }
    }
}

/// 補助元帳エントリ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsidiaryLedgerEntry {
    /// エントリID
    entry_id: Uuid,
    /// 取引日
    transaction_date: DateTime<Utc>,
    /// 勘定科目コード
    account_code: String,
    /// 補助科目コード
    sub_account_code: Option<String>,
    /// 借方金額
    debit_amount: Amount,
    /// 貸方金額
    credit_amount: Amount,
    /// 摘要
    description: String,
    /// 証憑参照
    evidence_reference: Option<String>,
    /// 収益認識証跡
    revenue_recognition_trace: Option<String>,
    /// 評価根拠
    valuation_basis: Option<String>,
    /// 仕訳ID参照
    journal_entry_id: Option<Uuid>,
}

impl SubsidiaryLedgerEntry {
    pub fn new(
        transaction_date: DateTime<Utc>,
        account_code: String,
        debit_amount: Amount,
        credit_amount: Amount,
        description: String,
    ) -> DomainResult<Self> {
        if account_code.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }

        Ok(Self {
            entry_id: Uuid::new_v4(),
            transaction_date,
            account_code,
            sub_account_code: None,
            debit_amount,
            credit_amount,
            description,
            evidence_reference: None,
            revenue_recognition_trace: None,
            valuation_basis: None,
            journal_entry_id: None,
        })
    }

    pub fn with_sub_account(mut self, sub_account_code: String) -> Self {
        self.sub_account_code = Some(sub_account_code);
        self
    }

    pub fn with_evidence(mut self, evidence_reference: String) -> Self {
        self.evidence_reference = Some(evidence_reference);
        self
    }

    pub fn with_revenue_trace(mut self, revenue_recognition_trace: String) -> Self {
        self.revenue_recognition_trace = Some(revenue_recognition_trace);
        self
    }

    pub fn with_valuation_basis(mut self, valuation_basis: String) -> Self {
        self.valuation_basis = Some(valuation_basis);
        self
    }

    pub fn with_journal_entry_id(mut self, journal_entry_id: Uuid) -> Self {
        self.journal_entry_id = Some(journal_entry_id);
        self
    }

    /// 純額を計算（借方 - 貸方）
    pub fn net_amount(&self) -> Amount {
        &self.debit_amount - &self.credit_amount
    }

    // Getters
    pub fn entry_id(&self) -> &Uuid {
        &self.entry_id
    }

    pub fn transaction_date(&self) -> &DateTime<Utc> {
        &self.transaction_date
    }

    pub fn account_code(&self) -> &str {
        &self.account_code
    }

    pub fn sub_account_code(&self) -> Option<&str> {
        self.sub_account_code.as_deref()
    }

    pub fn debit_amount(&self) -> &Amount {
        &self.debit_amount
    }

    pub fn credit_amount(&self) -> &Amount {
        &self.credit_amount
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }

    pub fn revenue_recognition_trace(&self) -> Option<&str> {
        self.revenue_recognition_trace.as_deref()
    }

    pub fn valuation_basis(&self) -> Option<&str> {
        self.valuation_basis.as_deref()
    }

    pub fn journal_entry_id(&self) -> Option<&Uuid> {
        self.journal_entry_id.as_ref()
    }
}

/// 補助元帳
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsidiaryLedger {
    id: SubsidiaryLedgerId,
    ledger_type: SubsidiaryLedgerType,
    account_code: String,
    account_name: String,
    entries: Vec<SubsidiaryLedgerEntry>,
    /// 勘定科目別残高（account_code -> balance）
    balances: HashMap<String, Amount>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SubsidiaryLedger {
    pub fn new(
        ledger_type: SubsidiaryLedgerType,
        account_code: String,
        account_name: String,
    ) -> DomainResult<Self> {
        if account_code.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }

        Ok(Self {
            id: SubsidiaryLedgerId::new(),
            ledger_type,
            account_code,
            account_name,
            entries: Vec::new(),
            balances: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// エントリを追加
    pub fn add_entry(&mut self, entry: SubsidiaryLedgerEntry) -> DomainResult<()> {
        // 残高を更新
        let account_key = if let Some(sub_code) = entry.sub_account_code() {
            format!("{}:{}", entry.account_code(), sub_code)
        } else {
            entry.account_code().to_string()
        };

        let current_balance = self.balances.get(&account_key).cloned().unwrap_or_else(Amount::zero);
        let new_balance = &current_balance + &entry.net_amount();
        self.balances.insert(account_key, new_balance);

        self.entries.push(entry);
        self.updated_at = Utc::now();

        Ok(())
    }

    /// 勘定科目別残高を取得
    pub fn get_balance(&self, account_code: &str, sub_account_code: Option<&str>) -> Amount {
        let account_key = if let Some(sub_code) = sub_account_code {
            format!("{}:{}", account_code, sub_code)
        } else {
            account_code.to_string()
        };

        self.balances.get(&account_key).cloned().unwrap_or_else(Amount::zero)
    }

    /// 全残高の合計を計算
    pub fn total_balance(&self) -> Amount {
        self.balances.values().fold(Amount::zero(), |acc, balance| &acc + balance)
    }

    /// 期間内のエントリを取得
    pub fn get_entries_in_period(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Vec<&SubsidiaryLedgerEntry> {
        self.entries
            .iter()
            .filter(|e| e.transaction_date() >= start_date && e.transaction_date() <= end_date)
            .collect()
    }

    /// 証憑参照のないエントリを取得
    pub fn get_entries_without_evidence(&self) -> Vec<&SubsidiaryLedgerEntry> {
        self.entries.iter().filter(|e| e.evidence_reference().is_none()).collect()
    }

    /// 収益認識証跡のあるエントリを取得
    pub fn get_entries_with_revenue_trace(&self) -> Vec<&SubsidiaryLedgerEntry> {
        self.entries
            .iter()
            .filter(|e| e.revenue_recognition_trace().is_some())
            .collect()
    }

    /// 評価根拠のあるエントリを取得
    pub fn get_entries_with_valuation_basis(&self) -> Vec<&SubsidiaryLedgerEntry> {
        self.entries.iter().filter(|e| e.valuation_basis().is_some()).collect()
    }

    // Getters
    pub fn id(&self) -> &SubsidiaryLedgerId {
        &self.id
    }

    pub fn ledger_type(&self) -> &SubsidiaryLedgerType {
        &self.ledger_type
    }

    pub fn account_code(&self) -> &str {
        &self.account_code
    }

    pub fn account_name(&self) -> &str {
        &self.account_name
    }

    pub fn entries(&self) -> &[SubsidiaryLedgerEntry] {
        &self.entries
    }

    pub fn balances(&self) -> &HashMap<String, Amount> {
        &self.balances
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

impl Entity for SubsidiaryLedger {
    type Id = SubsidiaryLedgerId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsidiary_ledger_creation() {
        let ledger = SubsidiaryLedger::new(
            SubsidiaryLedgerType::ProfitAndLoss,
            "4000".to_string(),
            "売上高".to_string(),
        )
        .unwrap();

        assert_eq!(ledger.account_code(), "4000");
        assert_eq!(ledger.account_name(), "売上高");
        assert_eq!(ledger.ledger_type(), &SubsidiaryLedgerType::ProfitAndLoss);
        assert_eq!(ledger.entries().len(), 0);
    }

    #[test]
    fn test_add_entry() {
        let mut ledger = SubsidiaryLedger::new(
            SubsidiaryLedgerType::ProfitAndLoss,
            "4000".to_string(),
            "売上高".to_string(),
        )
        .unwrap();

        let entry = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(1_000_000),
            "商品売上".to_string(),
        )
        .unwrap();

        ledger.add_entry(entry).unwrap();

        assert_eq!(ledger.entries().len(), 1);
        assert_eq!(ledger.get_balance("4000", None).to_i64(), Some(-1_000_000));
    }

    #[test]
    fn test_add_entry_with_sub_account() {
        let mut ledger = SubsidiaryLedger::new(
            SubsidiaryLedgerType::ProfitAndLoss,
            "4000".to_string(),
            "売上高".to_string(),
        )
        .unwrap();

        let entry = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(1_000_000),
            "商品A売上".to_string(),
        )
        .unwrap()
        .with_sub_account("A001".to_string());

        ledger.add_entry(entry).unwrap();

        assert_eq!(ledger.get_balance("4000", Some("A001")).to_i64(), Some(-1_000_000));
        assert_eq!(ledger.get_balance("4000", None).to_i64(), Some(0));
    }

    #[test]
    fn test_multiple_entries() {
        let mut ledger = SubsidiaryLedger::new(
            SubsidiaryLedgerType::ProfitAndLoss,
            "4000".to_string(),
            "売上高".to_string(),
        )
        .unwrap();

        // 売上計上
        let entry1 = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(1_000_000),
            "商品売上".to_string(),
        )
        .unwrap();

        // 売上返品
        let entry2 = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "4000".to_string(),
            Amount::from_i64(100_000),
            Amount::zero(),
            "売上返品".to_string(),
        )
        .unwrap();

        ledger.add_entry(entry1).unwrap();
        ledger.add_entry(entry2).unwrap();

        assert_eq!(ledger.entries().len(), 2);
        // 純額: -1,000,000 + 100,000 = -900,000
        assert_eq!(ledger.get_balance("4000", None).to_i64(), Some(-900_000));
    }

    #[test]
    fn test_entry_with_evidence() {
        let entry = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(1_000_000),
            "商品売上".to_string(),
        )
        .unwrap()
        .with_evidence("INV-2024-001".to_string());

        assert_eq!(entry.evidence_reference(), Some("INV-2024-001"));
    }

    #[test]
    fn test_entry_with_revenue_trace() {
        let entry = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(1_000_000),
            "商品売上".to_string(),
        )
        .unwrap()
        .with_revenue_trace("CONTRACT-001:PO-001:STEP5".to_string());

        assert_eq!(entry.revenue_recognition_trace(), Some("CONTRACT-001:PO-001:STEP5"));
    }

    #[test]
    fn test_entry_with_valuation_basis() {
        let entry = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "1300".to_string(),
            Amount::from_i64(1_000_000),
            Amount::zero(),
            "棚卸資産評価損".to_string(),
        )
        .unwrap()
        .with_valuation_basis("純実現可能価額: 900,000円".to_string());

        assert_eq!(entry.valuation_basis(), Some("純実現可能価額: 900,000円"));
    }

    #[test]
    fn test_get_entries_in_period() {
        let mut ledger = SubsidiaryLedger::new(
            SubsidiaryLedgerType::ProfitAndLoss,
            "4000".to_string(),
            "売上高".to_string(),
        )
        .unwrap();

        let date1 = Utc::now();
        let date2 = date1 + chrono::Duration::days(5);
        let date3 = date1 + chrono::Duration::days(10);

        let entry1 = SubsidiaryLedgerEntry::new(
            date1,
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(1_000_000),
            "売上1".to_string(),
        )
        .unwrap();

        let entry2 = SubsidiaryLedgerEntry::new(
            date2,
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(2_000_000),
            "売上2".to_string(),
        )
        .unwrap();

        let entry3 = SubsidiaryLedgerEntry::new(
            date3,
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(3_000_000),
            "売上3".to_string(),
        )
        .unwrap();

        ledger.add_entry(entry1).unwrap();
        ledger.add_entry(entry2).unwrap();
        ledger.add_entry(entry3).unwrap();

        let start = date1;
        let end = date1 + chrono::Duration::days(7);

        let entries = ledger.get_entries_in_period(&start, &end);
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_get_entries_without_evidence() {
        let mut ledger = SubsidiaryLedger::new(
            SubsidiaryLedgerType::ProfitAndLoss,
            "4000".to_string(),
            "売上高".to_string(),
        )
        .unwrap();

        let entry1 = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(1_000_000),
            "売上1".to_string(),
        )
        .unwrap()
        .with_evidence("INV-001".to_string());

        let entry2 = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(2_000_000),
            "売上2".to_string(),
        )
        .unwrap();

        ledger.add_entry(entry1).unwrap();
        ledger.add_entry(entry2).unwrap();

        let without_evidence = ledger.get_entries_without_evidence();
        assert_eq!(without_evidence.len(), 1);
    }

    #[test]
    fn test_total_balance() {
        let mut ledger = SubsidiaryLedger::new(
            SubsidiaryLedgerType::ProfitAndLoss,
            "4000".to_string(),
            "売上高".to_string(),
        )
        .unwrap();

        let entry1 = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(1_000_000),
            "商品A売上".to_string(),
        )
        .unwrap()
        .with_sub_account("A001".to_string());

        let entry2 = SubsidiaryLedgerEntry::new(
            Utc::now(),
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(2_000_000),
            "商品B売上".to_string(),
        )
        .unwrap()
        .with_sub_account("B001".to_string());

        ledger.add_entry(entry1).unwrap();
        ledger.add_entry(entry2).unwrap();

        // 合計: -1,000,000 + -2,000,000 = -3,000,000
        assert_eq!(ledger.total_balance().to_i64(), Some(-3_000_000));
    }
}
