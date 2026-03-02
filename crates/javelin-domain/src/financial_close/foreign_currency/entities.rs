// 外貨換算のエンティティ

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use super::values::ForeignCurrencyTransactionId as ForeignCurrencyTransactionIdExport;
use super::values::{
    Currency, ExchangeRate, ForeignCurrencyTransactionId, FunctionalCurrency,
    MonetaryClassification,
};
use crate::{
    common::Amount,
    entity::Entity,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 外貨建取引エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignCurrencyTransaction {
    /// 取引ID
    id: ForeignCurrencyTransactionId,
    /// 機能通貨
    functional_currency: FunctionalCurrency,
    /// 外貨通貨
    foreign_currency: Currency,
    /// 外貨建金額
    foreign_amount: Amount,
    /// 取引日レート
    transaction_rate: ExchangeRate,
    /// 機能通貨換算額（取引日）
    functional_amount_at_transaction: Amount,
    /// 貨幣性・非貨幣性分類
    monetary_classification: MonetaryClassification,
    /// 勘定科目コード
    account_code: String,
    /// 取引日
    transaction_date: DateTime<Utc>,
    /// 期末評価替え済フラグ
    remeasured: bool,
    /// 期末換算額
    functional_amount_at_closing: Option<Amount>,
    /// 為替差損益
    exchange_gain_loss: Option<Amount>,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// 更新日時
    updated_at: DateTime<Utc>,
}

impl ForeignCurrencyTransaction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ForeignCurrencyTransactionId,
        functional_currency: FunctionalCurrency,
        foreign_currency: Currency,
        foreign_amount: Amount,
        transaction_rate: ExchangeRate,
        monetary_classification: MonetaryClassification,
        account_code: String,
        transaction_date: DateTime<Utc>,
    ) -> DomainResult<Self> {
        if account_code.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }

        transaction_rate.validate()?;

        let functional_amount_at_transaction = transaction_rate.convert(&foreign_amount);

        let now = Utc::now();
        Ok(Self {
            id,
            functional_currency,
            foreign_currency,
            foreign_amount,
            transaction_rate,
            functional_amount_at_transaction,
            monetary_classification,
            account_code,
            transaction_date,
            remeasured: false,
            functional_amount_at_closing: None,
            exchange_gain_loss: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// 期末評価替えを実施
    pub fn remeasure(&mut self, closing_rate: ExchangeRate) -> DomainResult<()> {
        if !self.monetary_classification.requires_remeasurement() {
            return Err(DomainError::InvalidMonetaryClassification);
        }

        closing_rate.validate()?;

        let functional_amount_at_closing = closing_rate.convert(&self.foreign_amount);
        let exchange_gain_loss =
            &functional_amount_at_closing - &self.functional_amount_at_transaction;

        self.functional_amount_at_closing = Some(functional_amount_at_closing);
        self.exchange_gain_loss = Some(exchange_gain_loss);
        self.remeasured = true;
        self.updated_at = Utc::now();

        Ok(())
    }

    /// 為替差損益を取得（評価替え済の場合）
    pub fn get_exchange_gain_loss(&self) -> Option<&Amount> {
        self.exchange_gain_loss.as_ref()
    }

    /// 期末帳簿価額を取得
    pub fn closing_carrying_amount(&self) -> Amount {
        self.functional_amount_at_closing
            .as_ref()
            .unwrap_or(&self.functional_amount_at_transaction)
            .clone()
    }

    // Getters
    pub fn id(&self) -> &ForeignCurrencyTransactionId {
        &self.id
    }

    pub fn functional_currency(&self) -> &FunctionalCurrency {
        &self.functional_currency
    }

    pub fn foreign_currency(&self) -> &Currency {
        &self.foreign_currency
    }

    pub fn foreign_amount(&self) -> &Amount {
        &self.foreign_amount
    }

    pub fn transaction_rate(&self) -> &ExchangeRate {
        &self.transaction_rate
    }

    pub fn functional_amount_at_transaction(&self) -> &Amount {
        &self.functional_amount_at_transaction
    }

    pub fn monetary_classification(&self) -> &MonetaryClassification {
        &self.monetary_classification
    }

    pub fn account_code(&self) -> &str {
        &self.account_code
    }

    pub fn transaction_date(&self) -> DateTime<Utc> {
        self.transaction_date
    }

    pub fn is_remeasured(&self) -> bool {
        self.remeasured
    }

    pub fn functional_amount_at_closing(&self) -> Option<&Amount> {
        self.functional_amount_at_closing.as_ref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Entity for ForeignCurrencyTransaction {
    type Id = ForeignCurrencyTransactionId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::values::{DeterminationBasis, ExchangeRateType},
        *,
    };

    fn create_test_transaction() -> ForeignCurrencyTransaction {
        let id = ForeignCurrencyTransactionId::new();
        let basis =
            DeterminationBasis::new(Currency::JPY, Currency::JPY, Currency::JPY, Currency::JPY);
        let functional_currency = FunctionalCurrency::new(Currency::JPY, basis);
        let transaction_rate = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
            150.0,
            ExchangeRateType::SpotRate,
            Utc::now(),
            "Central Bank".to_string(),
        )
        .unwrap();

        ForeignCurrencyTransaction::new(
            id,
            functional_currency,
            Currency::USD,
            Amount::from_i64(1_000), // 1,000 USD
            transaction_rate,
            MonetaryClassification::Monetary,
            "1100".to_string(),
            Utc::now(),
        )
        .unwrap()
    }

    #[test]
    fn test_foreign_currency_transaction_creation() {
        let transaction = create_test_transaction();
        assert_eq!(transaction.foreign_amount().to_i64(), Some(1_000));
        assert_eq!(transaction.functional_amount_at_transaction().to_i64(), Some(150_000)); // 1,000 * 150
        assert!(!transaction.is_remeasured());
    }

    #[test]
    fn test_remeasure_monetary_item() {
        let mut transaction = create_test_transaction();

        let closing_rate = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
            155.0, // レートが上昇
            ExchangeRateType::ClosingRate,
            Utc::now(),
            "Central Bank".to_string(),
        )
        .unwrap();

        assert!(transaction.remeasure(closing_rate).is_ok());
        assert!(transaction.is_remeasured());
        assert_eq!(transaction.functional_amount_at_closing().unwrap().to_i64(), Some(155_000)); // 1,000 * 155
        assert_eq!(transaction.get_exchange_gain_loss().unwrap().to_i64(), Some(5_000)); // 155,000 - 150,000
    }

    #[test]
    fn test_remeasure_non_monetary_cost_item() {
        let id = ForeignCurrencyTransactionId::new();
        let basis =
            DeterminationBasis::new(Currency::JPY, Currency::JPY, Currency::JPY, Currency::JPY);
        let functional_currency = FunctionalCurrency::new(Currency::JPY, basis);
        let transaction_rate = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
            150.0,
            ExchangeRateType::SpotRate,
            Utc::now(),
            "Central Bank".to_string(),
        )
        .unwrap();

        let mut transaction = ForeignCurrencyTransaction::new(
            id,
            functional_currency,
            Currency::USD,
            Amount::from_i64(1_000),
            transaction_rate,
            MonetaryClassification::NonMonetaryCost, // 非貨幣性項目（原価測定）
            "1500".to_string(),
            Utc::now(),
        )
        .unwrap();

        let closing_rate = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
            155.0,
            ExchangeRateType::ClosingRate,
            Utc::now(),
            "Central Bank".to_string(),
        )
        .unwrap();

        // 非貨幣性項目（原価測定）は評価替えできない
        assert!(transaction.remeasure(closing_rate).is_err());
    }

    #[test]
    fn test_closing_carrying_amount() {
        let mut transaction = create_test_transaction();

        // 評価替え前
        assert_eq!(transaction.closing_carrying_amount().to_i64(), Some(150_000));

        // 評価替え後
        let closing_rate = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
            155.0,
            ExchangeRateType::ClosingRate,
            Utc::now(),
            "Central Bank".to_string(),
        )
        .unwrap();
        transaction.remeasure(closing_rate).unwrap();

        assert_eq!(transaction.closing_carrying_amount().to_i64(), Some(155_000));
    }

    #[test]
    fn test_exchange_loss() {
        let mut transaction = create_test_transaction();

        let closing_rate = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
            145.0, // レートが下落
            ExchangeRateType::ClosingRate,
            Utc::now(),
            "Central Bank".to_string(),
        )
        .unwrap();

        transaction.remeasure(closing_rate).unwrap();
        assert_eq!(transaction.get_exchange_gain_loss().unwrap().to_i64(), Some(-5_000)); // 145,000 - 150,000 = -5,000（為替差損）
    }
}
