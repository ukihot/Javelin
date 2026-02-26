// 持分台帳 - 資本取引履歴
// 統制要件: 株主持分整合

use crate::{
    entity::{Entity, EntityId},
    financial_close::{AccountingPeriod, Amount},
};

/// 持分台帳ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquityLedgerId(String);

impl EntityId for EquityLedgerId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl EquityLedgerId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

/// 持分台帳
#[derive(Debug)]
pub struct EquityLedger {
    id: EquityLedgerId,
    accounting_period: AccountingPeriod,
    capital_stock: Amount,
    retained_earnings: Amount,
}

impl Entity for EquityLedger {
    type Id = EquityLedgerId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl EquityLedger {
    pub fn new(
        id: EquityLedgerId,
        accounting_period: AccountingPeriod,
        capital_stock: Amount,
        retained_earnings: Amount,
    ) -> Self {
        Self { id, accounting_period, capital_stock, retained_earnings }
    }

    pub fn accounting_period(&self) -> &AccountingPeriod {
        &self.accounting_period
    }

    pub fn capital_stock(&self) -> &Amount {
        &self.capital_stock
    }

    pub fn retained_earnings(&self) -> &Amount {
        &self.retained_earnings
    }

    pub fn total_equity(&self) -> Amount {
        self.capital_stock.add(&self.retained_earnings)
    }
}
