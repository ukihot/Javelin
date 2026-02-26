// 総勘定元帳 - 勘定別残高管理
// 統制要件: 仕訳整合

use crate::entity::{Entity, EntityId};
use crate::financial_close::{AccountCode, AccountingPeriod, Amount};

/// 総勘定元帳ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneralLedgerId(String);

impl EntityId for GeneralLedgerId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl GeneralLedgerId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

/// 総勘定元帳
#[derive(Debug)]
pub struct GeneralLedger {
    id: GeneralLedgerId,
    account_code: AccountCode,
    accounting_period: AccountingPeriod,
    opening_balance: Amount,
    debit_total: Amount,
    credit_total: Amount,
}

impl Entity for GeneralLedger {
    type Id = GeneralLedgerId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl GeneralLedger {
    pub fn new(
        id: GeneralLedgerId,
        account_code: AccountCode,
        accounting_period: AccountingPeriod,
        opening_balance: Amount,
    ) -> Self {
        Self {
            id,
            account_code,
            accounting_period,
            opening_balance,
            debit_total: Amount::new(0).unwrap(),
            credit_total: Amount::new(0).unwrap(),
        }
    }

    /// 借方記帳
    pub fn post_debit(&mut self, amount: Amount) {
        self.debit_total = self.debit_total.add(&amount);
    }

    /// 貸方記帳
    pub fn post_credit(&mut self, amount: Amount) {
        self.credit_total = self.credit_total.add(&amount);
    }

    /// 期末残高計算
    pub fn closing_balance(&self) -> Amount {
        self.opening_balance
            .add(&self.debit_total)
            .subtract(&self.credit_total)
    }
}
