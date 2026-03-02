// 包括的財務諸表生成レスポンスDTO

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 包括的財務諸表生成レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateComprehensiveFinancialStatementsResponse {
    /// 生成された財務諸表
    pub statements: Vec<GeneratedStatement>,
    /// 整合性検証結果
    pub consistency_check: Option<ConsistencyCheckResult>,
    /// クロスチェック結果
    pub cross_check: Option<CrossCheckResult>,
    /// 生成日時
    pub generated_at: DateTime<Utc>,
    /// 承認状態
    pub approval_status: ApprovalStatus,
}

/// 生成された財務諸表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedStatement {
    /// 財務諸表ID
    pub statement_id: String,
    /// 財務諸表タイプ
    pub statement_type: String,
    /// 項目数
    pub item_count: usize,
    /// 合計金額
    pub total_amount: i64,
}

/// 整合性検証結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyCheckResult {
    /// 整合性あり
    pub is_consistent: bool,
    /// 不整合の数
    pub inconsistency_count: usize,
    /// 不整合詳細
    pub inconsistencies: Vec<InconsistencyDetail>,
}

/// 不整合詳細
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InconsistencyDetail {
    /// 不整合タイプ
    pub inconsistency_type: String,
    /// 説明
    pub description: String,
    /// 影響額
    pub impact_amount: Option<i64>,
}

/// クロスチェック結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossCheckResult {
    /// チェック合格
    pub passed: bool,
    /// 合格したチェック数
    pub checks_passed: usize,
    /// 失敗したチェック数
    pub checks_failed: usize,
    /// 失敗したチェック詳細
    pub failed_checks: Vec<FailedCheck>,
}

/// 失敗したチェック
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedCheck {
    /// チェック名
    pub check_name: String,
    /// 期待値
    pub expected: String,
    /// 実際の値
    pub actual: String,
    /// 差異
    pub difference: Option<i64>,
}

/// 承認状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Draft,
    PendingApproval,
    Approved,
    Rejected,
}
