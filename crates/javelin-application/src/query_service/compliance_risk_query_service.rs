// ComplianceRiskQueryService
// 月次決算コンプライアンス・リスク監視統合QueryService

use serde::{Deserialize, Serialize};

use crate::error::ApplicationResult;

/// 月次決算リスク監視スナップショットクエリ
#[derive(Debug, Clone)]
pub struct GetComplianceRiskSnapshotQuery {
    pub period_year: u32,
    pub period_month: u8,
}

/// リスク指標計測値 - レスポンスDTO
/// BigDecimalの値をString（10進数文字列）で保持し、精度を保証
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMeasurement {
    pub indicator_type: String, // "JournalIntegrity" など
    pub name: String,           // "仕訳行為区分違反" など
    pub score: String,          // BigDecimalを10進数文字列で保持（0-100）
    pub level: String,          // "Low" / "Medium" / "High" / "Critical"
    pub details: String,        // 詳細説明
    pub count: u32,             // 違反・未対応件数
}

/// リスク監視スナップショット - レスポンスDTO
/// すべての数値をString（10進数文字列）で保持し、精度を保証
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRiskSnapshot {
    pub snapshot_id: String,
    pub period_year: u32,
    pub period_month: u8,
    pub measurements: Vec<RiskMeasurement>, // 8個のリスク指標
    pub overall_risk_level: String,         // "Critical" / "High" / "Medium" / "Low"
    pub average_risk_score: String,         // 10進数文字列形式
    pub maximum_risk_score: String,         // 10進数文字列形式
    pub critical_count: u32,                // Critical判定の数
    pub captured_at: String,                // ISO 8601形式
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<String>,
}

/// ComplianceRiskQueryService - 統合リスク監視QueryService
///
/// 8つのリスク指標を計測し、月次決算のコンプライアンス・リスク監視を実施する
#[allow(async_fn_in_trait)]
pub trait ComplianceRiskQueryService: Send + Sync {
    /// リスク監視スナップショットを取得
    async fn get_compliance_risk_snapshot(
        &self,
        query: GetComplianceRiskSnapshotQuery,
    ) -> ApplicationResult<ComplianceRiskSnapshot>;

    /// 1. 仕訳行為区分・直接訂正違反リスクを集計
    async fn measure_journal_integrity_risk(
        &self,
        period_year: u32,
        period_month: u8,
    ) -> ApplicationResult<RiskMeasurement>;

    /// 2. 判断ログ・見積根拠欠如リスクを集計
    async fn measure_judgment_log_deficiency_risk(
        &self,
        period_year: u32,
        period_month: u8,
    ) -> ApplicationResult<RiskMeasurement>;

    /// 3. 帳簿価額・補助簿不整合リスクを集計
    async fn measure_carrying_amount_discrepancy_risk(
        &self,
        period_year: u32,
        period_month: u8,
    ) -> ApplicationResult<RiskMeasurement>;

    /// 4. 重要性基準超過補正リスクを集計
    async fn measure_materiality_exceeded_risk(
        &self,
        period_year: u32,
        period_month: u8,
    ) -> ApplicationResult<RiskMeasurement>;

    /// 5. 収益認識5ステップ・未定義処理リスクを集計
    async fn measure_ifrs15_risk(
        &self,
        period_year: u32,
        period_month: u8,
    ) -> ApplicationResult<RiskMeasurement>;

    /// 6. ECLステージ遷移・信用リスクを集計
    async fn measure_ecl_stage_drift_risk(
        &self,
        period_year: u32,
        period_month: u8,
    ) -> ApplicationResult<RiskMeasurement>;

    /// 7. 締日固定後の補正連鎖リスクを集計
    async fn measure_post_lock_adjustment_risk(
        &self,
        period_year: u32,
        period_month: u8,
    ) -> ApplicationResult<RiskMeasurement>;

    /// 8. 外貨換算・機能通貨整合性リスクを集計
    async fn measure_ias21_compliance_risk(
        &self,
        period_year: u32,
        period_month: u8,
    ) -> ApplicationResult<RiskMeasurement>;
}
