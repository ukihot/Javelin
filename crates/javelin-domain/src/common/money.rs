// Money - 通貨付き金額の値オブジェクト

use std::str::FromStr;

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::error::{DomainError, DomainResult};

/// 通貨コード
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    JPY, // 日本円
    USD, // 米ドル
    EUR, // ユーロ
    GBP, // 英ポンド
    CNY, // 人民元
}

impl Currency {
    pub fn as_str(&self) -> &str {
        match self {
            Currency::JPY => "JPY",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::CNY => "CNY",
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            Currency::JPY => "¥",
            Currency::USD => "$",
            Currency::EUR => "€",
            Currency::GBP => "£",
            Currency::CNY => "¥",
        }
    }

    pub fn decimal_places(&self) -> u32 {
        match self {
            Currency::JPY => 0, // 円は小数点なし
            Currency::USD | Currency::EUR | Currency::GBP | Currency::CNY => 2,
        }
    }
}

impl FromStr for Currency {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JPY" => Ok(Currency::JPY),
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "CNY" => Ok(Currency::CNY),
            _ => Err(DomainError::ValidationError(format!("Invalid currency code: {}", s))),
        }
    }
}

/// Money - 通貨付き金額
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    amount: BigDecimal,
    currency: Currency,
}

impl Money {
    /// 新しいMoneyインスタンスを作成
    pub fn new(amount: BigDecimal, currency: Currency) -> DomainResult<Self> {
        // 通貨の小数点桁数に合わせて丸める
        let scale = currency.decimal_places() as i64;
        let rounded = amount.with_scale(scale);

        Ok(Self { amount: rounded, currency })
    }

    /// 金額から作成（文字列）
    pub fn from_str(amount_str: &str, currency: Currency) -> DomainResult<Self> {
        let amount = BigDecimal::from_str(amount_str)
            .map_err(|e| DomainError::ValidationError(format!("Invalid amount: {}", e)))?;
        Self::new(amount, currency)
    }

    /// 金額から作成（整数）
    pub fn from_i64(amount: i64, currency: Currency) -> DomainResult<Self> {
        let amount = BigDecimal::from(amount);
        Self::new(amount, currency)
    }

    /// ゼロ金額を作成
    pub fn zero(currency: Currency) -> Self {
        Self { amount: BigDecimal::from(0), currency }
    }

    /// 金額を取得
    pub fn amount(&self) -> &BigDecimal {
        &self.amount
    }

    /// 通貨を取得
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// 金額が正かどうか
    pub fn is_positive(&self) -> bool {
        self.amount > 0
    }

    /// 金額が負かどうか
    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }

    /// 金額がゼロかどうか
    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }

    /// 加算
    pub fn add(&self, other: &Money) -> DomainResult<Money> {
        if self.currency != other.currency {
            return Err(DomainError::ValidationError(format!(
                "Cannot add different currencies: {} and {}",
                self.currency.as_str(),
                other.currency.as_str()
            )));
        }

        Money::new(&self.amount + &other.amount, self.currency)
    }

    /// 減算
    pub fn subtract(&self, other: &Money) -> DomainResult<Money> {
        if self.currency != other.currency {
            return Err(DomainError::ValidationError(format!(
                "Cannot subtract different currencies: {} and {}",
                self.currency.as_str(),
                other.currency.as_str()
            )));
        }

        Money::new(&self.amount - &other.amount, self.currency)
    }

    /// 乗算
    pub fn multiply(&self, factor: &BigDecimal) -> DomainResult<Money> {
        Money::new(&self.amount * factor, self.currency)
    }

    /// 除算
    pub fn divide(&self, divisor: &BigDecimal) -> DomainResult<Money> {
        if divisor == &BigDecimal::from(0) {
            return Err(DomainError::ValidationError("Cannot divide by zero".to_string()));
        }

        Money::new(&self.amount / divisor, self.currency)
    }

    /// 絶対値
    pub fn abs(&self) -> Money {
        Money { amount: self.amount.abs(), currency: self.currency }
    }

    /// 符号反転
    pub fn negate(&self) -> Money {
        Money { amount: -&self.amount, currency: self.currency }
    }

    /// 文字列表現
    pub fn to_string(&self) -> String {
        format!("{} {}", self.currency.symbol(), self.amount)
    }
}

/// 金額のAmount（通貨はサポートしない単位）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Amount(BigDecimal);

impl Amount {
    pub fn from_i64(value: i64) -> Self {
        Amount(BigDecimal::from(value))
    }

    pub fn from(value: BigDecimal) -> Self {
        Amount(value)
    }

    pub fn zero() -> Self {
        Amount(BigDecimal::from(0))
    }

    pub fn value(&self) -> &BigDecimal {
        &self.0
    }
}

impl std::ops::Add for Amount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Amount(self.0 + other.0)
    }
}

impl<'a> std::ops::Add<&'a Amount> for Amount {
    type Output = Amount;

    fn add(self, other: &'a Amount) -> Amount {
        Amount(self.0 + &other.0)
    }
}

impl std::ops::Add<Amount> for &Amount {
    type Output = Amount;

    fn add(self, other: Amount) -> Amount {
        Amount(&self.0 + other.0)
    }
}

impl<'b> std::ops::Add<&'b Amount> for &Amount {
    type Output = Amount;

    fn add(self, other: &'b Amount) -> Amount {
        Amount(&self.0 + &other.0)
    }
}

impl std::ops::Sub for Amount {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Amount(self.0 - other.0)
    }
}

impl<'a> std::ops::Sub<&'a Amount> for Amount {
    type Output = Amount;

    fn sub(self, other: &'a Amount) -> Amount {
        Amount(self.0 - &other.0)
    }
}

impl std::ops::Sub<Amount> for &Amount {
    type Output = Amount;

    fn sub(self, other: Amount) -> Amount {
        Amount(&self.0 - other.0)
    }
}

impl<'b> std::ops::Sub<&'b Amount> for &Amount {
    type Output = Amount;

    fn sub(self, other: &'b Amount) -> Amount {
        Amount(&self.0 - &other.0)
    }
}

impl std::ops::AddAssign for Amount {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl std::ops::SubAssign for Amount {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_creation() {
        let money = Money::from_i64(1000, Currency::JPY).unwrap();
        assert_eq!(money.amount(), &BigDecimal::from(1000));
        assert_eq!(money.currency(), Currency::JPY);
    }

    #[test]
    fn test_money_addition() {
        let m1 = Money::from_i64(1000, Currency::JPY).unwrap();
        let m2 = Money::from_i64(500, Currency::JPY).unwrap();
        let result = m1.add(&m2).unwrap();
        assert_eq!(result.amount(), &BigDecimal::from(1500));
    }

    #[test]
    fn test_money_different_currency_error() {
        let m1 = Money::from_i64(1000, Currency::JPY).unwrap();
        let m2 = Money::from_i64(500, Currency::USD).unwrap();
        assert!(m1.add(&m2).is_err());
    }

    #[test]
    fn test_money_zero() {
        let money = Money::zero(Currency::JPY);
        assert!(money.is_zero());
        assert!(!money.is_positive());
        assert!(!money.is_negative());
    }

    #[test]
    fn test_currency_decimal_places() {
        assert_eq!(Currency::JPY.decimal_places(), 0);
        assert_eq!(Currency::USD.decimal_places(), 2);
    }
}
