// 元帳整合性検証リクエストDTO

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 元帳整合性検証リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyLedgerConsistencyRequest {
    /// 会計期間開始日
    pub period_start: DateTime<Utc>,
    /// 会計期間終了日
    pub period_end: DateTime<Utc>,
    /// 検証レベル（basic, detailed, comprehensive）
    pub verification_level: VerificationLevel,
    /// 前週末残高との比較を実施するか
    pub compare_with_previous_week: bool,
    /// 異常値検出を実施するか
    pub detect_anomalies: bool,
}

/// 検証レベル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationLevel {
    /// 基本検証（補助元帳と総勘定元帳の一致確認のみ）
    Basic,
    /// 詳細検証（基本 + 前週末残高比較）
    Detailed,
    /// 包括的検証（詳細 + 異常値検出 + 仮勘定分析）
    Comprehensive,
}
