// 収益認識のドメインサービス

use super::{entities::Contract, values::ProgressRate};
use crate::{
    common::Amount,
    error::{DomainError, DomainResult},
};

/// 収益認識ドメインサービス
pub struct RevenueRecognitionService;

impl RevenueRecognitionService {
    /// 契約結合の要否を判定（IFRS 15 Step 1）
    pub fn should_combine_contracts(
        contracts: &[Contract],
        negotiation_package: bool,
        interdependent: bool,
    ) -> bool {
        if contracts.len() < 2 {
            return false;
        }

        // 単一の商業的実質を有する一つのパッケージとして交渉された
        if negotiation_package {
            return true;
        }

        // 一つの契約で約束した対価の金額が他の契約の価格または履行に依存している
        if interdependent {
            return true;
        }

        false
    }

    /// 履行義務の別個性を判定（IFRS 15 Step 2）
    pub fn is_distinct_good_or_service(
        can_benefit_on_its_own: bool,
        separately_identifiable: bool,
    ) -> bool {
        // 顧客が単独で、または容易に利用可能な他の資源と組み合わせて便益を享受できる
        // かつ
        // 契約における他の約束と区分して識別可能
        can_benefit_on_its_own && separately_identifiable
    }

    /// 変動対価を見積（期待値法）
    pub fn estimate_variable_consideration_expected_value(
        scenarios: &[(Amount, f64)], // (金額, 確率)
    ) -> DomainResult<Amount> {
        if scenarios.is_empty() {
            return Err(DomainError::InvalidTransactionPrice);
        }

        // 確率の合計が1.0であることを確認
        let total_probability: f64 = scenarios.iter().map(|(_, prob)| prob).sum();
        if (total_probability - 1.0).abs() > 0.01 {
            return Err(DomainError::InvalidTransactionPrice);
        }

        let expected_value: f64 = scenarios
            .iter()
            .filter_map(|(amount, prob)| amount.to_f64().map(|a| a * prob))
            .sum();

        Ok(Amount::from_i64(expected_value as i64))
    }

    /// 変動対価を見積（最頻値法）
    pub fn estimate_variable_consideration_most_likely(
        scenarios: &[(Amount, f64)], // (金額, 確率)
    ) -> DomainResult<Amount> {
        if scenarios.is_empty() {
            return Err(DomainError::InvalidTransactionPrice);
        }

        // 最も確率が高いシナリオを選択
        let most_likely = scenarios
            .iter()
            .max_by(|(_, prob1), (_, prob2)| {
                prob1.partial_cmp(prob2).unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or(DomainError::InvalidTransactionPrice)?;

        Ok(most_likely.0.clone())
    }

    /// 変動対価の制約を評価
    pub fn evaluate_constraint(
        _estimated_amount: &Amount,
        uncertainty_factors: &[String],
        experience_with_similar: bool,
    ) -> bool {
        // 重要な戻入れが生じない可能性が非常に高いかを評価

        // 不確実性要因が多い場合は制約あり
        if uncertainty_factors.len() > 3 {
            return false;
        }

        // 類似契約の経験がない場合は制約あり
        if !experience_with_similar {
            return false;
        }

        true
    }

    /// 独立販売価格を見積（調整市場評価アプローチ）
    pub fn estimate_ssp_adjusted_market_assessment(
        market_prices: &[Amount],
    ) -> DomainResult<Amount> {
        if market_prices.is_empty() {
            return Err(DomainError::InvalidStandaloneSellingPrice);
        }

        // 市場価格の平均を使用
        let sum = market_prices.iter().fold(Amount::zero(), |acc, price| acc + price.clone());
        let count = Amount::from_i64(market_prices.len() as i64);
        Ok(&sum / &count)
    }

    /// 独立販売価格を見積（予想コスト加算アプローチ）
    pub fn estimate_ssp_expected_cost_plus_margin(
        expected_cost: &Amount,
        margin_rate: u32, // パーセント
    ) -> DomainResult<Amount> {
        if expected_cost.is_negative() {
            return Err(DomainError::InvalidStandaloneSellingPrice);
        }

        if margin_rate > 100 {
            return Err(DomainError::InvalidStandaloneSellingPrice);
        }

        let margin =
            expected_cost * &Amount::from_i64(i64::from(margin_rate)) / Amount::from_i64(100);
        Ok(expected_cost + &margin)
    }

    /// 残余アプローチの適用要件を判定
    pub fn can_use_residual_approach(
        highly_variable_price: bool,
        not_yet_established_price: bool,
    ) -> bool {
        // 価格が高度に変動する、または価格がまだ設定されていない
        highly_variable_price || not_yet_established_price
    }

    /// 進捗度を測定（インプット法：コスト基準）
    pub fn measure_progress_input_method(
        costs_incurred: &Amount,
        total_expected_costs: &Amount,
    ) -> DomainResult<ProgressRate> {
        if !total_expected_costs.is_positive() {
            return Err(DomainError::InvalidRevenueRecognitionPattern);
        }

        if costs_incurred.is_negative() {
            return Err(DomainError::InvalidRevenueRecognitionPattern);
        }

        let percentage = if costs_incurred >= total_expected_costs {
            100
        } else if let (Some(incurred), Some(total)) =
            (costs_incurred.to_i64(), total_expected_costs.to_i64())
        {
            ((incurred * 100) / total) as u32
        } else {
            0
        };

        ProgressRate::new(percentage)
    }

    /// 進捗度を測定（アウトプット法：成果物基準）
    pub fn measure_progress_output_method(
        units_delivered: u32,
        total_units: u32,
    ) -> DomainResult<ProgressRate> {
        if total_units == 0 {
            return Err(DomainError::InvalidRevenueRecognitionPattern);
        }

        let percentage = if units_delivered >= total_units {
            100
        } else {
            (units_delivered * 100) / total_units
        };

        ProgressRate::new(percentage)
    }

    /// 期間認識の要件を判定
    pub fn should_recognize_over_time(
        customer_receives_and_consumes: bool,
        creates_or_enhances_customer_controlled: bool,
        no_alternative_use_and_enforceable_right: bool,
    ) -> bool {
        // 以下のいずれかを満たす場合、期間にわたり認識
        customer_receives_and_consumes
            || creates_or_enhances_customer_controlled
            || no_alternative_use_and_enforceable_right
    }

    /// 重要な金融要素の調整額を計算
    pub fn calculate_financing_adjustment(
        promised_consideration: &Amount,
        cash_selling_price: &Amount,
        discount_rate: f64,
        years: u32,
    ) -> DomainResult<Amount> {
        if !(0.0..=1.0).contains(&discount_rate) {
            return Err(DomainError::InvalidTransactionPrice);
        }

        // 現在価値を計算
        if let Some(promised_f64) = promised_consideration.to_f64() {
            let present_value = promised_f64 / (1.0 + discount_rate).powi(years as i32);
            Ok(&Amount::from_i64(present_value as i64) - cash_selling_price)
        } else {
            Err(DomainError::InvalidTransactionPrice)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::financial_close::revenue_recognition::values::{ContractId, TransactionPrice};

    #[test]
    fn test_should_combine_contracts() {
        // 契約が2つ未満の場合は結合不要
        assert!(!RevenueRecognitionService::should_combine_contracts(&[], true, false));
        assert!(!RevenueRecognitionService::should_combine_contracts(&[], false, true));
        assert!(!RevenueRecognitionService::should_combine_contracts(&[], false, false));

        // 2つ以上の契約がある場合
        let contract1 = Contract::new(
            ContractId::new(),
            "CUST001".to_string(),
            Utc::now(),
            TransactionPrice::new(
                Amount::from_i64(1_000_000),
                Amount::zero(),
                Amount::zero(),
                Amount::zero(),
            )
            .unwrap(),
        )
        .unwrap();
        let contract2 = Contract::new(
            ContractId::new(),
            "CUST001".to_string(),
            Utc::now(),
            TransactionPrice::new(
                Amount::from_i64(500_000),
                Amount::zero(),
                Amount::zero(),
                Amount::zero(),
            )
            .unwrap(),
        )
        .unwrap();
        let contracts = vec![contract1, contract2];

        // パッケージ交渉の場合は結合
        assert!(RevenueRecognitionService::should_combine_contracts(&contracts, true, false));

        // 相互依存の場合は結合
        assert!(RevenueRecognitionService::should_combine_contracts(&contracts, false, true));

        // どちらでもない場合は結合不要
        assert!(!RevenueRecognitionService::should_combine_contracts(&contracts, false, false));
    }

    #[test]
    fn test_is_distinct_good_or_service() {
        assert!(RevenueRecognitionService::is_distinct_good_or_service(true, true));
        assert!(!RevenueRecognitionService::is_distinct_good_or_service(true, false));
        assert!(!RevenueRecognitionService::is_distinct_good_or_service(false, true));
    }

    #[test]
    fn test_estimate_variable_consideration_expected_value() {
        let scenarios = vec![
            (Amount::from_i64(1_000_000), 0.5),
            (Amount::from_i64(800_000), 0.3),
            (Amount::from_i64(600_000), 0.2),
        ];

        let result =
            RevenueRecognitionService::estimate_variable_consideration_expected_value(&scenarios)
                .unwrap();

        // 1,000,000 * 0.5 + 800,000 * 0.3 + 600,000 * 0.2 = 860,000
        assert_eq!(result.to_i64(), Some(860_000));
    }

    #[test]
    fn test_estimate_variable_consideration_most_likely() {
        let scenarios = vec![
            (Amount::from_i64(1_000_000), 0.5),
            (Amount::from_i64(800_000), 0.3),
            (Amount::from_i64(600_000), 0.2),
        ];

        let result =
            RevenueRecognitionService::estimate_variable_consideration_most_likely(&scenarios)
                .unwrap();

        assert_eq!(result.to_i64(), Some(1_000_000)); // 最も確率が高い
    }

    #[test]
    fn test_estimate_ssp_adjusted_market_assessment() {
        let market_prices =
            vec![Amount::from_i64(500_000), Amount::from_i64(550_000), Amount::from_i64(480_000)];

        let result =
            RevenueRecognitionService::estimate_ssp_adjusted_market_assessment(&market_prices)
                .unwrap();

        assert_eq!(result.to_i64(), Some(510_000)); // 平均
    }

    #[test]
    fn test_estimate_ssp_expected_cost_plus_margin() {
        let result = RevenueRecognitionService::estimate_ssp_expected_cost_plus_margin(
            &Amount::from_i64(400_000),
            25,
        )
        .unwrap();

        assert_eq!(result.to_i64(), Some(500_000)); // 400,000 + 25% = 500,000
    }

    #[test]
    fn test_can_use_residual_approach() {
        assert!(RevenueRecognitionService::can_use_residual_approach(true, false));
        assert!(RevenueRecognitionService::can_use_residual_approach(false, true));
        assert!(!RevenueRecognitionService::can_use_residual_approach(false, false));
    }

    #[test]
    fn test_measure_progress_input_method() {
        let result = RevenueRecognitionService::measure_progress_input_method(
            &Amount::from_i64(300_000),
            &Amount::from_i64(1_000_000),
        )
        .unwrap();

        assert_eq!(result.percentage(), 30);
    }

    #[test]
    fn test_measure_progress_output_method() {
        let result = RevenueRecognitionService::measure_progress_output_method(30, 100).unwrap();

        assert_eq!(result.percentage(), 30);
    }

    #[test]
    fn test_should_recognize_over_time() {
        assert!(RevenueRecognitionService::should_recognize_over_time(true, false, false));
        assert!(RevenueRecognitionService::should_recognize_over_time(false, true, false));
        assert!(RevenueRecognitionService::should_recognize_over_time(false, false, true));
        assert!(!RevenueRecognitionService::should_recognize_over_time(false, false, false));
    }

    #[test]
    fn test_calculate_financing_adjustment() {
        let result = RevenueRecognitionService::calculate_financing_adjustment(
            &Amount::from_i64(1_100_000),
            &Amount::from_i64(1_000_000),
            0.05,
            2,
        )
        .unwrap();

        // 1,100,000 / (1.05)^2 ≈ 997,732
        // 997,732 - 1,000,000 ≈ -2,268
        assert!(result.is_negative());
        assert!(result.to_i64().unwrap() > -10_000);
    }
}
