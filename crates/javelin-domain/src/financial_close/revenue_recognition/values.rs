// 収益認識の値オブジェクト

use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::Amount,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 契約ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContractId(Uuid);

impl ContractId {
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

impl Default for ContractId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ContractId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for ContractId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// 履行義務ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerformanceObligationId(Uuid);

impl PerformanceObligationId {
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

impl Default for PerformanceObligationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PerformanceObligationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for PerformanceObligationId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// 契約ステータス
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractStatus {
    /// 識別済
    Identified,
    /// 有効
    Active,
    /// 変更済
    Modified,
    /// 完了
    Completed,
    /// 取消
    Cancelled,
}

impl ContractStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Identified => "Identified",
            Self::Active => "Active",
            Self::Modified => "Modified",
            Self::Completed => "Completed",
            Self::Cancelled => "Cancelled",
        }
    }
}

impl fmt::Display for ContractStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ContractStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Identified" => Ok(Self::Identified),
            "Active" => Ok(Self::Active),
            "Modified" => Ok(Self::Modified),
            "Completed" => Ok(Self::Completed),
            "Cancelled" => Ok(Self::Cancelled),
            _ => Err(DomainError::InvalidContract),
        }
    }
}

/// 取引価格
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionPrice {
    /// 固定対価
    fixed_consideration: Amount,
    /// 変動対価（見積額）
    variable_consideration: Amount,
    /// 重要な金融要素の調整額
    financing_adjustment: Amount,
    /// 顧客に支払われる対価
    consideration_payable_to_customer: Amount,
}

impl TransactionPrice {
    pub fn new(
        fixed_consideration: Amount,
        variable_consideration: Amount,
        financing_adjustment: Amount,
        consideration_payable_to_customer: Amount,
    ) -> DomainResult<Self> {
        if fixed_consideration.is_negative() {
            return Err(DomainError::InvalidTransactionPrice);
        }

        Ok(Self {
            fixed_consideration,
            variable_consideration,
            financing_adjustment,
            consideration_payable_to_customer,
        })
    }

    /// 合計取引価格を計算
    pub fn total(&self) -> Amount {
        &(&(&self.fixed_consideration + &self.variable_consideration) + &self.financing_adjustment)
            - &self.consideration_payable_to_customer
    }

    pub fn fixed_consideration(&self) -> &Amount {
        &self.fixed_consideration
    }

    pub fn variable_consideration(&self) -> &Amount {
        &self.variable_consideration
    }

    pub fn financing_adjustment(&self) -> &Amount {
        &self.financing_adjustment
    }

    pub fn consideration_payable_to_customer(&self) -> &Amount {
        &self.consideration_payable_to_customer
    }
}

impl ValueObject for TransactionPrice {
    fn validate(&self) -> DomainResult<()> {
        if self.fixed_consideration.is_negative() {
            return Err(DomainError::InvalidTransactionPrice);
        }
        Ok(())
    }
}

/// 独立販売価格
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StandaloneSellingPrice {
    amount: Amount,
    estimation_method: Option<EstimationMethod>,
}

impl StandaloneSellingPrice {
    pub fn new(amount: Amount, estimation_method: Option<EstimationMethod>) -> DomainResult<Self> {
        if amount.is_negative() {
            return Err(DomainError::InvalidStandaloneSellingPrice);
        }

        Ok(Self { amount, estimation_method })
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn estimation_method(&self) -> Option<&EstimationMethod> {
        self.estimation_method.as_ref()
    }

    pub fn is_observable(&self) -> bool {
        self.estimation_method.is_none()
    }
}

impl ValueObject for StandaloneSellingPrice {
    fn validate(&self) -> DomainResult<()> {
        if self.amount.is_negative() {
            return Err(DomainError::InvalidStandaloneSellingPrice);
        }
        Ok(())
    }
}

/// 独立販売価格の見積技法
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EstimationMethod {
    /// 調整市場評価アプローチ
    AdjustedMarketAssessment,
    /// 予想コストに利益相当額を加算するアプローチ
    ExpectedCostPlusMargin { cost: Amount, margin_rate: u32 },
    /// 残余アプローチ
    Residual,
}

impl EstimationMethod {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AdjustedMarketAssessment => "AdjustedMarketAssessment",
            Self::ExpectedCostPlusMargin { .. } => "ExpectedCostPlusMargin",
            Self::Residual => "Residual",
        }
    }
}

/// 変動対価の見積方法
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariableConsiderationMethod {
    /// 期待値法
    ExpectedValue,
    /// 最頻値法
    MostLikelyAmount,
}

impl VariableConsiderationMethod {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ExpectedValue => "ExpectedValue",
            Self::MostLikelyAmount => "MostLikelyAmount",
        }
    }
}

impl fmt::Display for VariableConsiderationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 収益認識タイミング
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecognitionTiming {
    /// 一時点で認識
    PointInTime { transfer_date: DateTime<Utc> },
    /// 期間にわたり認識
    OverTime { start_date: DateTime<Utc>, end_date: DateTime<Utc> },
}

impl RecognitionTiming {
    pub fn is_point_in_time(&self) -> bool {
        matches!(self, Self::PointInTime { .. })
    }

    pub fn is_over_time(&self) -> bool {
        matches!(self, Self::OverTime { .. })
    }
}

/// 収益認識パターン（期間認識の場合）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecognitionPattern {
    /// インプット法
    InputMethod { method_description: String },
    /// アウトプット法
    OutputMethod { method_description: String },
}

impl RecognitionPattern {
    pub fn as_str(&self) -> &str {
        match self {
            Self::InputMethod { .. } => "InputMethod",
            Self::OutputMethod { .. } => "OutputMethod",
        }
    }
}

impl fmt::Display for RecognitionPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 進捗度
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressRate {
    percentage: u32, // 0-100
}

impl ProgressRate {
    pub fn new(percentage: u32) -> DomainResult<Self> {
        if percentage > 100 {
            return Err(DomainError::InvalidRevenueRecognitionPattern);
        }

        Ok(Self { percentage })
    }

    pub fn percentage(&self) -> u32 {
        self.percentage
    }

    pub fn as_decimal(&self) -> f64 {
        f64::from(self.percentage) / 100.0
    }
}

impl ValueObject for ProgressRate {
    fn validate(&self) -> DomainResult<()> {
        if self.percentage > 100 {
            return Err(DomainError::InvalidRevenueRecognitionPattern);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_id_creation() {
        let id1 = ContractId::new();
        let id2 = ContractId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_performance_obligation_id_creation() {
        let id1 = PerformanceObligationId::new();
        let id2 = PerformanceObligationId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_contract_status_from_str() {
        assert_eq!("Identified".parse::<ContractStatus>().unwrap(), ContractStatus::Identified);
        assert_eq!("Active".parse::<ContractStatus>().unwrap(), ContractStatus::Active);
        assert!("Invalid".parse::<ContractStatus>().is_err());
    }

    #[test]
    fn test_transaction_price_total() {
        let price = TransactionPrice::new(
            Amount::from_i64(1_000_000),
            Amount::from_i64(100_000),
            Amount::from_i64(50_000),
            Amount::from_i64(20_000),
        )
        .unwrap();
        assert_eq!(price.total().to_i64(), Some(1_130_000));
    }

    #[test]
    fn test_transaction_price_invalid() {
        assert!(
            TransactionPrice::new(
                Amount::from_i64(-1_000_000),
                Amount::zero(),
                Amount::zero(),
                Amount::zero()
            )
            .is_err()
        );
    }

    #[test]
    fn test_standalone_selling_price() {
        let ssp = StandaloneSellingPrice::new(Amount::from_i64(500_000), None).unwrap();
        assert_eq!(ssp.amount().to_i64(), Some(500_000));
        assert!(ssp.is_observable());
    }

    #[test]
    fn test_standalone_selling_price_with_estimation() {
        let ssp = StandaloneSellingPrice::new(
            Amount::from_i64(500_000),
            Some(EstimationMethod::ExpectedCostPlusMargin {
                cost: Amount::from_i64(400_000),
                margin_rate: 25,
            }),
        )
        .unwrap();
        assert!(!ssp.is_observable());
    }

    #[test]
    fn test_recognition_timing_point_in_time() {
        let timing = RecognitionTiming::PointInTime { transfer_date: Utc::now() };
        assert!(timing.is_point_in_time());
        assert!(!timing.is_over_time());
    }

    #[test]
    fn test_recognition_timing_over_time() {
        let timing = RecognitionTiming::OverTime {
            start_date: Utc::now(),
            end_date: Utc::now() + chrono::Duration::days(30),
        };
        assert!(!timing.is_point_in_time());
        assert!(timing.is_over_time());
    }

    #[test]
    fn test_progress_rate_valid() {
        let rate = ProgressRate::new(50).unwrap();
        assert_eq!(rate.percentage(), 50);
        assert_eq!(rate.as_decimal(), 0.5);
    }

    #[test]
    fn test_progress_rate_invalid() {
        assert!(ProgressRate::new(101).is_err());
    }

    #[test]
    fn test_progress_rate_boundary() {
        assert!(ProgressRate::new(0).is_ok());
        assert!(ProgressRate::new(100).is_ok());
    }
}
