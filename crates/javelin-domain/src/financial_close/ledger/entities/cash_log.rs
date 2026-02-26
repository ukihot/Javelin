// キャッシュログ - 現金・銀行取引記録
// 統制要件: 銀行照合

use chrono::NaiveDate;

use crate::{
    entity::{Entity, EntityId},
    financial_close::{AccountingPeriod, Amount},
};

/// キャッシュログID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CashLogId(String);

impl EntityId for CashLogId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl CashLogId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

/// キャッシュログ
#[derive(Debug)]
pub struct CashLog {
    id: CashLogId,
    transaction_date: NaiveDate,
    accounting_period: AccountingPeriod,
    amount: Amount,
    bank_reconciled: bool,
}

impl Entity for CashLog {
    type Id = CashLogId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl CashLog {
    pub fn new(
        id: CashLogId,
        transaction_date: NaiveDate,
        accounting_period: AccountingPeriod,
        amount: Amount,
    ) -> Self {
        Self { id, transaction_date, accounting_period, amount, bank_reconciled: false }
    }

    pub fn transaction_date(&self) -> NaiveDate {
        self.transaction_date
    }

    pub fn accounting_period(&self) -> &AccountingPeriod {
        &self.accounting_period
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn is_bank_reconciled(&self) -> bool {
        self.bank_reconciled
    }

    /// 銀行照合完了
    pub fn mark_reconciled(&mut self) {
        self.bank_reconciled = true;
    }
}
