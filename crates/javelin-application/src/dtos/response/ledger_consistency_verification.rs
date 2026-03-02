// 元帳整合性検証レスポンスDTO

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 元帳整合性検証レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyLedgerConsistencyResponse {
    /// 検証ID
    pub verification_id: String,
    /// 検証日時
    pub verified_at: DateTime<Utc>,
    /// 整合性検証結果
    pub is_consistent: bool,
    /// 差異の数
    pub discrepancy_count: usize,
    /// 差異詳細
    pub discrepancies: Vec<DiscrepancyDetail>,
    /// 残高変動分析結果
    pub balance_changes: Option<Vec<BalanceChange>>,
    /// 異常値アラート
    pub anomaly_alerts: Option<Vec<AnomalyAlert>>,
    /// 仮勘定残高
    pub temporary_accounts: Option<Vec<TemporaryAccountBalance>>,
}

/// 差異詳細
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscrepancyDetail {
    /// 勘定科目コード
    pub account_code: String,
    /// 勘定科目名
    pub account_name: String,
    /// 補助元帳残高
    pub subsidiary_balance: i64,
    /// 総勘定元帳残高
    pub general_ledger_balance: i64,
    /// 差異金額
    pub difference: i64,
}

/// 残高変動
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceChange {
    /// 勘定科目コード
    pub account_code: String,
    /// 勘定科目名
    pub account_name: String,
    /// 前週末残高
    pub previous_balance: i64,
    /// 当週末残高
    pub current_balance: i64,
    /// 変動額
    pub change_amount: i64,
    /// 変動率（%）
    pub change_rate: f64,
}

/// 異常値アラート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyAlert {
    /// アラートタイプ
    pub alert_type: String,
    /// 重大度
    pub severity: AlertSeverity,
    /// 対象勘定科目
    pub account_code: String,
    /// アラートメッセージ
    pub message: String,
    /// 詳細情報
    pub details: String,
}

/// アラート重大度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// 仮勘定残高
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryAccountBalance {
    /// 勘定科目コード
    pub account_code: String,
    /// 勘定科目名
    pub account_name: String,
    /// 残高
    pub balance: i64,
    /// 滞留日数
    pub days_outstanding: u32,
}
