// 財務諸表のエンティティ

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{
    CashFlowClassification, CurrentNonCurrentClassification, FinancialStatementId,
    FinancialStatementItem, FinancialStatementType,
};
use crate::{
    common::Amount,
    entity::Entity,
    error::{DomainError, DomainResult},
};

/// 財務諸表エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialStatement {
    id: FinancialStatementId,
    statement_type: FinancialStatementType,
    period_end_date: DateTime<Utc>,
    items: Vec<FinancialStatementItem>,
    is_consolidated: bool,
    prepared_by: String,
    prepared_at: DateTime<Utc>,
    approved_by: Option<String>,
    approved_at: Option<DateTime<Utc>>,
}

impl FinancialStatement {
    pub fn new(
        statement_type: FinancialStatementType,
        period_end_date: DateTime<Utc>,
        prepared_by: String,
    ) -> Self {
        Self {
            id: FinancialStatementId::new(),
            statement_type,
            period_end_date,
            items: Vec::new(),
            is_consolidated: false,
            prepared_by,
            prepared_at: Utc::now(),
            approved_by: None,
            approved_at: None,
        }
    }

    /// 項目を追加
    pub fn add_item(&mut self, item: FinancialStatementItem) -> DomainResult<()> {
        // 項目の妥当性を検証
        match self.statement_type {
            FinancialStatementType::BalanceSheet => {
                if item.current_classification().is_none() {
                    return Err(DomainError::ValidationError(
                        "貸借対照表項目には流動・非流動区分が必要です".to_string(),
                    ));
                }
            }
            FinancialStatementType::IncomeStatement => {
                if item.expense_classification().is_none() && item.item_code().starts_with('5') {
                    return Err(DomainError::ValidationError(
                        "損益計算書の費用項目には費用区分が必要です".to_string(),
                    ));
                }
            }
            FinancialStatementType::ComprehensiveIncome => {
                // OCI項目の検証は任意
            }
            FinancialStatementType::CashFlowStatement => {
                if item.cf_classification().is_none() {
                    return Err(DomainError::ValidationError(
                        "キャッシュフロー計算書項目にはCF区分が必要です".to_string(),
                    ));
                }
            }
            FinancialStatementType::StatementOfChangesInEquity => {
                // 特別な検証なし
            }
        }

        self.items.push(item);
        Ok(())
    }

    /// 項目を表示順序でソート
    pub fn sort_items(&mut self) {
        self.items.sort_by_key(|item| item.display_order());
    }

    /// 合計を計算
    pub fn calculate_total(&self) -> Amount {
        self.items.iter().fold(Amount::zero(), |acc, item| &acc + item.amount())
    }

    /// 流動資産合計を計算（貸借対照表のみ）
    pub fn calculate_current_assets(&self) -> DomainResult<Amount> {
        if self.statement_type != FinancialStatementType::BalanceSheet {
            return Err(DomainError::ValidationError(
                "貸借対照表以外では流動資産を計算できません".to_string(),
            ));
        }

        let total = self
            .items
            .iter()
            .filter(|item| {
                item.current_classification() == Some(&CurrentNonCurrentClassification::Current)
                    && item.item_code().starts_with('1')
            })
            .fold(Amount::zero(), |acc, item| &acc + item.amount());

        Ok(total)
    }

    /// 非流動資産合計を計算（貸借対照表のみ）
    pub fn calculate_non_current_assets(&self) -> DomainResult<Amount> {
        if self.statement_type != FinancialStatementType::BalanceSheet {
            return Err(DomainError::ValidationError(
                "貸借対照表以外では非流動資産を計算できません".to_string(),
            ));
        }

        let total = self
            .items
            .iter()
            .filter(|item| {
                item.current_classification() == Some(&CurrentNonCurrentClassification::NonCurrent)
                    && item.item_code().starts_with('1')
            })
            .fold(Amount::zero(), |acc, item| &acc + item.amount());

        Ok(total)
    }

    /// 営業活動によるキャッシュフローを計算
    pub fn calculate_operating_cash_flow(&self) -> DomainResult<Amount> {
        if self.statement_type != FinancialStatementType::CashFlowStatement {
            return Err(DomainError::ValidationError(
                "キャッシュフロー計算書以外では営業CFを計算できません".to_string(),
            ));
        }

        let total = self
            .items
            .iter()
            .filter(|item| item.cf_classification() == Some(&CashFlowClassification::Operating))
            .fold(Amount::zero(), |acc, item| &acc + item.amount());

        Ok(total)
    }

    /// 承認
    pub fn approve(&mut self, approver: String) -> DomainResult<()> {
        if self.approved_by.is_some() {
            return Err(DomainError::ValidationError("既に承認済みです".to_string()));
        }

        self.approved_by = Some(approver);
        self.approved_at = Some(Utc::now());
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &FinancialStatementId {
        &self.id
    }

    pub fn statement_type(&self) -> &FinancialStatementType {
        &self.statement_type
    }

    pub fn period_end_date(&self) -> &DateTime<Utc> {
        &self.period_end_date
    }

    pub fn items(&self) -> &[FinancialStatementItem] {
        &self.items
    }

    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }

    pub fn prepared_by(&self) -> &str {
        &self.prepared_by
    }

    pub fn prepared_at(&self) -> &DateTime<Utc> {
        &self.prepared_at
    }

    pub fn approved_by(&self) -> Option<&str> {
        self.approved_by.as_deref()
    }

    pub fn approved_at(&self) -> Option<&DateTime<Utc>> {
        self.approved_at.as_ref()
    }

    pub fn is_approved(&self) -> bool {
        self.approved_by.is_some()
    }
}

impl Entity for FinancialStatement {
    type Id = FinancialStatementId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_financial_statement_creation() {
        let stmt = FinancialStatement::new(
            FinancialStatementType::BalanceSheet,
            Utc::now(),
            "経理担当者".to_string(),
        );

        assert_eq!(stmt.statement_type(), &FinancialStatementType::BalanceSheet);
        assert_eq!(stmt.items().len(), 0);
        assert!(!stmt.is_approved());
    }

    #[test]
    fn test_add_balance_sheet_item() {
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

        assert_eq!(stmt.items().len(), 1);
    }

    #[test]
    fn test_add_balance_sheet_item_without_classification() {
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
        .unwrap();

        // 流動・非流動区分がないとエラー
        assert!(stmt.add_item(item).is_err());
    }

    #[test]
    fn test_calculate_current_assets() {
        let mut stmt = FinancialStatement::new(
            FinancialStatementType::BalanceSheet,
            Utc::now(),
            "経理担当者".to_string(),
        );

        let item1 = FinancialStatementItem::new(
            "1100".to_string(),
            "現金及び預金".to_string(),
            Amount::from_i64(1_000_000),
            1,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        let item2 = FinancialStatementItem::new(
            "1200".to_string(),
            "売掛金".to_string(),
            Amount::from_i64(2_000_000),
            2,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        let item3 = FinancialStatementItem::new(
            "1500".to_string(),
            "建物".to_string(),
            Amount::from_i64(5_000_000),
            3,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::NonCurrent);

        stmt.add_item(item1).unwrap();
        stmt.add_item(item2).unwrap();
        stmt.add_item(item3).unwrap();

        let current_assets = stmt.calculate_current_assets().unwrap();
        assert_eq!(current_assets.to_i64(), Some(3_000_000));
    }

    #[test]
    fn test_calculate_non_current_assets() {
        let mut stmt = FinancialStatement::new(
            FinancialStatementType::BalanceSheet,
            Utc::now(),
            "経理担当者".to_string(),
        );

        let item1 = FinancialStatementItem::new(
            "1100".to_string(),
            "現金及び預金".to_string(),
            Amount::from_i64(1_000_000),
            1,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        let item2 = FinancialStatementItem::new(
            "1500".to_string(),
            "建物".to_string(),
            Amount::from_i64(5_000_000),
            2,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::NonCurrent);

        stmt.add_item(item1).unwrap();
        stmt.add_item(item2).unwrap();

        let non_current_assets = stmt.calculate_non_current_assets().unwrap();
        assert_eq!(non_current_assets.to_i64(), Some(5_000_000));
    }

    #[test]
    fn test_calculate_operating_cash_flow() {
        let mut stmt = FinancialStatement::new(
            FinancialStatementType::CashFlowStatement,
            Utc::now(),
            "経理担当者".to_string(),
        );

        let item1 = FinancialStatementItem::new(
            "CF001".to_string(),
            "営業収入".to_string(),
            Amount::from_i64(10_000_000),
            1,
        )
        .unwrap()
        .with_cf_classification(CashFlowClassification::Operating);

        let item2 = FinancialStatementItem::new(
            "CF002".to_string(),
            "営業支出".to_string(),
            Amount::from_i64(-7_000_000),
            2,
        )
        .unwrap()
        .with_cf_classification(CashFlowClassification::Operating);

        let item3 = FinancialStatementItem::new(
            "CF003".to_string(),
            "投資支出".to_string(),
            Amount::from_i64(-2_000_000),
            3,
        )
        .unwrap()
        .with_cf_classification(CashFlowClassification::Investing);

        stmt.add_item(item1).unwrap();
        stmt.add_item(item2).unwrap();
        stmt.add_item(item3).unwrap();

        let operating_cf = stmt.calculate_operating_cash_flow().unwrap();
        assert_eq!(operating_cf.to_i64(), Some(3_000_000));
    }

    #[test]
    fn test_sort_items() {
        let mut stmt = FinancialStatement::new(
            FinancialStatementType::BalanceSheet,
            Utc::now(),
            "経理担当者".to_string(),
        );

        let item1 = FinancialStatementItem::new(
            "1300".to_string(),
            "棚卸資産".to_string(),
            Amount::from_i64(3_000_000),
            3,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        let item2 = FinancialStatementItem::new(
            "1100".to_string(),
            "現金及び預金".to_string(),
            Amount::from_i64(1_000_000),
            1,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        let item3 = FinancialStatementItem::new(
            "1200".to_string(),
            "売掛金".to_string(),
            Amount::from_i64(2_000_000),
            2,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        stmt.add_item(item1).unwrap();
        stmt.add_item(item2).unwrap();
        stmt.add_item(item3).unwrap();

        stmt.sort_items();

        assert_eq!(stmt.items()[0].item_code(), "1100");
        assert_eq!(stmt.items()[1].item_code(), "1200");
        assert_eq!(stmt.items()[2].item_code(), "1300");
    }

    #[test]
    fn test_approve() {
        let mut stmt = FinancialStatement::new(
            FinancialStatementType::BalanceSheet,
            Utc::now(),
            "経理担当者".to_string(),
        );

        assert!(!stmt.is_approved());

        stmt.approve("経理部長".to_string()).unwrap();

        assert!(stmt.is_approved());
        assert_eq!(stmt.approved_by(), Some("経理部長"));
        assert!(stmt.approved_at().is_some());

        // 二重承認はエラー
        assert!(stmt.approve("CFO".to_string()).is_err());
    }

    #[test]
    fn test_calculate_total() {
        let mut stmt = FinancialStatement::new(
            FinancialStatementType::BalanceSheet,
            Utc::now(),
            "経理担当者".to_string(),
        );

        let item1 = FinancialStatementItem::new(
            "1100".to_string(),
            "現金及び預金".to_string(),
            Amount::from_i64(1_000_000),
            1,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        let item2 = FinancialStatementItem::new(
            "1200".to_string(),
            "売掛金".to_string(),
            Amount::from_i64(2_000_000),
            2,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        stmt.add_item(item1).unwrap();
        stmt.add_item(item2).unwrap();

        let total = stmt.calculate_total();
        assert_eq!(total.to_i64(), Some(3_000_000));
    }
}
