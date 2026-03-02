// 元帳のドメインサービス

use std::collections::HashMap;

use super::entities::{GeneralLedger, SubsidiaryLedger};
use crate::{common::Amount, error::DomainResult};

/// 元帳ドメインサービス
pub struct LedgerService;

impl LedgerService {
    /// 補助元帳と総勘定元帳の整合性を検証
    pub fn verify_subsidiary_gl_consistency(
        subsidiary_ledgers: &[&SubsidiaryLedger],
        general_ledger: &GeneralLedger,
    ) -> DomainResult<ConsistencyReport> {
        let mut report = ConsistencyReport::new();

        // 補助元帳の勘定科目別合計を計算
        let mut subsidiary_totals: HashMap<String, Amount> = HashMap::new();

        for ledger in subsidiary_ledgers {
            let account_code = ledger.account_code().to_string();
            let total = ledger.total_balance();

            subsidiary_totals
                .entry(account_code)
                .and_modify(|e| *e = &*e + &total)
                .or_insert_with(|| total.clone());
        }

        // 総勘定元帳の残高と比較
        for (account_code, subsidiary_total) in &subsidiary_totals {
            let gl_balance = general_ledger.get_balance(account_code);

            if subsidiary_total != &gl_balance {
                report.add_discrepancy(account_code.clone(), subsidiary_total.clone(), gl_balance);
            }
        }

        Ok(report)
    }

    /// 前週末残高との増減を分析
    pub fn analyze_balance_changes(
        current_ledger: &GeneralLedger,
        previous_ledger: &GeneralLedger,
    ) -> Vec<BalanceChange> {
        let mut changes = Vec::new();

        // 現在の勘定科目を走査
        for account_code in current_ledger.get_all_account_codes() {
            let current_balance = current_ledger.get_balance(&account_code);
            let previous_balance = previous_ledger.get_balance(&account_code);

            if current_balance != previous_balance {
                let change_amount = &current_balance - &previous_balance;
                let change_rate = Self::calculate_change_rate(&previous_balance, &change_amount);

                changes.push(BalanceChange {
                    account_code: account_code.clone(),
                    previous_balance,
                    current_balance,
                    change_amount,
                    change_rate,
                });
            }
        }

        // 前回存在したが今回存在しない勘定科目も確認
        for account_code in previous_ledger.get_all_account_codes() {
            if !current_ledger.get_all_account_codes().contains(&account_code) {
                let previous_balance = previous_ledger.get_balance(&account_code);
                let current_balance = Amount::zero();
                let change_amount = -&previous_balance;
                let change_rate = -100.0;

                changes.push(BalanceChange {
                    account_code,
                    previous_balance,
                    current_balance,
                    change_amount,
                    change_rate,
                });
            }
        }

        changes
    }

    /// 異常値を検出
    pub fn detect_anomalies(changes: &[BalanceChange], threshold_rate: f64) -> Vec<AnomalyAlert> {
        let mut alerts = Vec::new();

        for change in changes {
            if change.change_rate.abs() >= threshold_rate {
                let severity = if change.change_rate.abs() >= threshold_rate * 2.0 {
                    AnomalySeverity::High
                } else {
                    AnomalySeverity::Medium
                };

                alerts.push(AnomalyAlert {
                    account_code: change.account_code.clone(),
                    change_rate: change.change_rate,
                    change_amount: change.change_amount.clone(),
                    severity,
                    description: format!(
                        "残高が{}%変動しました（{}）",
                        change.change_rate, change.change_amount
                    ),
                });
            }
        }

        alerts
    }

    /// 仮勘定残高を分析
    pub fn analyze_temporary_accounts(
        ledger: &GeneralLedger,
        temporary_account_prefixes: &[&str],
    ) -> Vec<TemporaryAccountAnalysis> {
        let mut analyses = Vec::new();

        for account_code in ledger.get_all_account_codes() {
            if temporary_account_prefixes.iter().any(|prefix| account_code.starts_with(prefix)) {
                let balance = ledger.get_balance(&account_code);

                if !balance.is_zero() {
                    analyses.push(TemporaryAccountAnalysis {
                        account_code,
                        balance,
                        is_long_term: false, // 実際には取引日からの経過日数で判定
                    });
                }
            }
        }

        analyses
    }

    /// 変動率を計算
    fn calculate_change_rate(previous_balance: &Amount, change_amount: &Amount) -> f64 {
        if previous_balance.is_zero() {
            if change_amount.is_zero() {
                0.0
            } else {
                100.0 // 0から変動した場合は100%とする
            }
        } else if let (Some(prev), Some(change)) =
            (previous_balance.to_f64(), change_amount.to_f64())
        {
            (change / prev.abs()) * 100.0
        } else {
            0.0
        }
    }
}

/// 整合性レポート
#[derive(Debug, Clone)]
pub struct ConsistencyReport {
    discrepancies: Vec<Discrepancy>,
}

impl ConsistencyReport {
    pub fn new() -> Self {
        Self { discrepancies: Vec::new() }
    }

    pub fn add_discrepancy(
        &mut self,
        account_code: String,
        subsidiary_total: Amount,
        gl_balance: Amount,
    ) {
        let difference = &subsidiary_total - &gl_balance;
        self.discrepancies.push(Discrepancy {
            account_code,
            subsidiary_total,
            gl_balance,
            difference,
        });
    }

    pub fn is_consistent(&self) -> bool {
        self.discrepancies.is_empty()
    }

    pub fn discrepancies(&self) -> &[Discrepancy] {
        &self.discrepancies
    }
}

impl Default for ConsistencyReport {
    fn default() -> Self {
        Self::new()
    }
}

/// 差異
#[derive(Debug, Clone)]
pub struct Discrepancy {
    pub account_code: String,
    pub subsidiary_total: Amount,
    pub gl_balance: Amount,
    pub difference: Amount,
}

/// 残高変動
#[derive(Debug, Clone)]
pub struct BalanceChange {
    pub account_code: String,
    pub previous_balance: Amount,
    pub current_balance: Amount,
    pub change_amount: Amount,
    pub change_rate: f64,
}

/// 異常アラート
#[derive(Debug, Clone)]
pub struct AnomalyAlert {
    pub account_code: String,
    pub change_rate: f64,
    pub change_amount: Amount,
    pub severity: AnomalySeverity,
    pub description: String,
}

/// 異常の重大度
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
}

/// 仮勘定分析
#[derive(Debug, Clone)]
pub struct TemporaryAccountAnalysis {
    pub account_code: String,
    pub balance: Amount,
    pub is_long_term: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::financial_close::ledger::entities::SubsidiaryLedgerType;

    #[test]
    fn test_verify_consistency_success() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();
        gl.post_credit("4000", Amount::from_i64(1_000_000)).unwrap();

        let mut sub1 = SubsidiaryLedger::new(
            SubsidiaryLedgerType::BalanceSheet,
            "1100".to_string(),
            "現金".to_string(),
        )
        .unwrap();

        let entry1 = super::super::entities::SubsidiaryLedgerEntry::new(
            chrono::Utc::now(),
            "1100".to_string(),
            Amount::from_i64(1_000_000),
            Amount::zero(),
            "現金入金".to_string(),
        )
        .unwrap();

        sub1.add_entry(entry1).unwrap();

        let mut sub2 = SubsidiaryLedger::new(
            SubsidiaryLedgerType::ProfitAndLoss,
            "4000".to_string(),
            "売上高".to_string(),
        )
        .unwrap();

        let entry2 = super::super::entities::SubsidiaryLedgerEntry::new(
            chrono::Utc::now(),
            "4000".to_string(),
            Amount::zero(),
            Amount::from_i64(1_000_000),
            "売上計上".to_string(),
        )
        .unwrap();

        sub2.add_entry(entry2).unwrap();

        let report = LedgerService::verify_subsidiary_gl_consistency(&[&sub1, &sub2], &gl).unwrap();

        assert!(report.is_consistent());
        assert_eq!(report.discrepancies().len(), 0);
    }

    #[test]
    fn test_verify_consistency_with_discrepancy() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();

        let mut sub1 = SubsidiaryLedger::new(
            SubsidiaryLedgerType::BalanceSheet,
            "1100".to_string(),
            "現金".to_string(),
        )
        .unwrap();

        let entry1 = super::super::entities::SubsidiaryLedgerEntry::new(
            chrono::Utc::now(),
            "1100".to_string(),
            Amount::from_i64(900_000), // 差異あり
            Amount::zero(),
            "現金入金".to_string(),
        )
        .unwrap();

        sub1.add_entry(entry1).unwrap();

        let report = LedgerService::verify_subsidiary_gl_consistency(&[&sub1], &gl).unwrap();

        assert!(!report.is_consistent());
        assert_eq!(report.discrepancies().len(), 1);
        assert_eq!(report.discrepancies()[0].account_code, "1100");
        assert_eq!(report.discrepancies()[0].difference.to_i64(), Some(-100_000));
    }

    #[test]
    fn test_analyze_balance_changes() {
        let mut previous_gl = GeneralLedger::new();
        previous_gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();
        previous_gl.post_credit("4000", Amount::from_i64(1_000_000)).unwrap();

        let mut current_gl = GeneralLedger::new();
        current_gl.post_debit("1100", Amount::from_i64(1_500_000)).unwrap();
        current_gl.post_credit("4000", Amount::from_i64(1_500_000)).unwrap();

        let changes = LedgerService::analyze_balance_changes(&current_gl, &previous_gl);

        assert_eq!(changes.len(), 2);

        let cash_change = changes.iter().find(|c| c.account_code == "1100").unwrap();
        assert_eq!(cash_change.change_amount.to_i64(), Some(500_000));
        assert_eq!(cash_change.change_rate, 50.0);

        let revenue_change = changes.iter().find(|c| c.account_code == "4000").unwrap();
        assert_eq!(revenue_change.change_amount.to_i64(), Some(-500_000));
        assert_eq!(revenue_change.change_rate, -50.0);
    }

    #[test]
    fn test_detect_anomalies() {
        let changes = vec![
            BalanceChange {
                account_code: "1100".to_string(),
                previous_balance: Amount::from_i64(1_000_000),
                current_balance: Amount::from_i64(1_500_000),
                change_amount: Amount::from_i64(500_000),
                change_rate: 50.0,
            },
            BalanceChange {
                account_code: "1200".to_string(),
                previous_balance: Amount::from_i64(1_000_000),
                current_balance: Amount::from_i64(1_100_000),
                change_amount: Amount::from_i64(100_000),
                change_rate: 10.0,
            },
            BalanceChange {
                account_code: "1300".to_string(),
                previous_balance: Amount::from_i64(1_000_000),
                current_balance: Amount::from_i64(3_000_000),
                change_amount: Amount::from_i64(2_000_000),
                change_rate: 200.0,
            },
        ];

        let alerts = LedgerService::detect_anomalies(&changes, 30.0);

        assert_eq!(alerts.len(), 2);

        let high_alert = alerts.iter().find(|a| a.account_code == "1300").unwrap();
        assert_eq!(high_alert.severity, AnomalySeverity::High);

        let medium_alert = alerts.iter().find(|a| a.account_code == "1100").unwrap();
        assert_eq!(medium_alert.severity, AnomalySeverity::Medium);
    }

    #[test]
    fn test_analyze_temporary_accounts() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("9000", Amount::from_i64(500_000)).unwrap(); // 仮勘定
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap(); // 通常勘定
        gl.post_debit("9100", Amount::from_i64(300_000)).unwrap(); // 仮勘定

        let analyses = LedgerService::analyze_temporary_accounts(&gl, &["9000", "9100"]);

        assert_eq!(analyses.len(), 2);
        assert!(analyses.iter().any(|a| a.account_code == "9000"));
        assert!(analyses.iter().any(|a| a.account_code == "9100"));
        assert!(!analyses.iter().any(|a| a.account_code == "1100"));
    }

    #[test]
    fn test_calculate_change_rate_zero_previous() {
        let previous = Amount::zero();
        let change = Amount::from_i64(1_000_000);

        let rate = LedgerService::calculate_change_rate(&previous, &change);

        assert_eq!(rate, 100.0);
    }

    #[test]
    fn test_calculate_change_rate_normal() {
        let previous = Amount::from_i64(1_000_000);
        let change = Amount::from_i64(500_000);

        let rate = LedgerService::calculate_change_rate(&previous, &change);

        assert_eq!(rate, 50.0);
    }

    #[test]
    fn test_calculate_change_rate_negative() {
        let previous = Amount::from_i64(1_000_000);
        let change = Amount::from_i64(-300_000);

        let rate = LedgerService::calculate_change_rate(&previous, &change);

        assert_eq!(rate, -30.0);
    }
}
