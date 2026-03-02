// 管理会計の値オブジェクト

use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 変換ロジックID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConversionLogicId(Uuid);

impl ConversionLogicId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ConversionLogicId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ConversionLogicId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for ConversionLogicId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// 変換タイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversionType {
    /// 固定費再分類
    FixedCostReclassification,
    /// 変動費識別
    VariableCostIdentification,
    /// 共通費配賦
    CommonCostAllocation,
    /// 投資性支出識別
    InvestmentExpenditureIdentification,
    /// 非経常項目分離
    NonRecurringItemSeparation,
}

impl ConversionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::FixedCostReclassification => "FixedCostReclassification",
            Self::VariableCostIdentification => "VariableCostIdentification",
            Self::CommonCostAllocation => "CommonCostAllocation",
            Self::InvestmentExpenditureIdentification => "InvestmentExpenditureIdentification",
            Self::NonRecurringItemSeparation => "NonRecurringItemSeparation",
        }
    }
}

impl fmt::Display for ConversionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// KPI指標
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KpiIndicator {
    /// 売上総利益率
    GrossProfitMargin,
    /// 限界利益率
    ContributionMargin,
    /// 営業利益率
    OperatingProfitMargin,
    /// 部門別ROI
    DepartmentROI,
    /// キャッシュ保有月数
    CashHoldingMonths,
    /// 営業CF比率
    OperatingCashFlowRatio,
    /// 流動比率
    CurrentRatio,
    /// 純有利子負債倍率
    NetDebtMultiple,
    /// ROIC
    ROIC,
    /// 損益分岐点売上高
    BreakEvenSales,
    /// 安全余裕率
    SafetyMarginRate,
}

impl KpiIndicator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GrossProfitMargin => "GrossProfitMargin",
            Self::ContributionMargin => "ContributionMargin",
            Self::OperatingProfitMargin => "OperatingProfitMargin",
            Self::DepartmentROI => "DepartmentROI",
            Self::CashHoldingMonths => "CashHoldingMonths",
            Self::OperatingCashFlowRatio => "OperatingCashFlowRatio",
            Self::CurrentRatio => "CurrentRatio",
            Self::NetDebtMultiple => "NetDebtMultiple",
            Self::ROIC => "ROIC",
            Self::BreakEvenSales => "BreakEvenSales",
            Self::SafetyMarginRate => "SafetyMarginRate",
        }
    }
}

impl fmt::Display for KpiIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// KPI閾値
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KpiThreshold {
    /// 指標
    indicator: KpiIndicator,
    /// 警告閾値
    warning_threshold: f64,
    /// 危険閾値
    critical_threshold: f64,
    /// 閾値タイプ（上限/下限）
    is_upper_limit: bool,
}

impl Eq for KpiThreshold {}

impl KpiThreshold {
    pub fn new(
        indicator: KpiIndicator,
        warning_threshold: f64,
        critical_threshold: f64,
        is_upper_limit: bool,
    ) -> DomainResult<Self> {
        if is_upper_limit && warning_threshold > critical_threshold {
            return Err(DomainError::InvalidManagementAccounting);
        }

        if !is_upper_limit && warning_threshold < critical_threshold {
            return Err(DomainError::InvalidManagementAccounting);
        }

        Ok(Self { indicator, warning_threshold, critical_threshold, is_upper_limit })
    }

    /// 値を評価
    pub fn evaluate(&self, value: f64) -> ThresholdStatus {
        if self.is_upper_limit {
            if value >= self.critical_threshold {
                ThresholdStatus::Critical
            } else if value >= self.warning_threshold {
                ThresholdStatus::Warning
            } else {
                ThresholdStatus::Normal
            }
        } else if value <= self.critical_threshold {
            ThresholdStatus::Critical
        } else if value <= self.warning_threshold {
            ThresholdStatus::Warning
        } else {
            ThresholdStatus::Normal
        }
    }

    pub fn indicator(&self) -> &KpiIndicator {
        &self.indicator
    }

    pub fn warning_threshold(&self) -> f64 {
        self.warning_threshold
    }

    pub fn critical_threshold(&self) -> f64 {
        self.critical_threshold
    }

    pub fn is_upper_limit(&self) -> bool {
        self.is_upper_limit
    }
}

impl ValueObject for KpiThreshold {
    fn validate(&self) -> DomainResult<()> {
        if self.is_upper_limit && self.warning_threshold > self.critical_threshold {
            return Err(DomainError::InvalidManagementAccounting);
        }

        if !self.is_upper_limit && self.warning_threshold < self.critical_threshold {
            return Err(DomainError::InvalidManagementAccounting);
        }

        Ok(())
    }
}

/// 閾値ステータス
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThresholdStatus {
    Normal,
    Warning,
    Critical,
}

/// 安全性指標
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyIndicator {
    /// キャッシュ保有月数
    pub cash_holding_months: f64,
    /// 営業CF比率
    pub operating_cf_ratio: f64,
    /// 流動比率
    pub current_ratio: f64,
    /// 純有利子負債倍率
    pub net_debt_multiple: f64,
}

impl SafetyIndicator {
    pub fn new(
        cash_holding_months: f64,
        operating_cf_ratio: f64,
        current_ratio: f64,
        net_debt_multiple: f64,
    ) -> Self {
        Self { cash_holding_months, operating_cf_ratio, current_ratio, net_debt_multiple }
    }

    /// 総合安全性スコアを計算（0-100）
    pub fn calculate_safety_score(&self) -> f64 {
        let mut score = 0.0;

        // キャッシュ保有月数（最大25点）
        score += (self.cash_holding_months.min(6.0) / 6.0) * 25.0;

        // 営業CF比率（最大25点）
        score += (self.operating_cf_ratio.min(1.0) / 1.0) * 25.0;

        // 流動比率（最大25点）
        score += (self.current_ratio - 1.0).clamp(0.0, 1.0) * 25.0;

        // 純有利子負債倍率（最大25点、低いほど良い）
        let debt_score = if self.net_debt_multiple <= 0.0 {
            25.0
        } else {
            (1.0 - (self.net_debt_multiple / 5.0).min(1.0)) * 25.0
        };
        score += debt_score;

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_logic_id_creation() {
        let id1 = ConversionLogicId::new();
        let id2 = ConversionLogicId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_kpi_threshold_upper_limit() {
        let threshold = KpiThreshold::new(
            KpiIndicator::NetDebtMultiple,
            3.0,  // Warning
            5.0,  // Critical
            true, // Upper limit
        )
        .unwrap();

        assert_eq!(threshold.evaluate(2.0), ThresholdStatus::Normal);
        assert_eq!(threshold.evaluate(4.0), ThresholdStatus::Warning);
        assert_eq!(threshold.evaluate(6.0), ThresholdStatus::Critical);
    }

    #[test]
    fn test_kpi_threshold_lower_limit() {
        let threshold = KpiThreshold::new(
            KpiIndicator::CurrentRatio,
            1.5,   // Warning
            1.0,   // Critical
            false, // Lower limit
        )
        .unwrap();

        assert_eq!(threshold.evaluate(2.0), ThresholdStatus::Normal);
        assert_eq!(threshold.evaluate(1.3), ThresholdStatus::Warning);
        assert_eq!(threshold.evaluate(0.8), ThresholdStatus::Critical);
    }

    #[test]
    fn test_safety_indicator_score() {
        let indicator = SafetyIndicator::new(
            6.0, // 6 months cash
            0.8, // 80% operating CF ratio
            2.0, // 200% current ratio
            1.0, // 1x net debt
        );

        let score = indicator.calculate_safety_score();
        assert!(score > 80.0); // Should have high safety score
    }

    #[test]
    fn test_safety_indicator_low_score() {
        let indicator = SafetyIndicator::new(
            1.0, // 1 month cash
            0.2, // 20% operating CF ratio
            0.8, // 80% current ratio
            5.0, // 5x net debt
        );

        let score = indicator.calculate_safety_score();
        assert!(score < 30.0); // Should have low safety score
    }
}
