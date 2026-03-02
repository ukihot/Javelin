// 外貨換算のドメインサービス

use super::{
    entities::ForeignCurrencyTransaction,
    values::{Currency, MonetaryClassification},
};
use crate::error::{DomainError, DomainResult};

/// 外貨換算ドメインサービス
pub struct ForeignCurrencyService;

impl ForeignCurrencyService {
    /// 機能通貨を決定
    pub fn determine_functional_currency(
        primary_revenue_currency: &Currency,
        primary_expense_currency: &Currency,
        _financing_currency: &Currency,
        operating_cash_currency: &Currency,
    ) -> Currency {
        // 主要な収益通貨を最優先
        if primary_revenue_currency == primary_expense_currency
            && primary_revenue_currency == operating_cash_currency
        {
            return primary_revenue_currency.clone();
        }

        // 収益通貨と費用通貨が一致する場合
        if primary_revenue_currency == primary_expense_currency {
            return primary_revenue_currency.clone();
        }

        // デフォルトは収益通貨
        primary_revenue_currency.clone()
    }

    /// 貨幣性・非貨幣性を判定
    pub fn classify_monetary_item(account_code: &str) -> MonetaryClassification {
        // 勘定科目コードに基づく判定（簡易版）
        match account_code {
            // 貨幣性項目
            code if code.starts_with("11") => MonetaryClassification::Monetary, // 現金・預金
            code if code.starts_with("12") => MonetaryClassification::Monetary, // 売掛金
            code if code.starts_with("21") => MonetaryClassification::Monetary, // 買掛金
            code if code.starts_with("22") => MonetaryClassification::Monetary, // 借入金

            // 非貨幣性項目（原価測定）
            code if code.starts_with("13") => MonetaryClassification::NonMonetaryCost, // 棚卸資産
            code if code.starts_with("15") => MonetaryClassification::NonMonetaryCost, // 固定資産

            // 非貨幣性項目（公正価値測定）
            code if code.starts_with("14") => MonetaryClassification::NonMonetaryFairValue, /* 有価証券 */

            // デフォルトは貨幣性項目
            _ => MonetaryClassification::Monetary,
        }
    }

    /// 平均レートの合理性を検証
    pub fn verify_average_rate_reasonableness(
        average_rate: f64,
        spot_rates: &[f64],
        tolerance: f64,
    ) -> DomainResult<bool> {
        if spot_rates.is_empty() {
            return Err(DomainError::InvalidExchangeRate);
        }

        let calculated_average: f64 = spot_rates.iter().sum::<f64>() / spot_rates.len() as f64;
        let deviation = (average_rate - calculated_average).abs() / calculated_average;

        Ok(deviation <= tolerance)
    }

    /// 為替差損益を分析
    pub fn analyze_exchange_gain_loss(transactions: &[ForeignCurrencyTransaction]) -> (i64, i64) {
        let mut total_gain = 0i64;
        let mut total_loss = 0i64;

        for transaction in transactions {
            if let Some(gain_loss) = transaction.get_exchange_gain_loss() {
                if gain_loss > 0 {
                    total_gain += gain_loss;
                } else {
                    total_loss += gain_loss.abs();
                }
            }
        }

        (total_gain, total_loss)
    }

    /// 重要な換算差額を判定
    pub fn is_significant_exchange_difference(
        exchange_difference: i64,
        total_assets: i64,
        materiality_threshold: f64,
    ) -> bool {
        if total_assets == 0 {
            return false;
        }

        let ratio = exchange_difference.abs() as f64 / total_assets as f64;
        ratio >= materiality_threshold
    }

    /// ヘッジ会計の有効性を評価（簡易版）
    pub fn evaluate_hedge_effectiveness(
        hedged_item_change: i64,
        hedging_instrument_change: i64,
        effectiveness_range: (f64, f64), // (下限, 上限) 例: (0.8, 1.25)
    ) -> bool {
        if hedged_item_change == 0 {
            return false;
        }

        // ヘッジ対象とヘッジ手段は逆方向に動くため、絶対値で比較
        let ratio = hedging_instrument_change.abs() as f64 / hedged_item_change.abs() as f64;
        ratio >= effectiveness_range.0 && ratio <= effectiveness_range.1
    }

    /// 複数通貨の総合的な為替リスクを評価
    pub fn assess_currency_risk(
        transactions: &[ForeignCurrencyTransaction],
    ) -> Vec<(Currency, i64)> {
        use std::collections::HashMap;

        let mut exposure_by_currency: HashMap<String, i64> = HashMap::new();

        for transaction in transactions {
            let currency_key = transaction.foreign_currency().as_str().to_string();
            let exposure = transaction.foreign_amount();

            *exposure_by_currency.entry(currency_key).or_insert(0) += exposure;
        }

        exposure_by_currency
            .into_iter()
            .map(|(currency_str, amount)| {
                let currency =
                    currency_str.parse::<Currency>().unwrap_or(Currency::Other(currency_str));
                (currency, amount)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_functional_currency() {
        let currency = ForeignCurrencyService::determine_functional_currency(
            &Currency::JPY,
            &Currency::JPY,
            &Currency::JPY,
            &Currency::JPY,
        );

        assert_eq!(currency, Currency::JPY);
    }

    #[test]
    fn test_classify_monetary_item() {
        assert_eq!(
            ForeignCurrencyService::classify_monetary_item("1100"),
            MonetaryClassification::Monetary
        );
        assert_eq!(
            ForeignCurrencyService::classify_monetary_item("1300"),
            MonetaryClassification::NonMonetaryCost
        );
        assert_eq!(
            ForeignCurrencyService::classify_monetary_item("1400"),
            MonetaryClassification::NonMonetaryFairValue
        );
    }

    #[test]
    fn test_verify_average_rate_reasonableness() {
        let spot_rates = vec![150.0, 151.0, 149.0, 150.5];
        let average_rate = 150.125;

        let result = ForeignCurrencyService::verify_average_rate_reasonableness(
            average_rate,
            &spot_rates,
            0.01, // 1%の許容範囲
        )
        .unwrap();

        assert!(result);
    }

    #[test]
    fn test_is_significant_exchange_difference() {
        let exchange_difference = 10_000_000; // 1,000万円
        let total_assets = 1_000_000_000; // 10億円
        let materiality_threshold = 0.005; // 0.5%

        let is_significant = ForeignCurrencyService::is_significant_exchange_difference(
            exchange_difference,
            total_assets,
            materiality_threshold,
        );

        assert!(is_significant); // 1% > 0.5%
    }

    #[test]
    fn test_evaluate_hedge_effectiveness() {
        let hedged_item_change = -10_000_000; // 1,000万円の損失
        let hedging_instrument_change = 9_500_000; // 950万円の利益

        let is_effective = ForeignCurrencyService::evaluate_hedge_effectiveness(
            hedged_item_change,
            hedging_instrument_change,
            (0.8, 1.25),
        );

        assert!(is_effective); // 0.95 は 0.8-1.25 の範囲内
    }
}
