// 外貨換算の値オブジェクト

use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::Amount,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 外貨建取引ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ForeignCurrencyTransactionId(Uuid);

impl ForeignCurrencyTransactionId {
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

impl Default for ForeignCurrencyTransactionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ForeignCurrencyTransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for ForeignCurrencyTransactionId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// 通貨
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    JPY,
    USD,
    EUR,
    GBP,
    CNY,
    Other(String),
}

impl Currency {
    pub fn as_str(&self) -> &str {
        match self {
            Self::JPY => "JPY",
            Self::USD => "USD",
            Self::EUR => "EUR",
            Self::GBP => "GBP",
            Self::CNY => "CNY",
            Self::Other(code) => code,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Currency {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JPY" => Ok(Self::JPY),
            "USD" => Ok(Self::USD),
            "EUR" => Ok(Self::EUR),
            "GBP" => Ok(Self::GBP),
            "CNY" => Ok(Self::CNY),
            other => Ok(Self::Other(other.to_string())),
        }
    }
}

/// 機能通貨
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionalCurrency {
    currency: Currency,
    determination_basis: DeterminationBasis,
}

impl FunctionalCurrency {
    pub fn new(currency: Currency, determination_basis: DeterminationBasis) -> Self {
        Self { currency, determination_basis }
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn determination_basis(&self) -> &DeterminationBasis {
        &self.determination_basis
    }
}

impl ValueObject for FunctionalCurrency {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

/// 機能通貨決定根拠
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterminationBasis {
    /// 主要な収益通貨
    primary_revenue_currency: Currency,
    /// 主要な費用通貨
    primary_expense_currency: Currency,
    /// 資金調達通貨
    financing_currency: Currency,
    /// 営業資金保有通貨
    operating_cash_currency: Currency,
}

impl DeterminationBasis {
    pub fn new(
        primary_revenue_currency: Currency,
        primary_expense_currency: Currency,
        financing_currency: Currency,
        operating_cash_currency: Currency,
    ) -> Self {
        Self {
            primary_revenue_currency,
            primary_expense_currency,
            financing_currency,
            operating_cash_currency,
        }
    }

    pub fn primary_revenue_currency(&self) -> &Currency {
        &self.primary_revenue_currency
    }

    pub fn primary_expense_currency(&self) -> &Currency {
        &self.primary_expense_currency
    }

    pub fn financing_currency(&self) -> &Currency {
        &self.financing_currency
    }

    pub fn operating_cash_currency(&self) -> &Currency {
        &self.operating_cash_currency
    }
}

/// 為替レート
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExchangeRate {
    /// 基準通貨
    base_currency: Currency,
    /// 対象通貨
    target_currency: Currency,
    /// レート（1,000,000倍した整数値、例: 150.5 -> 150_500_000）
    rate_scaled: i64,
    /// レートタイプ
    rate_type: ExchangeRateType,
    /// 取得日時
    obtained_at: DateTime<Utc>,
    /// 取得元
    source: String,
}

impl ExchangeRate {
    const SCALE_FACTOR: i64 = 1_000_000;

    pub fn new(
        base_currency: Currency,
        target_currency: Currency,
        rate: f64,
        rate_type: ExchangeRateType,
        obtained_at: DateTime<Utc>,
        source: String,
    ) -> DomainResult<Self> {
        if rate <= 0.0 {
            return Err(DomainError::InvalidExchangeRate);
        }

        let rate_scaled = (rate * Self::SCALE_FACTOR as f64) as i64;

        Ok(Self { base_currency, target_currency, rate_scaled, rate_type, obtained_at, source })
    }

    /// 金額を換算
    pub fn convert(&self, amount: &Amount) -> Amount {
        use bigdecimal::BigDecimal;
        let rate_decimal =
            BigDecimal::from(self.rate_scaled) / BigDecimal::from(Self::SCALE_FACTOR);
        Amount::from(amount.value() * rate_decimal)
    }

    pub fn base_currency(&self) -> &Currency {
        &self.base_currency
    }

    pub fn target_currency(&self) -> &Currency {
        &self.target_currency
    }

    pub fn rate(&self) -> f64 {
        self.rate_scaled as f64 / Self::SCALE_FACTOR as f64
    }

    pub fn rate_type(&self) -> &ExchangeRateType {
        &self.rate_type
    }

    pub fn obtained_at(&self) -> DateTime<Utc> {
        self.obtained_at
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

impl ValueObject for ExchangeRate {
    fn validate(&self) -> DomainResult<()> {
        if self.rate_scaled <= 0 {
            return Err(DomainError::InvalidExchangeRate);
        }
        Ok(())
    }
}

/// 為替レートタイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExchangeRateType {
    /// 直物レート（取引日）
    SpotRate,
    /// 期末日レート
    ClosingRate,
    /// 平均レート
    AverageRate,
    /// 取引日レート（歴史的レート）
    HistoricalRate,
}

impl ExchangeRateType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::SpotRate => "SpotRate",
            Self::ClosingRate => "ClosingRate",
            Self::AverageRate => "AverageRate",
            Self::HistoricalRate => "HistoricalRate",
        }
    }
}

impl fmt::Display for ExchangeRateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 貨幣性・非貨幣性分類
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonetaryClassification {
    /// 貨幣性項目
    Monetary,
    /// 非貨幣性項目（原価測定）
    NonMonetaryCost,
    /// 非貨幣性項目（公正価値測定）
    NonMonetaryFairValue,
}

impl MonetaryClassification {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Monetary => "Monetary",
            Self::NonMonetaryCost => "NonMonetaryCost",
            Self::NonMonetaryFairValue => "NonMonetaryFairValue",
        }
    }

    /// 期末評価替えが必要か
    pub fn requires_remeasurement(&self) -> bool {
        matches!(self, Self::Monetary | Self::NonMonetaryFairValue)
    }

    /// 使用する為替レートタイプ
    pub fn exchange_rate_type(&self) -> ExchangeRateType {
        match self {
            Self::Monetary => ExchangeRateType::ClosingRate,
            Self::NonMonetaryCost => ExchangeRateType::HistoricalRate,
            Self::NonMonetaryFairValue => ExchangeRateType::ClosingRate,
        }
    }
}

impl fmt::Display for MonetaryClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MonetaryClassification {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monetary" => Ok(Self::Monetary),
            "NonMonetaryCost" => Ok(Self::NonMonetaryCost),
            "NonMonetaryFairValue" => Ok(Self::NonMonetaryFairValue),
            _ => Err(DomainError::InvalidMonetaryClassification),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreign_currency_transaction_id_creation() {
        let id1 = ForeignCurrencyTransactionId::new();
        let id2 = ForeignCurrencyTransactionId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_currency_from_str() {
        assert_eq!("JPY".parse::<Currency>().unwrap(), Currency::JPY);
        assert_eq!("USD".parse::<Currency>().unwrap(), Currency::USD);
        assert_eq!("EUR".parse::<Currency>().unwrap(), Currency::EUR);
    }

    #[test]
    fn test_functional_currency() {
        let basis =
            DeterminationBasis::new(Currency::JPY, Currency::JPY, Currency::JPY, Currency::JPY);
        let functional_currency = FunctionalCurrency::new(Currency::JPY, basis);

        assert_eq!(functional_currency.currency(), &Currency::JPY);
        assert!(functional_currency.validate().is_ok());
    }

    #[test]
    fn test_exchange_rate_convert() {
        let rate = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
            150.0,
            ExchangeRateType::SpotRate,
            Utc::now(),
            "Central Bank".to_string(),
        )
        .unwrap();

        assert_eq!(rate.convert(&Amount::from_i64(100)).to_i64(), Some(15_000)); // 100 USD * 150 = 15,000 JPY
    }

    #[test]
    fn test_exchange_rate_invalid() {
        let result = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
            -150.0,
            ExchangeRateType::SpotRate,
            Utc::now(),
            "Central Bank".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_monetary_classification_requires_remeasurement() {
        assert!(MonetaryClassification::Monetary.requires_remeasurement());
        assert!(!MonetaryClassification::NonMonetaryCost.requires_remeasurement());
        assert!(MonetaryClassification::NonMonetaryFairValue.requires_remeasurement());
    }

    #[test]
    fn test_monetary_classification_exchange_rate_type() {
        assert_eq!(
            MonetaryClassification::Monetary.exchange_rate_type(),
            ExchangeRateType::ClosingRate
        );
        assert_eq!(
            MonetaryClassification::NonMonetaryCost.exchange_rate_type(),
            ExchangeRateType::HistoricalRate
        );
        assert_eq!(
            MonetaryClassification::NonMonetaryFairValue.exchange_rate_type(),
            ExchangeRateType::ClosingRate
        );
    }

    #[test]
    fn test_monetary_classification_from_str() {
        assert_eq!(
            "Monetary".parse::<MonetaryClassification>().unwrap(),
            MonetaryClassification::Monetary
        );
        assert_eq!(
            "NonMonetaryCost".parse::<MonetaryClassification>().unwrap(),
            MonetaryClassification::NonMonetaryCost
        );
        assert!("Invalid".parse::<MonetaryClassification>().is_err());
    }
}
