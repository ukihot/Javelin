// ComplianceRiskDomainService

use crate::compliance_risk::{RiskIndicator, RiskIndicatorType, RiskScore};

/// リスク指標計算ドメインサービス
pub trait ComplianceRiskCalculator: Send + Sync {
    /// 1. 仕訳行為区分・直接訂正違反リスク計算
    fn calculate_journal_integrity_risk(&self) -> (RiskScore, String, u32);

    /// 2. 判断ログ・見積根拠欠如リスク計算
    fn calculate_judgment_log_deficiency_risk(&self) -> (RiskScore, String, u32);

    /// 3. 帳簿価額・補助簿不整合リスク計算
    fn calculate_carrying_amount_discrepancy_risk(&self) -> (RiskScore, String, u32);

    /// 4. 重要性基準超過補正リスク計算
    fn calculate_materiality_exceeded_risk(&self) -> (RiskScore, String, u32);

    /// 5. 収益認識5ステップ・未定義処理リスク計算
    fn calculate_ifrs15_risk(&self) -> (RiskScore, String, u32);

    /// 6. ECLステージ遷移・信用リスク計算
    fn calculate_ecl_stage_drift_risk(&self) -> (RiskScore, String, u32);

    /// 7. 締日固定後の補正連鎖リスク計算
    fn calculate_post_lock_adjustment_risk(&self) -> (RiskScore, String, u32);

    /// 8. 外貨換算・機能通貨整合性リスク計算
    fn calculate_ias21_compliance_risk(&self) -> (RiskScore, String, u32);

    /// 全リスク指標を計算
    fn calculate_all_risks(&self) -> Vec<RiskIndicator> {
        vec![
            {
                let (score, details, count) = self.calculate_journal_integrity_risk();
                RiskIndicator::new(RiskIndicatorType::JournalIntegrity, score, details, count)
            },
            {
                let (score, details, count) = self.calculate_judgment_log_deficiency_risk();
                RiskIndicator::new(RiskIndicatorType::JudgmentLogDeficiency, score, details, count)
            },
            {
                let (score, details, count) = self.calculate_carrying_amount_discrepancy_risk();
                RiskIndicator::new(
                    RiskIndicatorType::CarryingAmountDiscrepancy,
                    score,
                    details,
                    count,
                )
            },
            {
                let (score, details, count) = self.calculate_materiality_exceeded_risk();
                RiskIndicator::new(RiskIndicatorType::MaterialityExceeded, score, details, count)
            },
            {
                let (score, details, count) = self.calculate_ifrs15_risk();
                RiskIndicator::new(RiskIndicatorType::IFRS15Risk, score, details, count)
            },
            {
                let (score, details, count) = self.calculate_ecl_stage_drift_risk();
                RiskIndicator::new(RiskIndicatorType::ECLStageDrift, score, details, count)
            },
            {
                let (score, details, count) = self.calculate_post_lock_adjustment_risk();
                RiskIndicator::new(RiskIndicatorType::PostLockAdjustment, score, details, count)
            },
            {
                let (score, details, count) = self.calculate_ias21_compliance_risk();
                RiskIndicator::new(RiskIndicatorType::IAS21Compliance, score, details, count)
            },
        ]
    }
}
