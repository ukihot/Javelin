// 財務諸表の値オブジェクト

use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::Amount,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 財務諸表ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FinancialStatementId(Uuid);

impl FinancialStatementId {
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

impl Default for FinancialStatementId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FinancialStatementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for FinancialStatementId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// 財務諸表タイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinancialStatementType {
    /// 貸借対照表
    BalanceSheet,
    /// 損益計算書
    IncomeStatement,
    /// 包括利益計算書
    ComprehensiveIncome,
    /// 株主資本等変動計算書
    StatementOfChangesInEquity,
    /// キャッシュフロー計算書
    CashFlowStatement,
}

impl FinancialStatementType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BalanceSheet => "BalanceSheet",
            Self::IncomeStatement => "IncomeStatement",
            Self::ComprehensiveIncome => "ComprehensiveIncome",
            Self::StatementOfChangesInEquity => "StatementOfChangesInEquity",
            Self::CashFlowStatement => "CashFlowStatement",
        }
    }
}

impl fmt::Display for FinancialStatementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 流動・非流動区分
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrentNonCurrentClassification {
    /// 流動
    Current,
    /// 非流動
    NonCurrent,
}

impl CurrentNonCurrentClassification {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Current => "Current",
            Self::NonCurrent => "NonCurrent",
        }
    }
}

/// 費用の性質別・機能別区分
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpenseClassification {
    /// 性質別（原材料費、人件費等）
    ByNature(String),
    /// 機能別（売上原価、販管費等）
    ByFunction(String),
}

impl ExpenseClassification {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ByNature(_) => "ByNature",
            Self::ByFunction(_) => "ByFunction",
        }
    }
}

/// その他包括利益項目
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OtherComprehensiveIncomeItem {
    /// 為替換算調整勘定
    ForeignCurrencyTranslationAdjustment,
    /// 有価証券評価差額金
    UnrealizedGainOnSecurities,
    /// 繰延ヘッジ損益
    DeferredHedgeGainLoss,
    /// 再評価差額金
    RevaluationSurplus,
    /// 退職給付に係る調整額
    RemeasurementOfDefinedBenefitPlans,
    /// その他
    Other(String),
}

impl OtherComprehensiveIncomeItem {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ForeignCurrencyTranslationAdjustment => "ForeignCurrencyTranslationAdjustment",
            Self::UnrealizedGainOnSecurities => "UnrealizedGainOnSecurities",
            Self::DeferredHedgeGainLoss => "DeferredHedgeGainLoss",
            Self::RevaluationSurplus => "RevaluationSurplus",
            Self::RemeasurementOfDefinedBenefitPlans => "RemeasurementOfDefinedBenefitPlans",
            Self::Other(_) => "Other",
        }
    }
}

/// キャッシュフロー区分
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CashFlowClassification {
    /// 営業活動
    Operating,
    /// 投資活動
    Investing,
    /// 財務活動
    Financing,
}

impl CashFlowClassification {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Operating => "Operating",
            Self::Investing => "Investing",
            Self::Financing => "Financing",
        }
    }
}

/// 財務諸表項目
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinancialStatementItem {
    /// 項目コード
    item_code: String,
    /// 項目名
    item_name: String,
    /// 金額
    amount: Amount,
    /// 流動・非流動区分（貸借対照表のみ）
    current_classification: Option<CurrentNonCurrentClassification>,
    /// 費用区分（損益計算書のみ）
    expense_classification: Option<ExpenseClassification>,
    /// OCI項目（包括利益計算書のみ）
    oci_item: Option<OtherComprehensiveIncomeItem>,
    /// CF区分（キャッシュフロー計算書のみ）
    cf_classification: Option<CashFlowClassification>,
    /// 表示順序
    display_order: u32,
}

impl FinancialStatementItem {
    pub fn new(
        item_code: String,
        item_name: String,
        amount: Amount,
        display_order: u32,
    ) -> DomainResult<Self> {
        if item_code.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }

        Ok(Self {
            item_code,
            item_name,
            amount,
            current_classification: None,
            expense_classification: None,
            oci_item: None,
            cf_classification: None,
            display_order,
        })
    }

    pub fn with_current_classification(
        mut self,
        classification: CurrentNonCurrentClassification,
    ) -> Self {
        self.current_classification = Some(classification);
        self
    }

    pub fn with_expense_classification(mut self, classification: ExpenseClassification) -> Self {
        self.expense_classification = Some(classification);
        self
    }

    pub fn with_oci_item(mut self, item: OtherComprehensiveIncomeItem) -> Self {
        self.oci_item = Some(item);
        self
    }

    pub fn with_cf_classification(mut self, classification: CashFlowClassification) -> Self {
        self.cf_classification = Some(classification);
        self
    }

    // Getters
    pub fn item_code(&self) -> &str {
        &self.item_code
    }

    pub fn item_name(&self) -> &str {
        &self.item_name
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn current_classification(&self) -> Option<&CurrentNonCurrentClassification> {
        self.current_classification.as_ref()
    }

    pub fn expense_classification(&self) -> Option<&ExpenseClassification> {
        self.expense_classification.as_ref()
    }

    pub fn oci_item(&self) -> Option<&OtherComprehensiveIncomeItem> {
        self.oci_item.as_ref()
    }

    pub fn cf_classification(&self) -> Option<&CashFlowClassification> {
        self.cf_classification.as_ref()
    }

    pub fn display_order(&self) -> u32 {
        self.display_order
    }
}

impl ValueObject for FinancialStatementItem {
    fn validate(&self) -> DomainResult<()> {
        if self.item_code.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_financial_statement_id_creation() {
        let id1 = FinancialStatementId::new();
        let id2 = FinancialStatementId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_financial_statement_item_creation() {
        let item = FinancialStatementItem::new(
            "1100".to_string(),
            "現金及び預金".to_string(),
            Amount::from_i64(1_000_000),
            1,
        )
        .unwrap();

        assert_eq!(item.item_code(), "1100");
        assert_eq!(item.item_name(), "現金及び預金");
        assert_eq!(item.amount().to_i64(), Some(1_000_000));
        assert_eq!(item.display_order(), 1);
    }

    #[test]
    fn test_financial_statement_item_with_current_classification() {
        let item = FinancialStatementItem::new(
            "1100".to_string(),
            "現金及び預金".to_string(),
            Amount::from_i64(1_000_000),
            1,
        )
        .unwrap()
        .with_current_classification(CurrentNonCurrentClassification::Current);

        assert_eq!(item.current_classification(), Some(&CurrentNonCurrentClassification::Current));
    }

    #[test]
    fn test_financial_statement_item_with_expense_classification() {
        let item = FinancialStatementItem::new(
            "5000".to_string(),
            "売上原価".to_string(),
            Amount::from_i64(5_000_000),
            1,
        )
        .unwrap()
        .with_expense_classification(ExpenseClassification::ByFunction("売上原価".to_string()));

        assert!(item.expense_classification().is_some());
    }

    #[test]
    fn test_financial_statement_item_with_oci() {
        let item = FinancialStatementItem::new(
            "9100".to_string(),
            "為替換算調整勘定".to_string(),
            Amount::from_i64(100_000),
            1,
        )
        .unwrap()
        .with_oci_item(OtherComprehensiveIncomeItem::ForeignCurrencyTranslationAdjustment);

        assert!(item.oci_item().is_some());
    }

    #[test]
    fn test_financial_statement_item_with_cf_classification() {
        let item = FinancialStatementItem::new(
            "CF001".to_string(),
            "営業収入".to_string(),
            Amount::from_i64(10_000_000),
            1,
        )
        .unwrap()
        .with_cf_classification(CashFlowClassification::Operating);

        assert_eq!(item.cf_classification(), Some(&CashFlowClassification::Operating));
    }
}
