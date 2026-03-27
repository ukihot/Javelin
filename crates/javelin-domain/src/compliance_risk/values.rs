// ComplianceRisk値オブジェクト

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

/// リスク指標ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskIndicatorType {
    // 1. 仕訳行為区分・直接訂正違反
    JournalIntegrity,
    // 2. 判断ログ・見積根拠欠如
    JudgmentLogDeficiency,
    // 3. 帳簿価額・補助簿不整合
    CarryingAmountDiscrepancy,
    // 4. 重要性基準超過補正
    MaterialityExceeded,
    // 5. 収益認識5ステップ・未定義処理
    IFRS15Risk,
    // 6. ECLステージ遷移・信用リスク
    ECLStageDrift,
    // 7. 締日固定後の補正連鎖
    PostLockAdjustment,
    // 8. 外貨換算・機能通貨整合性
    IAS21Compliance,
}

impl RiskIndicatorType {
    pub fn as_str(&self) -> &str {
        match self {
            RiskIndicatorType::JournalIntegrity => "Journal Integrity",
            RiskIndicatorType::JudgmentLogDeficiency => "Judgment Log",
            RiskIndicatorType::CarryingAmountDiscrepancy => "Carrying Amt",
            RiskIndicatorType::MaterialityExceeded => "Materiality",
            RiskIndicatorType::IFRS15Risk => "IFRS 15",
            RiskIndicatorType::ECLStageDrift => "ECL Drift",
            RiskIndicatorType::PostLockAdjustment => "Post-Lock",
            RiskIndicatorType::IAS21Compliance => "IAS 21",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            RiskIndicatorType::JournalIntegrity => "仕訳行為区分違反",
            RiskIndicatorType::JudgmentLogDeficiency => "判断ログ欠如",
            RiskIndicatorType::CarryingAmountDiscrepancy => "帳簿価額不整合",
            RiskIndicatorType::MaterialityExceeded => "重要性超過",
            RiskIndicatorType::IFRS15Risk => "収収基準違反",
            RiskIndicatorType::ECLStageDrift => "ECL段階遷移",
            RiskIndicatorType::PostLockAdjustment => "締後修正連鎖",
            RiskIndicatorType::IAS21Compliance => "外貨換算不適合",
        }
    }
}

/// リスク指標スコア (0.0 - 100.0) - BigDecimal で高精度管理
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RiskScore(BigDecimal);

impl RiskScore {
    pub fn new(score: BigDecimal) -> crate::error::DomainResult<Self> {
        let zero = BigDecimal::from(0);
        let hundred = BigDecimal::from(100);
        if score >= zero && score <= hundred {
            Ok(RiskScore(score))
        } else {
            Err(crate::error::DomainError::ValidationError(
                "Risk score must be between 0 and 100".to_string(),
            ))
        }
    }

    pub fn value(&self) -> BigDecimal {
        self.0.clone()
    }

    pub fn value_f64(&self) -> f64 {
        self.0.to_string().parse::<f64>().unwrap_or(0.0)
    }

    pub fn risk_level(&self) -> RiskLevel {
        let score_f64 = self.value_f64();
        match score_f64 {
            s if s <= 20.0 => RiskLevel::Low,
            s if s <= 40.0 => RiskLevel::Medium,
            s if s <= 60.0 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }

    pub fn validate(&self) -> crate::error::DomainResult<()> {
        let zero = BigDecimal::from(0);
        let hundred = BigDecimal::from(100);
        if self.0 >= zero && self.0 <= hundred {
            Ok(())
        } else {
            Err(crate::error::DomainError::ValidationError(
                "Risk score out of range".to_string(),
            ))
        }
    }
}

/// リスクレベル
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn as_str(&self) -> &str {
        match self {
            RiskLevel::Low => "Low",
            RiskLevel::Medium => "Medium",
            RiskLevel::High => "High",
            RiskLevel::Critical => "Critical",
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            RiskLevel::Low => "●",
            RiskLevel::Medium => "●",
            RiskLevel::High => "●",
            RiskLevel::Critical => "●",
        }
    }
}

/// リスク指標値
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskIndicator {
    indicator_type: RiskIndicatorType,
    score: RiskScore,
    details: String,
    count: u32,
}

impl RiskIndicator {
    pub fn new(
        indicator_type: RiskIndicatorType,
        score: RiskScore,
        details: String,
        count: u32,
    ) -> Self {
        Self { indicator_type, score, details, count }
    }

    pub fn indicator_type(&self) -> &RiskIndicatorType {
        &self.indicator_type
    }

    pub fn score(&self) -> &RiskScore {
        &self.score
    }

    pub fn details(&self) -> &str {
        &self.details
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}
