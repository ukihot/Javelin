// 包括的財務諸表生成リクエストDTO

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 包括的財務諸表生成リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateComprehensiveFinancialStatementsRequest {
    /// 会計期間開始日
    pub period_start: DateTime<Utc>,
    /// 会計期間終了日
    pub period_end: DateTime<Utc>,
    /// 生成する財務諸表タイプ
    pub statement_types: Vec<StatementType>,
    /// 整合性検証を実施するか
    pub verify_consistency: bool,
    /// クロスチェックを実施するか
    pub perform_cross_check: bool,
    /// 承認者
    pub approver: Option<String>,
}

/// 財務諸表タイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatementType {
    /// 貸借対照表
    BalanceSheet,
    /// 損益計算書
    IncomeStatement,
    /// 包括利益計算書
    ComprehensiveIncome,
    /// 株主資本等変動計算書
    EquityChanges,
    /// キャッシュフロー計算書
    CashFlow,
}
