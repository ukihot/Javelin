// 重要性判定リクエストDTO

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 重要性判定リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateMaterialityRequest {
    /// 判定対象項目名
    pub item_name: String,
    /// 金額
    pub amount: i64,
    /// 判定日時
    pub judgment_date: DateTime<Utc>,
    /// 判定理由
    pub reason: String,
    /// 判定者
    pub judged_by: String,
    /// 財務指標（重要性基準計算用）
    pub financial_metrics: FinancialMetrics,
    /// 質的要因（オプション）
    pub qualitative_factors: Option<Vec<String>>,
}

/// 財務指標
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialMetrics {
    /// 税引前利益
    pub pretax_income: i64,
    /// 総資産
    pub total_assets: i64,
    /// 売上高
    pub revenue: i64,
    /// 純資産
    pub equity: i64,
}
