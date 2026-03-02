// 判断ログの値オブジェクト

use std::{collections::HashMap, fmt};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{DomainError, DomainResult};

/// 判断ログID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JudgmentLogId(Uuid);

impl JudgmentLogId {
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

impl Default for JudgmentLogId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for JudgmentLogId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for JudgmentLogId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// 判断タイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JudgmentType {
    /// 収益認識（IFRS 15）
    RevenueRecognition,
    /// 期待信用損失（IFRS 9）
    ExpectedCreditLoss,
    /// 外貨換算（IAS 21）
    ForeignCurrency,
    /// 減損判定（IAS 36）
    Impairment,
    /// 公正価値測定（IFRS 13）
    FairValue,
    /// 引当金（IAS 37）
    Provision,
    /// 税効果会計（IAS 12）
    DeferredTax,
    /// リース会計（IFRS 16）
    Lease,
    /// 固定資産（IAS 16/38）
    FixedAssets,
    /// その他
    Other(String),
}

impl JudgmentType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::RevenueRecognition => "RevenueRecognition",
            Self::ExpectedCreditLoss => "ExpectedCreditLoss",
            Self::ForeignCurrency => "ForeignCurrency",
            Self::Impairment => "Impairment",
            Self::FairValue => "FairValue",
            Self::Provision => "Provision",
            Self::DeferredTax => "DeferredTax",
            Self::Lease => "Lease",
            Self::FixedAssets => "FixedAssets",
            Self::Other(s) => s,
        }
    }
}

impl fmt::Display for JudgmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for JudgmentType {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RevenueRecognition" => Ok(Self::RevenueRecognition),
            "ExpectedCreditLoss" => Ok(Self::ExpectedCreditLoss),
            "ForeignCurrency" => Ok(Self::ForeignCurrency),
            "Impairment" => Ok(Self::Impairment),
            "FairValue" => Ok(Self::FairValue),
            "Provision" => Ok(Self::Provision),
            "DeferredTax" => Ok(Self::DeferredTax),
            "Lease" => Ok(Self::Lease),
            "FixedAssets" => Ok(Self::FixedAssets),
            other => Ok(Self::Other(other.to_string())),
        }
    }
}

/// パラメータ値
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Date(DateTime<Utc>),
    Array(Vec<ParameterValue>),
}

impl ParameterValue {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Integer(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(v) => Some(*v),
            _ => None,
        }
    }
}

/// シナリオ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scenario {
    /// シナリオ名
    name: String,
    /// シナリオ説明
    description: String,
    /// 確率（0.0-1.0）
    probability: f64,
    /// パラメータ
    parameters: HashMap<String, ParameterValue>,
    /// 結果値
    result_value: Option<i64>,
}

impl Scenario {
    pub fn new(
        name: String,
        description: String,
        probability: f64,
        parameters: HashMap<String, ParameterValue>,
    ) -> DomainResult<Self> {
        if name.is_empty() {
            return Err(DomainError::InvalidJudgmentLog);
        }

        if !(0.0..=1.0).contains(&probability) {
            return Err(DomainError::InvalidJudgmentLog);
        }

        Ok(Self { name, description, probability, parameters, result_value: None })
    }

    pub fn with_result(mut self, result_value: i64) -> Self {
        self.result_value = Some(result_value);
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn probability(&self) -> f64 {
        self.probability
    }

    pub fn parameters(&self) -> &HashMap<String, ParameterValue> {
        &self.parameters
    }

    pub fn result_value(&self) -> Option<i64> {
        self.result_value
    }
}

/// 感度分析
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensitivityAnalysis {
    /// 分析対象パラメータ
    parameter_name: String,
    /// ベース値
    base_value: f64,
    /// ベース結果
    base_result: i64,
    /// 変動率（例: 0.1 = 10%）
    variation_rate: f64,
    /// 上方変動結果
    upside_result: i64,
    /// 下方変動結果
    downside_result: i64,
}

impl SensitivityAnalysis {
    pub fn new(
        parameter_name: String,
        base_value: f64,
        base_result: i64,
        variation_rate: f64,
        upside_result: i64,
        downside_result: i64,
    ) -> DomainResult<Self> {
        if parameter_name.is_empty() {
            return Err(DomainError::InvalidJudgmentLog);
        }

        if variation_rate <= 0.0 {
            return Err(DomainError::InvalidJudgmentLog);
        }

        Ok(Self {
            parameter_name,
            base_value,
            base_result,
            variation_rate,
            upside_result,
            downside_result,
        })
    }

    /// 上方感度を計算（結果の変化率）
    pub fn upside_sensitivity(&self) -> f64 {
        if self.base_result == 0 {
            return 0.0;
        }
        ((self.upside_result - self.base_result) as f64 / self.base_result as f64) * 100.0
    }

    /// 下方感度を計算（結果の変化率）
    pub fn downside_sensitivity(&self) -> f64 {
        if self.base_result == 0 {
            return 0.0;
        }
        ((self.downside_result - self.base_result) as f64 / self.base_result as f64) * 100.0
    }

    pub fn parameter_name(&self) -> &str {
        &self.parameter_name
    }

    pub fn base_value(&self) -> f64 {
        self.base_value
    }

    pub fn base_result(&self) -> i64 {
        self.base_result
    }

    pub fn variation_rate(&self) -> f64 {
        self.variation_rate
    }

    pub fn upside_result(&self) -> i64 {
        self.upside_result
    }

    pub fn downside_result(&self) -> i64 {
        self.downside_result
    }
}

/// 保存期間（7年間）
pub const RETENTION_PERIOD_YEARS: i64 = 7;

pub fn calculate_retention_expiry(judgment_date: DateTime<Utc>) -> DateTime<Utc> {
    judgment_date + Duration::days(RETENTION_PERIOD_YEARS * 365)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judgment_log_id_creation() {
        let id1 = JudgmentLogId::new();
        let id2 = JudgmentLogId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_judgment_type_from_str() {
        assert_eq!(
            "RevenueRecognition".parse::<JudgmentType>().unwrap(),
            JudgmentType::RevenueRecognition
        );
        assert_eq!(
            "ExpectedCreditLoss".parse::<JudgmentType>().unwrap(),
            JudgmentType::ExpectedCreditLoss
        );
    }

    #[test]
    fn test_parameter_value_conversions() {
        let int_val = ParameterValue::Integer(100);
        assert_eq!(int_val.as_i64(), Some(100));
        assert_eq!(int_val.as_f64(), None);

        let float_val = ParameterValue::Float(3.14);
        assert_eq!(float_val.as_f64(), Some(3.14));
        assert_eq!(float_val.as_i64(), None);

        let str_val = ParameterValue::String("test".to_string());
        assert_eq!(str_val.as_str(), Some("test"));
    }

    #[test]
    fn test_scenario_creation() {
        let mut params = HashMap::new();
        params.insert("discount_rate".to_string(), ParameterValue::Float(0.05));

        let scenario =
            Scenario::new("Base Case".to_string(), "Base scenario".to_string(), 0.6, params)
                .unwrap();

        assert_eq!(scenario.name(), "Base Case");
        assert_eq!(scenario.probability(), 0.6);
        // Scenario validation is done in constructor
    }

    #[test]
    fn test_scenario_invalid_probability() {
        let params = HashMap::new();
        let result = Scenario::new(
            "Invalid".to_string(),
            "Invalid probability".to_string(),
            1.5, // Invalid
            params,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_sensitivity_analysis() {
        let analysis = SensitivityAnalysis::new(
            "discount_rate".to_string(),
            0.05,
            1_000_000,
            0.1, // 10% variation
            1_100_000,
            900_000,
        )
        .unwrap();

        assert_eq!(analysis.parameter_name(), "discount_rate");
        assert_eq!(analysis.upside_sensitivity(), 10.0); // 10% increase
        assert_eq!(analysis.downside_sensitivity(), -10.0); // 10% decrease
    }

    #[test]
    fn test_retention_period_calculation() {
        let judgment_date = Utc::now();
        let expiry = calculate_retention_expiry(judgment_date);

        let duration = expiry - judgment_date;
        assert_eq!(duration.num_days(), RETENTION_PERIOD_YEARS * 365);
    }
}
