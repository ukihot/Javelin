// ComplianceRiskQueryService実装

use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::Utc;
use javelin_application::{
    error::ApplicationResult,
    query_service::{
        ComplianceRiskQueryService, ComplianceRiskSnapshot as AppSnapshot,
        GetComplianceRiskSnapshotQuery, RiskMeasurement,
    },
};

/// ComplianceRiskQueryService実装
pub struct ComplianceRiskQueryServiceImpl {
    // 将来的に外部QueryServiceをDIできるが、現在は最小実装
}

impl ComplianceRiskQueryServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ComplianceRiskQueryServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplianceRiskQueryService for ComplianceRiskQueryServiceImpl {
    async fn get_compliance_risk_snapshot(
        &self,
        query: GetComplianceRiskSnapshotQuery,
    ) -> ApplicationResult<AppSnapshot> {
        let period_year = query.period_year;
        let period_month = query.period_month;

        let measurements = vec![
            self.measure_journal_integrity_risk(period_year, period_month).await?,
            self.measure_judgment_log_deficiency_risk(period_year, period_month).await?,
            self.measure_carrying_amount_discrepancy_risk(period_year, period_month).await?,
            self.measure_materiality_exceeded_risk(period_year, period_month).await?,
            self.measure_ifrs15_risk(period_year, period_month).await?,
            self.measure_ecl_stage_drift_risk(period_year, period_month).await?,
            self.measure_post_lock_adjustment_risk(period_year, period_month).await?,
            self.measure_ias21_compliance_risk(period_year, period_month).await?,
        ];

        // 内部計算用: scoreをStringから解析してBigDecimalで計算
        let scores: Vec<BigDecimal> = measurements
            .iter()
            .filter_map(|m| BigDecimal::from_str(&m.score).ok())
            .collect();

        let sum_score: BigDecimal = scores.iter().cloned().sum();
        let average_risk_score = if scores.is_empty() {
            BigDecimal::from(0)
        } else {
            sum_score / BigDecimal::from(scores.len() as i32)
        };

        let maximum_risk_score =
            scores.iter().cloned().max().unwrap_or_else(|| BigDecimal::from(0));

        let critical_count = measurements.iter().filter(|m| m.level == "Critical").count() as u32;

        let critical_threshold = BigDecimal::from(60);
        let high_threshold = BigDecimal::from(40);
        let medium_threshold = BigDecimal::from(20);

        let overall_risk_level = if average_risk_score >= critical_threshold {
            "Critical".to_string()
        } else if average_risk_score >= high_threshold {
            "High".to_string()
        } else if average_risk_score >= medium_threshold {
            "Medium".to_string()
        } else {
            "Low".to_string()
        };

        Ok(AppSnapshot {
            snapshot_id: uuid::Uuid::new_v4().to_string(),
            period_year,
            period_month,
            measurements,
            overall_risk_level,
            average_risk_score: average_risk_score.to_string(), // String化
            maximum_risk_score: maximum_risk_score.to_string(), // String化
            critical_count,
            captured_at: Utc::now().to_rfc3339(),
            reviewed_by: None,
            reviewed_at: None,
        })
    }

    async fn measure_journal_integrity_risk(
        &self,
        _period_year: u32,
        _period_month: u8,
    ) -> ApplicationResult<RiskMeasurement> {
        // TODO: 実装 - 仕訳行為区分・直接訂正違反を監視
        // 現在：デモ用にダミー値を返す
        let score = BigDecimal::from_str("15.0").unwrap();
        Ok(RiskMeasurement {
            indicator_type: "JournalIntegrity".to_string(),
            name: "仕訳行為区分違反".to_string(),
            score: score.to_string(), // String化
            level: "Low".to_string(),
            details: "直接訂正違反なし".to_string(),
            count: 0,
        })
    }

    async fn measure_judgment_log_deficiency_risk(
        &self,
        _period_year: u32,
        _period_month: u8,
    ) -> ApplicationResult<RiskMeasurement> {
        // TODO: 実装 - 判断ログ・見積根拠欠如を監視
        let score = BigDecimal::from_str("25.0").unwrap();
        Ok(RiskMeasurement {
            indicator_type: "JudgmentLogDeficiency".to_string(),
            name: "判断ログ欠如".to_string(),
            score: score.to_string(),
            level: "Low".to_string(),
            details: "見積根拠ログ記録率 85%".to_string(),
            count: 3,
        })
    }

    async fn measure_carrying_amount_discrepancy_risk(
        &self,
        _period_year: u32,
        _period_month: u8,
    ) -> ApplicationResult<RiskMeasurement> {
        // TODO: 実装 - 帳簿価額・補助簿不整合を監視
        let score = BigDecimal::from_str("8.0").unwrap();
        Ok(RiskMeasurement {
            indicator_type: "CarryingAmountDiscrepancy".to_string(),
            name: "帳簿価額不整合".to_string(),
            score: score.to_string(),
            level: "Low".to_string(),
            details: "GL-補助簿整合率 100%".to_string(),
            count: 0,
        })
    }

    async fn measure_materiality_exceeded_risk(
        &self,
        _period_year: u32,
        _period_month: u8,
    ) -> ApplicationResult<RiskMeasurement> {
        // TODO: 実装 - 重要性基準超過補正を監視
        let score = BigDecimal::from_str("35.0").unwrap();
        Ok(RiskMeasurement {
            indicator_type: "MaterialityExceeded".to_string(),
            name: "重要性超過".to_string(),
            score: score.to_string(),
            level: "Low".to_string(),
            details: "重要性超過補正 2件 / CFO承認済".to_string(),
            count: 2,
        })
    }

    async fn measure_ifrs15_risk(
        &self,
        _period_year: u32,
        _period_month: u8,
    ) -> ApplicationResult<RiskMeasurement> {
        // TODO: 実装 - 収益認識5ステップ・未定義処理を監視
        let score = BigDecimal::from_str("20.0").unwrap();
        Ok(RiskMeasurement {
            indicator_type: "IFRS15Risk".to_string(),
            name: "収収基準違反".to_string(),
            score: score.to_string(),
            level: "Low".to_string(),
            details: "システム外処理 1件 / 要改善".to_string(),
            count: 1,
        })
    }

    async fn measure_ecl_stage_drift_risk(
        &self,
        _period_year: u32,
        _period_month: u8,
    ) -> ApplicationResult<RiskMeasurement> {
        // TODO: 実装 - ECLステージ遷移・信用リスクを監視
        let score = BigDecimal::from_str("45.0").unwrap();
        Ok(RiskMeasurement {
            indicator_type: "ECLStageDrift".to_string(),
            name: "ECL段階遷移".to_string(),
            score: score.to_string(),
            level: "Medium".to_string(),
            details: "Stage 1→2移行 3件 要注視".to_string(),
            count: 3,
        })
    }

    async fn measure_post_lock_adjustment_risk(
        &self,
        _period_year: u32,
        _period_month: u8,
    ) -> ApplicationResult<RiskMeasurement> {
        // TODO: 実装 - 締日固定後の補正連鎖を監視
        let score = BigDecimal::from_str("55.0").unwrap();
        Ok(RiskMeasurement {
            indicator_type: "PostLockAdjustment".to_string(),
            name: "締後修正連鎖".to_string(),
            score: score.to_string(),
            level: "High".to_string(),
            details: "ロック後補正 5件 / 影響範囲分析中".to_string(),
            count: 5,
        })
    }

    async fn measure_ias21_compliance_risk(
        &self,
        _period_year: u32,
        _period_month: u8,
    ) -> ApplicationResult<RiskMeasurement> {
        // TODO: 実装 - 外貨換算・機能通貨整合性を監視
        let score = BigDecimal::from_str("12.0").unwrap();
        Ok(RiskMeasurement {
            indicator_type: "IAS21Compliance".to_string(),
            name: "外貨換算不適合".to_string(),
            score: score.to_string(),
            level: "Low".to_string(),
            details: "貨幣性判定誤り なし".to_string(),
            count: 0,
        })
    }
}
