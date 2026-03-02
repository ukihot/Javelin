// 重要性判定レスポンスDTO

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 重要性判定レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateMaterialityResponse {
    /// 判定ID
    pub judgment_id: String,
    /// 重要性判定結果
    pub is_material: bool,
    /// 承認レベル
    pub approval_level: ApprovalLevel,
    /// 適用された閾値
    pub applied_threshold: ThresholdInfo,
    /// 閾値超過率（%）
    pub threshold_excess_rate: Option<f64>,
    /// 質的要因による重要性判定
    pub qualitative_materiality: Option<bool>,
    /// 判定理由
    pub judgment_reason: String,
    /// 判定日時
    pub judgment_date: DateTime<Utc>,
}

/// 承認レベル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalLevel {
    Staff,
    Manager,
    Director,
    CFO,
    Board,
}

/// 閾値情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdInfo {
    /// 閾値タイプ
    pub threshold_type: String,
    /// 閾値金額
    pub threshold_amount: i64,
    /// 基準指標
    pub base_metric: String,
}
