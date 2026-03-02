// 財務諸表のドメインサービス

use super::{
    entities::FinancialStatement,
    values::{CurrentNonCurrentClassification, FinancialStatementItem, FinancialStatementType},
};
use crate::{
    common::Amount, error::DomainResult, financial_close::ledger::entities::GeneralLedger,
};

/// 財務諸表ドメインサービス
pub struct FinancialStatementService;

impl FinancialStatementService {
    /// 総勘定元帳から貸借対照表を生成
    pub fn generate_balance_sheet_from_gl(
        general_ledger: &GeneralLedger,
        period_end_date: chrono::DateTime<chrono::Utc>,
        prepared_by: String,
    ) -> DomainResult<FinancialStatement> {
        let mut stmt = FinancialStatement::new(
            FinancialStatementType::BalanceSheet,
            period_end_date,
            prepared_by,
        );

        let mut display_order = 1;

        // 勘定科目コードから財務諸表項目を生成
        for account_code in general_ledger.get_all_account_codes() {
            let balance = general_ledger.get_balance(&account_code);

            if balance.is_zero() {
                continue;
            }

            // 勘定科目コードから流動・非流動を判定
            let classification = Self::determine_current_classification(&account_code)?;

            let item = FinancialStatementItem::new(
                account_code.clone(),
                Self::get_account_name(&account_code),
                balance,
                display_order,
            )?
            .with_current_classification(classification);

            stmt.add_item(item)?;
            display_order += 1;
        }

        stmt.sort_items();
        Ok(stmt)
    }

    /// 流動・非流動区分を判定
    fn determine_current_classification(
        account_code: &str,
    ) -> DomainResult<CurrentNonCurrentClassification> {
        // 簡易的な判定ロジック
        match account_code.chars().next() {
            Some('1') => {
                // 資産
                if account_code.starts_with("11")
                    || account_code.starts_with("12")
                    || account_code.starts_with("13")
                {
                    Ok(CurrentNonCurrentClassification::Current)
                } else {
                    Ok(CurrentNonCurrentClassification::NonCurrent)
                }
            }
            Some('2') => {
                // 負債
                if account_code.starts_with("21") || account_code.starts_with("22") {
                    Ok(CurrentNonCurrentClassification::Current)
                } else {
                    Ok(CurrentNonCurrentClassification::NonCurrent)
                }
            }
            _ => Ok(CurrentNonCurrentClassification::NonCurrent),
        }
    }

    /// 勘定科目名を取得（簡易版）
    fn get_account_name(account_code: &str) -> String {
        match account_code {
            "1100" => "現金及び預金".to_string(),
            "1200" => "売掛金".to_string(),
            "1300" => "棚卸資産".to_string(),
            "1500" => "建物".to_string(),
            "2100" => "買掛金".to_string(),
            "2200" => "短期借入金".to_string(),
            "3000" => "資本金".to_string(),
            "4000" => "売上高".to_string(),
            "5000" => "売上原価".to_string(),
            _ => format!("勘定科目{}", account_code),
        }
    }

    /// 財務諸表間のクロスチェック
    pub fn cross_check_statements(
        balance_sheet: &FinancialStatement,
        income_statement: &FinancialStatement,
        cash_flow_statement: &FinancialStatement,
    ) -> DomainResult<CrossCheckReport> {
        let mut report = CrossCheckReport::new();

        // 貸借対照表の資産・負債・純資産のバランスチェック
        let _total_assets = balance_sheet.calculate_total();
        // 実際には負債と純資産の合計を計算する必要がある
        // ここでは簡易的な実装

        // 損益計算書の当期純利益とキャッシュフロー計算書の整合性
        let net_income = income_statement.calculate_total();
        let operating_cf = cash_flow_statement.calculate_operating_cash_flow()?;

        // 差異が大きい場合は警告
        if let (Some(ni), Some(ocf)) = (net_income.to_f64(), operating_cf.to_f64()) {
            let difference_ratio = ((ni - ocf).abs() / ni.abs()).abs();
            if difference_ratio > 0.5 {
                report.add_warning(format!(
                    "当期純利益と営業CFの差異が大きい: {}%",
                    difference_ratio * 100.0
                ));
            }
        }

        Ok(report)
    }

    /// 表示と測定の分離を検証
    pub fn verify_presentation_measurement_separation(
        statement: &FinancialStatement,
    ) -> DomainResult<bool> {
        // 表示項目が測定ロジックを含んでいないかチェック
        // ここでは簡易的な実装として、すべての項目が適切に分類されているかを確認

        for item in statement.items() {
            match statement.statement_type() {
                FinancialStatementType::BalanceSheet => {
                    if item.current_classification().is_none() {
                        return Ok(false);
                    }
                }
                FinancialStatementType::CashFlowStatement => {
                    if item.cf_classification().is_none() {
                        return Ok(false);
                    }
                }
                _ => {}
            }
        }

        Ok(true)
    }

    /// 補助元帳・総勘定元帳・財務諸表の整合性を検証
    pub fn verify_ledger_statement_consistency(
        general_ledger: &GeneralLedger,
        statement: &FinancialStatement,
    ) -> DomainResult<ConsistencyReport> {
        let mut report = ConsistencyReport::new();

        // 総勘定元帳の残高と財務諸表の金額を比較
        for item in statement.items() {
            let gl_balance = general_ledger.get_balance(item.item_code());
            let stmt_amount = item.amount();

            if &gl_balance != stmt_amount {
                report.add_inconsistency(
                    item.item_code().to_string(),
                    gl_balance,
                    stmt_amount.clone(),
                );
            }
        }

        Ok(report)
    }
}

/// クロスチェックレポート
#[derive(Debug, Clone)]
pub struct CrossCheckReport {
    warnings: Vec<String>,
}

impl CrossCheckReport {
    pub fn new() -> Self {
        Self { warnings: Vec::new() }
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
}

impl Default for CrossCheckReport {
    fn default() -> Self {
        Self::new()
    }
}

/// 整合性レポート
#[derive(Debug, Clone)]
pub struct ConsistencyReport {
    inconsistencies: Vec<Inconsistency>,
}

impl ConsistencyReport {
    pub fn new() -> Self {
        Self { inconsistencies: Vec::new() }
    }

    pub fn add_inconsistency(
        &mut self,
        account_code: String,
        gl_balance: Amount,
        statement_amount: Amount,
    ) {
        let difference = &gl_balance - &statement_amount;
        self.inconsistencies.push(Inconsistency {
            account_code,
            gl_balance,
            statement_amount,
            difference,
        });
    }

    pub fn is_consistent(&self) -> bool {
        self.inconsistencies.is_empty()
    }

    pub fn inconsistencies(&self) -> &[Inconsistency] {
        &self.inconsistencies
    }
}

impl Default for ConsistencyReport {
    fn default() -> Self {
        Self::new()
    }
}

/// 不整合
#[derive(Debug, Clone)]
pub struct Inconsistency {
    pub account_code: String,
    pub gl_balance: Amount,
    pub statement_amount: Amount,
    pub difference: Amount,
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn test_generate_balance_sheet_from_gl() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();
        gl.post_debit("1200", Amount::from_i64(2_000_000)).unwrap();
        gl.post_credit("2100", Amount::from_i64(1_500_000)).unwrap();
        gl.post_credit("3000", Amount::from_i64(1_500_000)).unwrap();

        let stmt = FinancialStatementService::generate_balance_sheet_from_gl(
            &gl,
            Utc::now(),
            "経理担当者".to_string(),
        )
        .unwrap();

        assert_eq!(stmt.statement_type(), &FinancialStatementType::BalanceSheet);
        assert_eq!(stmt.items().len(), 4);
    }

    #[test]
    fn test_determine_current_classification() {
        // 流動資産
        assert_eq!(
            FinancialStatementService::determine_current_classification("1100").unwrap(),
            CurrentNonCurrentClassification::Current
        );

        // 非流動資産
        assert_eq!(
            FinancialStatementService::determine_current_classification("1500").unwrap(),
            CurrentNonCurrentClassification::NonCurrent
        );

        // 流動負債
        assert_eq!(
            FinancialStatementService::determine_current_classification("2100").unwrap(),
            CurrentNonCurrentClassification::Current
        );

        // 非流動負債
        assert_eq!(
            FinancialStatementService::determine_current_classification("2500").unwrap(),
            CurrentNonCurrentClassification::NonCurrent
        );
    }

    #[test]
    fn test_verify_presentation_measurement_separation() {
        let mut stmt = FinancialStatement::new(
            FinancialStatementType::BalanceSheet,
            Utc::now(),
            "経理担当者".to_string(),
        );

        let item = FinancialStatementItem::new(
            "1100".to_string(),
            "現金及び預金".to_string(),
            Amount::from_i64(1_000_000),
            1,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        stmt.add_item(item).unwrap();

        let is_separated =
            FinancialStatementService::verify_presentation_measurement_separation(&stmt).unwrap();

        assert!(is_separated);
    }

    #[test]
    fn test_verify_ledger_statement_consistency() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();

        let mut stmt = FinancialStatement::new(
            FinancialStatementType::BalanceSheet,
            Utc::now(),
            "経理担当者".to_string(),
        );

        let item = FinancialStatementItem::new(
            "1100".to_string(),
            "現金及び預金".to_string(),
            Amount::from_i64(1_000_000),
            1,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        stmt.add_item(item).unwrap();

        let report =
            FinancialStatementService::verify_ledger_statement_consistency(&gl, &stmt).unwrap();

        assert!(report.is_consistent());
    }

    #[test]
    fn test_verify_ledger_statement_inconsistency() {
        let mut gl = GeneralLedger::new();
        gl.post_debit("1100", Amount::from_i64(1_000_000)).unwrap();

        let mut stmt = FinancialStatement::new(
            FinancialStatementType::BalanceSheet,
            Utc::now(),
            "経理担当者".to_string(),
        );

        let item = FinancialStatementItem::new(
            "1100".to_string(),
            "現金及び預金".to_string(),
            Amount::from_i64(900_000), // 差異あり
            1,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        stmt.add_item(item).unwrap();

        let report =
            FinancialStatementService::verify_ledger_statement_consistency(&gl, &stmt).unwrap();

        assert!(!report.is_consistent());
        assert_eq!(report.inconsistencies().len(), 1);
        assert_eq!(report.inconsistencies()[0].difference.to_i64(), Some(100_000));
    }
}
