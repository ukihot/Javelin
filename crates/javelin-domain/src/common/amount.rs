// 金額値オブジェクト
// 会計ドメインにおける金額を表現する共通の値オブジェクト

use std::{
    fmt,
    ops::{Add, Neg, Sub},
    str::FromStr,
};

use bigdecimal::{BigDecimal, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 金額
///
/// 会計ドメインにおける金額を表現する値オブジェクト。
/// BigDecimalを使用することで、以下の問題を解決：
///
/// 1. 精度の保証：浮動小数点の丸め誤差を回避
/// 2. 符号付き演算：借方・貸方、差額計算、戻し処理に対応
/// 3. 外部連携：銀行API、会計APIとの互換性
///
/// # 設計原則
///
/// - 型ではなくバリデーションで制約を表現
/// - 差分計算で破綻しない（a - b が a < b でも安全）
/// - 複式簿記の符号的概念に対応
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Amount(BigDecimal);

impl Amount {
    /// ゼロ金額を作成
    pub fn zero() -> Self {
        Self(BigDecimal::zero())
    }

    /// 整数から金額を作成
    pub fn from_i64(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }

    /// 文字列から金額を作成
    pub fn from_str_value(value: &str) -> DomainResult<Self> {
        let decimal = BigDecimal::from_str(value)
            .map_err(|_| DomainError::InvalidAmount(format!("Invalid amount format: {}", value)))?;
        Ok(Self(decimal))
    }

    /// 正の金額であることを検証して作成
    pub fn positive(value: BigDecimal) -> DomainResult<Self> {
        if value <= BigDecimal::zero() {
            return Err(DomainError::InvalidAmount("Amount must be positive".to_string()));
        }
        Ok(Self(value))
    }

    /// 非負の金額であることを検証して作成
    pub fn non_negative(value: BigDecimal) -> DomainResult<Self> {
        if value < BigDecimal::zero() {
            return Err(DomainError::InvalidAmount("Amount must be non-negative".to_string()));
        }
        Ok(Self(value))
    }

    /// 内部値への参照を取得
    pub fn value(&self) -> &BigDecimal {
        &self.0
    }

    /// 内部値を取得
    pub fn into_inner(self) -> BigDecimal {
        self.0
    }

    /// ゼロかどうかを判定
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// 正の値かどうかを判定
    pub fn is_positive(&self) -> bool {
        self.0 > BigDecimal::zero()
    }

    /// 負の値かどうかを判定
    pub fn is_negative(&self) -> bool {
        self.0 < BigDecimal::zero()
    }

    /// 絶対値を取得
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    /// i64に変換（精度が失われる可能性あり）
    pub fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    /// f64に変換（精度が失われる可能性あり）
    pub fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl ValueObject for Amount {
    fn validate(&self) -> DomainResult<()> {
        // 金額自体に制約はない（正負どちらも許容）
        Ok(())
    }
}

impl From<i64> for Amount {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl From<BigDecimal> for Amount {
    fn from(value: BigDecimal) -> Self {
        Self(value)
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add for &Amount {
    type Output = Amount;

    fn add(self, rhs: Self) -> Self::Output {
        Amount(self.0.clone() + rhs.0.clone())
    }
}

impl Sub for Amount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub for &Amount {
    type Output = Amount;

    fn sub(self, rhs: Self) -> Self::Output {
        Amount(self.0.clone() - rhs.0.clone())
    }
}

impl Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Neg for &Amount {
    type Output = Amount;

    fn neg(self) -> Self::Output {
        Amount(-self.0.clone())
    }
}

impl std::ops::Mul for Amount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Mul for &Amount {
    type Output = Amount;

    fn mul(self, rhs: Self) -> Self::Output {
        Amount(self.0.clone() * rhs.0.clone())
    }
}

impl std::ops::Div for Amount {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::ops::Div for &Amount {
    type Output = Amount;

    fn div(self, rhs: Self) -> Self::Output {
        Amount(self.0.clone() / rhs.0.clone())
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Amount {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_value(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_zero() {
        let amount = Amount::zero();
        assert!(amount.is_zero());
        assert!(!amount.is_positive());
        assert!(!amount.is_negative());
    }

    #[test]
    fn test_amount_from_i64() {
        let amount = Amount::from_i64(1000);
        assert!(amount.is_positive());
        assert_eq!(amount.to_i64(), Some(1000));
    }

    #[test]
    fn test_amount_negative() {
        let amount = Amount::from_i64(-500);
        assert!(amount.is_negative());
        assert_eq!(amount.to_i64(), Some(-500));
    }

    #[test]
    fn test_amount_addition() {
        let a = Amount::from_i64(1000);
        let b = Amount::from_i64(500);
        let result = a + b;
        assert_eq!(result.to_i64(), Some(1500));
    }

    #[test]
    fn test_amount_subtraction() {
        let a = Amount::from_i64(1000);
        let b = Amount::from_i64(1500);
        let result = a - b; // 負の結果でもOK
        assert_eq!(result.to_i64(), Some(-500));
        assert!(result.is_negative());
    }

    #[test]
    fn test_amount_negation() {
        let amount = Amount::from_i64(1000);
        let negated = -amount;
        assert_eq!(negated.to_i64(), Some(-1000));
    }

    #[test]
    fn test_amount_abs() {
        let amount = Amount::from_i64(-1000);
        let abs = amount.abs();
        assert_eq!(abs.to_i64(), Some(1000));
    }

    #[test]
    fn test_amount_positive_validation() {
        let result = Amount::positive(BigDecimal::from(1000));
        assert!(result.is_ok());

        let result = Amount::positive(BigDecimal::zero());
        assert!(result.is_err());

        let result = Amount::positive(BigDecimal::from(-100));
        assert!(result.is_err());
    }

    #[test]
    fn test_amount_non_negative_validation() {
        let result = Amount::non_negative(BigDecimal::from(1000));
        assert!(result.is_ok());

        let result = Amount::non_negative(BigDecimal::zero());
        assert!(result.is_ok());

        let result = Amount::non_negative(BigDecimal::from(-100));
        assert!(result.is_err());
    }

    #[test]
    fn test_amount_from_str() {
        let amount = Amount::from_str("1234.56").unwrap();
        assert!(amount.is_positive());

        let amount = Amount::from_str("-1234.56").unwrap();
        assert!(amount.is_negative());
    }

    #[test]
    fn test_amount_display() {
        let amount = Amount::from_i64(1000);
        assert_eq!(format!("{}", amount), "1000");
    }

    #[test]
    fn test_amount_reference_operations() {
        let a = Amount::from_i64(1000);
        let b = Amount::from_i64(500);
        let result = &a + &b;
        assert_eq!(result.to_i64(), Some(1500));

        let result = &a - &b;
        assert_eq!(result.to_i64(), Some(500));

        let result = -&a;
        assert_eq!(result.to_i64(), Some(-1000));
    }
}
