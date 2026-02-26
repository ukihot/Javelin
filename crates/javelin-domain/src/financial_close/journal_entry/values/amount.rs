// 金額関連の値オブジェクト

use std::str::FromStr;

use crate::{
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 通貨
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Currency {
    /// 日本円
    JPY,
    /// 米ドル
    USD,
    /// ユーロ
    EUR,
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JPY" => Ok(Currency::JPY),
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            _ => Err(format!("Invalid Currency: {}", s)),
        }
    }
}

impl Currency {
    /// 文字列に変換
    pub fn as_str(&self) -> &str {
        match self {
            Currency::JPY => "JPY",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
        }
    }

    /// 表示名を取得
    pub fn display_name(&self) -> &str {
        match self {
            Currency::JPY => "日本円",
            Currency::USD => "米ドル",
            Currency::EUR => "ユーロ",
        }
    }
}

impl ValueObject for Currency {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

/// 金額
///
/// 不変条件:
/// - 金額は0以上
/// - 小数点以下2桁まで
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Amount {
    /// 金額の値（整数部分 + 小数部分を100倍した整数で表現）
    value_cents: i64,
    /// 通貨
    currency: Currency,
}

impl Amount {
    /// 新しい金額を作成
    ///
    /// # Arguments
    /// * `value` - 金額（小数点以下2桁まで）
    /// * `currency` - 通貨
    ///
    /// # Errors
    /// * 金額が負の場合
    /// * 小数点以下3桁以上の場合
    pub fn new(value: f64, currency: Currency) -> DomainResult<Self> {
        if value < 0.0 {
            return Err(DomainError::InvalidAmount(
                "Amount must be greater than or equal to 0".to_string(),
            ));
        }

        // 小数点以下2桁までに丸める
        let value_cents = (value * 100.0).round() as i64;

        // 小数点以下3桁以上の精度がある場合はエラー
        let rounded_value = value_cents as f64 / 100.0;
        if (value - rounded_value).abs() > 0.001 {
            return Err(DomainError::InvalidAmount(
                "Amount precision must be up to 2 decimal places".to_string(),
            ));
        }

        let amount = Self { value_cents, currency };
        amount.validate()?;
        Ok(amount)
    }

    /// 金額の値を取得（f64として）
    pub fn value(&self) -> f64 {
        self.value_cents as f64 / 100.0
    }

    /// 金額の値を取得（セント単位の整数として）
    pub fn value_cents(&self) -> i64 {
        self.value_cents
    }

    /// 通貨を取得
    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    /// ゼロ金額を作成
    pub fn zero(currency: Currency) -> Self {
        Self { value_cents: 0, currency }
    }

    /// 金額を加算
    ///
    /// # Errors
    /// * 通貨が異なる場合
    pub fn add(&self, other: &Amount) -> DomainResult<Amount> {
        if self.currency != other.currency {
            return Err(DomainError::InvalidAmount(
                "Cannot add amounts with different currencies".to_string(),
            ));
        }

        let new_value_cents = self.value_cents + other.value_cents;
        if new_value_cents < 0 {
            return Err(DomainError::InvalidAmount(
                "Amount must be greater than or equal to 0".to_string(),
            ));
        }

        Ok(Self { value_cents: new_value_cents, currency: self.currency.clone() })
    }

    /// 金額を減算
    ///
    /// # Errors
    /// * 通貨が異なる場合
    /// * 結果が負になる場合
    pub fn subtract(&self, other: &Amount) -> DomainResult<Amount> {
        if self.currency != other.currency {
            return Err(DomainError::InvalidAmount(
                "Cannot subtract amounts with different currencies".to_string(),
            ));
        }

        let new_value_cents = self.value_cents - other.value_cents;
        if new_value_cents < 0 {
            return Err(DomainError::InvalidAmount(
                "Amount must be greater than or equal to 0".to_string(),
            ));
        }

        Ok(Self { value_cents: new_value_cents, currency: self.currency.clone() })
    }

    /// 仕訳明細行の金額として有効かを検証
    ///
    /// 会計の鉄則: 仕訳の借方・貸方の金額は必ず正の値（非ゼロ、非負）
    ///
    /// # Errors
    /// * 金額がゼロの場合
    /// * 金額が負の場合
    pub fn validate_as_journal_entry_line_amount(&self) -> DomainResult<()> {
        if self.value_cents <= 0 {
            return Err(DomainError::InvalidAmount(
                "Journal entry line amount must be positive (greater than 0)".to_string(),
            ));
        }
        Ok(())
    }
}

impl ValueObject for Amount {
    fn validate(&self) -> DomainResult<()> {
        if self.value_cents < 0 {
            return Err(DomainError::InvalidAmount(
                "Amount must be greater than or equal to 0".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_creation() {
        // 正常な金額
        let amount = Amount::new(100.50, Currency::JPY);
        assert!(amount.is_ok());
        let amount = amount.unwrap();
        assert_eq!(amount.value(), 100.50);
        assert_eq!(amount.value_cents(), 10050);
        assert_eq!(amount.currency(), &Currency::JPY);

        // ゼロ金額
        let zero = Amount::zero(Currency::USD);
        assert_eq!(zero.value(), 0.0);
        assert_eq!(zero.value_cents(), 0);

        // 負の金額はエラー
        let negative = Amount::new(-10.0, Currency::JPY);
        assert!(negative.is_err());
    }

    #[test]
    fn test_amount_precision() {
        // 小数点以下2桁まで
        let amount = Amount::new(123.45, Currency::JPY);
        assert!(amount.is_ok());

        // 小数点以下1桁
        let amount = Amount::new(123.4, Currency::JPY);
        assert!(amount.is_ok());

        // 整数
        let amount = Amount::new(123.0, Currency::JPY);
        assert!(amount.is_ok());
    }

    #[test]
    fn test_amount_addition() {
        let amount1 = Amount::new(100.50, Currency::JPY).unwrap();
        let amount2 = Amount::new(50.25, Currency::JPY).unwrap();

        let result = amount1.add(&amount2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 150.75);

        // 異なる通貨の加算はエラー
        let amount_usd = Amount::new(100.0, Currency::USD).unwrap();
        let result = amount1.add(&amount_usd);
        assert!(result.is_err());
    }

    #[test]
    fn test_amount_subtraction() {
        let amount1 = Amount::new(100.50, Currency::JPY).unwrap();
        let amount2 = Amount::new(50.25, Currency::JPY).unwrap();

        let result = amount1.subtract(&amount2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 50.25);

        // 結果が負になる減算はエラー
        let result = amount2.subtract(&amount1);
        assert!(result.is_err());

        // 異なる通貨の減算はエラー
        let amount_usd = Amount::new(100.0, Currency::USD).unwrap();
        let result = amount1.subtract(&amount_usd);
        assert!(result.is_err());
    }

    #[test]
    fn test_journal_entry_line_amount_validation() {
        // 正の金額は有効
        let positive = Amount::new(100.50, Currency::JPY).unwrap();
        assert!(positive.validate_as_journal_entry_line_amount().is_ok());

        // ゼロ金額は仕訳明細行として無効
        let zero = Amount::zero(Currency::JPY);
        assert!(zero.validate_as_journal_entry_line_amount().is_err());

        // 最小の正の金額（0.01）は有効
        let min_positive = Amount::new(0.01, Currency::JPY).unwrap();
        assert!(min_positive.validate_as_journal_entry_line_amount().is_ok());
    }

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        // 通貨生成戦略
        fn currency_strategy() -> impl Strategy<Value = Currency> {
            prop_oneof![Just(Currency::JPY), Just(Currency::USD), Just(Currency::EUR),]
        }

        // 有効な金額生成戦略（0以上、小数点以下2桁まで）
        fn valid_amount_strategy() -> impl Strategy<Value = f64> {
            (0i64..=100_000_000_i64).prop_map(|cents| cents as f64 / 100.0)
        }

        proptest! {
            // プロパティ1: 有効な金額は常に作成可能
            #[test]
            fn prop_valid_amount_creation(
                value in valid_amount_strategy(),
                currency in currency_strategy()
            ) {
                let amount = Amount::new(value, currency);
                prop_assert!(amount.is_ok());
            }

            // プロパティ2: 負の金額は常にエラー
            #[test]
            fn prop_negative_amount_fails(
                value in -1_000_000.0..-0.01,
                currency in currency_strategy()
            ) {
                let amount = Amount::new(value, currency);
                prop_assert!(amount.is_err());
            }

            // プロパティ3: 金額の値は常に非負
            #[test]
            fn prop_amount_value_non_negative(
                value in valid_amount_strategy(),
                currency in currency_strategy()
            ) {
                let amount = Amount::new(value, currency).unwrap();
                prop_assert!(amount.value() >= 0.0);
                prop_assert!(amount.value_cents() >= 0);
            }

            // プロパティ4: 加算の結合律 (a + b) + c = a + (b + c)
            #[test]
            fn prop_addition_associativity(
                a in valid_amount_strategy(),
                b in valid_amount_strategy(),
                c in valid_amount_strategy(),
                currency in currency_strategy()
            ) {
                let amount_a = Amount::new(a, currency.clone()).unwrap();
                let amount_b = Amount::new(b, currency.clone()).unwrap();
                let amount_c = Amount::new(c, currency.clone()).unwrap();

                let left = amount_a.add(&amount_b).and_then(|ab| ab.add(&amount_c));
                let right = amount_b.add(&amount_c).and_then(|bc| amount_a.add(&bc));

                if let (Ok(left_result), Ok(right_result)) = (left, right) {
                    prop_assert_eq!(left_result.value_cents(), right_result.value_cents());
                }
            }

            // プロパティ5: 加算の交換律 a + b = b + a
            #[test]
            fn prop_addition_commutativity(
                a in valid_amount_strategy(),
                b in valid_amount_strategy(),
                currency in currency_strategy()
            ) {
                let amount_a = Amount::new(a, currency.clone()).unwrap();
                let amount_b = Amount::new(b, currency.clone()).unwrap();

                let left = amount_a.add(&amount_b);
                let right = amount_b.add(&amount_a);

                if let (Ok(left_result), Ok(right_result)) = (left, right) {
                    prop_assert_eq!(left_result.value_cents(), right_result.value_cents());
                }
            }

            // プロパティ6: ゼロ元 a + 0 = a
            #[test]
            fn prop_addition_identity(
                value in valid_amount_strategy(),
                currency in currency_strategy()
            ) {
                let amount = Amount::new(value, currency.clone()).unwrap();
                let zero = Amount::zero(currency);

                let result = amount.add(&zero).unwrap();
                prop_assert_eq!(result.value_cents(), amount.value_cents());
            }

            // プロパティ7: 減算の逆元 a - a = 0
            #[test]
            fn prop_subtraction_inverse(
                value in valid_amount_strategy(),
                currency in currency_strategy()
            ) {
                let amount = Amount::new(value, currency).unwrap();
                let result = amount.subtract(&amount).unwrap();
                prop_assert_eq!(result.value_cents(), 0);
            }

            // プロパティ8: 減算後の加算 (a - b) + b = a (a >= b の場合)
            #[test]
            fn prop_subtraction_addition_inverse(
                a in valid_amount_strategy(),
                b in valid_amount_strategy(),
                currency in currency_strategy()
            ) {
                let amount_a = Amount::new(a, currency.clone()).unwrap();
                let amount_b = Amount::new(b, currency.clone()).unwrap();

                if a >= b {
                    let diff = amount_a.subtract(&amount_b).unwrap();
                    let result = diff.add(&amount_b).unwrap();
                    prop_assert_eq!(result.value_cents(), amount_a.value_cents());
                }
            }

            // プロパティ9: 異なる通貨の演算は常にエラー
            #[test]
            fn prop_different_currency_operations_fail(
                value1 in valid_amount_strategy(),
                value2 in valid_amount_strategy(),
            ) {
                let amount_jpy = Amount::new(value1, Currency::JPY).unwrap();
                let amount_usd = Amount::new(value2, Currency::USD).unwrap();

                prop_assert!(amount_jpy.add(&amount_usd).is_err());
                prop_assert!(amount_jpy.subtract(&amount_usd).is_err());
            }

            // プロパティ10: 値の往復変換 value -> value_cents -> value
            #[test]
            fn prop_value_roundtrip(
                value in valid_amount_strategy(),
                currency in currency_strategy()
            ) {
                let amount = Amount::new(value, currency).unwrap();
                let reconstructed = amount.value_cents() as f64 / 100.0;
                // 浮動小数点の誤差を考慮
                prop_assert!((amount.value() - reconstructed).abs() < 0.001);
            }

            // プロパティ11: 正の金額は仕訳明細行として常に有効
            #[test]
            fn prop_positive_amount_valid_for_journal_entry(
                value_cents in 1i64..=100_000_000_i64,
                currency in currency_strategy()
            ) {
                let value = value_cents as f64 / 100.0;
                let amount = Amount::new(value, currency).unwrap();
                prop_assert!(amount.validate_as_journal_entry_line_amount().is_ok());
            }

            // プロパティ12: ゼロ金額は仕訳明細行として常に無効
            #[test]
            fn prop_zero_amount_invalid_for_journal_entry(
                currency in currency_strategy()
            ) {
                let zero = Amount::zero(currency);
                prop_assert!(zero.validate_as_journal_entry_line_amount().is_err());
            }
        }
    }
}
