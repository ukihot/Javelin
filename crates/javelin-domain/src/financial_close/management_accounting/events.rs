// 管理会計のイベント

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{ConversionLogicId, ConversionType, KpiIndicator};
use crate::{common::Amount, event::DomainEvent};

/// 管理会計イベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagementAccountingEvent {
    /// イベントタイプ
    event_type: ManagementAccountingEventType,
    /// 集約ID
    aggregate_id: String,
    /// 発生日時
    occurred_at: DateTime<Utc>,
    /// バージョン
    version: u64,
}

impl ManagementAccountingEvent {
    pub fn new(
        event_type: ManagementAccountingEventType,
        aggregate_id: String,
        version: u64,
    ) -> Self {
        Self { event_type, aggregate_id, occurred_at: Utc::now(), version }
    }

    pub fn event_type(&self) -> &ManagementAccountingEventType {
        &self.event_type
    }
}

impl DomainEvent for ManagementAccountingEvent {
    fn event_type(&self) -> &str {
        self.event_type.as_str()
    }

    fn aggregate_id(&self) -> &str {
        &self.aggregate_id
    }

    fn version(&self) -> u64 {
        self.version
    }
}

/// 管理会計イベントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManagementAccountingEventType {
    /// 業況表生成
    ReportGenerated {
        /// 会計期間
        period: String,
        /// 売上高
        sales: Amount,
        /// 営業利益
        operating_profit: Amount,
        /// 損益分岐点売上高
        break_even_sales: Amount,
        /// 安全余裕率
        safety_margin_rate: f64,
    },
    /// 変換適用
    ConversionApplied {
        /// 変換ロジックID
        conversion_id: ConversionLogicId,
        /// 変換タイプ
        conversion_type: ConversionType,
        /// 変換前金額
        amount_before: Amount,
        /// 変換後金額
        amount_after: Amount,
    },
    /// 閾値超過
    ThresholdExceeded {
        /// KPI指標
        indicator: KpiIndicator,
        /// 実績値
        actual_value: f64,
        /// 閾値
        threshold: f64,
        /// 重要度
        severity: String,
    },
    /// KPI計算
    KpiCalculated {
        /// KPI指標
        indicator: KpiIndicator,
        /// 計算値
        value: f64,
        /// 会計期間
        period: String,
    },
}

impl ManagementAccountingEventType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ReportGenerated { .. } => "ReportGenerated",
            Self::ConversionApplied { .. } => "ConversionApplied",
            Self::ThresholdExceeded { .. } => "ThresholdExceeded",
            Self::KpiCalculated { .. } => "KpiCalculated",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_generated_event() {
        let event_type = ManagementAccountingEventType::ReportGenerated {
            period: "2024-01".to_string(),
            sales: Amount::from_i64(10_000_000),
            operating_profit: Amount::from_i64(3_000_000),
            break_even_sales: Amount::from_i64(5_000_000),
            safety_margin_rate: 50.0,
        };

        let event = ManagementAccountingEvent::new(event_type, "report-001".to_string(), 1);

        assert_eq!(event.event_type().as_str(), "ReportGenerated");
        assert_eq!(event.aggregate_id(), "report-001");
        assert_eq!(event.version(), 1);
    }

    #[test]
    fn test_conversion_applied_event() {
        let event_type = ManagementAccountingEventType::ConversionApplied {
            conversion_id: ConversionLogicId::new(),
            conversion_type: ConversionType::FixedCostReclassification,
            amount_before: Amount::from_i64(5_000_000),
            amount_after: Amount::from_i64(5_000_000),
        };

        let event = ManagementAccountingEvent::new(event_type, "conversion-001".to_string(), 1);

        assert_eq!(event.event_type().as_str(), "ConversionApplied");
    }

    #[test]
    fn test_threshold_exceeded_event() {
        let event_type = ManagementAccountingEventType::ThresholdExceeded {
            indicator: KpiIndicator::CurrentRatio,
            actual_value: 0.8,
            threshold: 1.0,
            severity: "Critical".to_string(),
        };

        let event = ManagementAccountingEvent::new(event_type, "threshold-001".to_string(), 1);

        assert_eq!(event.event_type().as_str(), "ThresholdExceeded");
    }

    #[test]
    fn test_kpi_calculated_event() {
        let event_type = ManagementAccountingEventType::KpiCalculated {
            indicator: KpiIndicator::ContributionMargin,
            value: 60.0,
            period: "2024-01".to_string(),
        };

        let event = ManagementAccountingEvent::new(event_type, "kpi-001".to_string(), 1);

        assert_eq!(event.event_type().as_str(), "KpiCalculated");
    }
}
