// 総勘定元帳 - 勘定別残高管理
// 統制要件: 仕訳整合

use std::collections::HashMap;

use crate::{
    common::Amount,
    entity::EntityId,
    error::{DomainError, DomainResult},
};

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
#[derive(Debug, Clone)]
pub struct GeneralLedger {
    /// 勘定科目別残高（account_code -> balance）
    balances: HashMap<String, Amount>,
}

impl GeneralLedger {
    pub fn new() -> Self {
        Self { balances: HashMap::new() }
    }

    /// 借方記帳
    pub fn post_debit(&mut self, account_code: &str, amount: Amount) -> DomainResult<()> {
        if account_code.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }

        let current_balance = self.balances.get(account_code).cloned().unwrap_or_else(Amount::zero);
        let new_balance = &current_balance + &amount;
        self.balances.insert(account_code.to_string(), new_balance);

        Ok(())
    }

    /// 貸方記帳
    pub fn post_credit(&mut self, account_code: &str, amount: Amount) -> DomainResult<()> {
        if account_code.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }

        let current_balance = self.balances.get(account_code).cloned().unwrap_or_else(Amount::zero);
        let new_balance = &current_balance - &amount;
        self.balances.insert(account_code.to_string(), new_balance);

        Ok(())
    }

    /// 残高を取得
    pub fn get_balance(&self, account_code: &str) -> Amount {
        self.balances.get(account_code).cloned().unwrap_or_else(Amount::zero)
    }

    /// 全勘定科目コードを取得
    pub fn get_all_account_codes(&self) -> Vec<String> {
        self.balances.keys().cloned().collect()
    }

    /// 全残高の合計を計算（借方合計 - 貸方合計）
    pub fn total_balance(&self) -> Amount {
        self.balances.values().fold(Amount::zero(), |acc, balance| &acc + balance)
    }

    /// 残高がゼロでない勘定科目のみを取得
    pub fn get_non_zero_accounts(&self) -> HashMap<String, Amount> {
        self.balances
            .iter()
            .filter(|(_, balance)| !balance.is_zero())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

impl Default for GeneralLedger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_ledger_creation() {
        let gl = GeneralLedger::new();
        assert_eq!(gl.get_all_account_codes().len(), 0);
    }

    #[test]
    fn test_post_debit() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();

        assert_eq!(gl.get_balance("1100").to_i64(), Some(1_000_000));
    }

    #[test]
    fn test_post_credit() {
        let mut gl = GeneralLedger::new();
        gl.post_credit("4000", Amount::from_i64(1_000_000)).unwrap();

        assert_eq!(gl.get_balance("4000").to_i64(), Some(-1_000_000));
    }

    #[test]
    fn test_multiple_postings() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();
        gl.post_debit("1100", Amount::from_i64(500_000)).unwrap();
        gl.post_credit("1100", Amount::from_i64(300_000)).unwrap();

        // 1,000,000 + 500,000 - 300,000 = 1,200,000
        assert_eq!(gl.get_balance("1100").to_i64(), Some(1_200_000));
    }

    #[test]
    fn test_get_all_account_codes() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();
        gl.post_credit("4000", Amount::from_i64(1_000_000)).unwrap();

        let codes = gl.get_all_account_codes();
        assert_eq!(codes.len(), 2);
        assert!(codes.contains(&"1100".to_string()));
        assert!(codes.contains(&"4000".to_string()));
    }

    #[test]
    fn test_total_balance() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();
        gl.post_credit("4000", Amount::from_i64(1_000_000)).unwrap();

        // 借方合計 - 貸方合計 = 1,000,000 - 1,000,000 = 0
        assert_eq!(gl.total_balance().to_i64(), Some(0));
    }

    #[test]
    fn test_get_non_zero_accounts() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();
        gl.post_credit("1100", Amount::from_i64(1_000_000)).unwrap(); // ゼロになる
        gl.post_debit("1200", Amount::from_i64(500_000)).unwrap();

        let non_zero = gl.get_non_zero_accounts();
        assert_eq!(non_zero.len(), 1);
        assert!(non_zero.contains_key("1200"));
        assert!(!non_zero.contains_key("1100"));
    }

    #[test]
    fn test_invalid_account_code() {
        let mut gl = GeneralLedger::new();
        assert!(gl.post_debit("", Amount::from_i64(1_000_000)).is_err());
        assert!(gl.post_credit("", Amount::from_i64(1_000_000)).is_err());
    }
}
