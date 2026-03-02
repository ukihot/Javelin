// 重要性基準のドメインサービス

use super::{
    entities::MaterialityJudgment,
    values::{EstimateParameter, QualitativeFactor, QuantitativeThreshold, SensitivityAnalysisResult},
};
use crate::{
    common::Amount,
    error::{DomainError, DomainResult},
};

/// 重要性判定ドメインサービス
pub struct MaterialityService;

impl MaterialityService {
    /// 金額的重要性を自動判定
    pub fn judge_quantitative(
        amount: &Amount,
        pretax_income: &Amount,
        total_assets: &Amount,
        revenue: &Amount,
        equity: &Amount,
        control_method: String,
        judged_by: String,
    ) -> DomainResult<MaterialityJudgment> {
        let threshold = QuantitativeThreshold::new(pretax_income, total_assets, revenue, equity);

        MaterialityJudgment::new_quantitative(threshold, amount, control_method, judged_by)
    }

    /// 質的重要性を判定
    pub fn judge_qualitative(
        factors: Vec<QualitativeFactor>,
        control_method: String,
        judged_by: String,
    ) -> DomainResult<MaterialityJudgment> {
        MaterialityJudgment::new_qualitative(factors, control_method, judged_by)
    }

    /// 見積重要性を判定（感度分析付き）
    pub fn judge_estimate_with_sensitivity<F>(
        parameters: Vec<EstimateParameter>,
        calculation_fn: F,
        quantitative_threshold: &QuantitativeThreshold,
        control_method: String,
        judged_by: String,
    ) -> DomainResult<MaterialityJudgment>
    where
        F: Fn(&[Amount]) -> Amount,
    {
        let mut judgment =
            MaterialityJudgment::new_estimate(parameters.clone(), control_method, judged_by)?;

        // 各パラメータの感度分析を実施
        for (idx, param) in parameters.iter().enumerate() {
            let mut base_values: Vec<Amount> = parameters.iter().map(|p| p.base_value().clone()).collect();
            let mut upper_values = base_values.clone();
            let mut lower_values = base_values.clone();

            // 対象パラメータのみ変動
            upper_values[idx] = param.upper_bound();
            lower_values[idx] = param.lower_bound();

            // 計算実行
            let base_result = calculation_fn(&base_values);
            let upper_result = calculation_fn(&upper_values);
            let lower_result = calculation_fn(&lower_values);

            let sensitivity = SensitivityAnalysisResult::new(
                param.name().to_string(),
                base_result,
                upper_result,
                lower_result,
            );

            judgment.add_sensitivity_result(sensitivity)?;
        }

        // 重要性を評価
        judgment.evaluate_sensitivity(quantitative_threshold)?;

        Ok(judgment)
    }

    /// 複数の金額的重要性判定を一括実施
    pub fn batch_judge_quantitative(
        amounts: &[(String, Amount)],
        pretax_income: &Amount,
        total_assets: &Amount,
        revenue: &Amount,
        equity: &Amount,
        judged_by: String,
    ) -> DomainResult<Vec<(String, MaterialityJudgment)>> {
        let threshold = QuantitativeThreshold::new(pretax_income, total_assets, revenue, equity);

        let mut results = Vec::new();

        for (description, amount) in amounts {
            let judgment = MaterialityJudgment::new_quantitative(
                threshold.clone(),
                amount,
                format!("一括判定: {}", description),
                judged_by.clone(),
            )?;

            results.push((description.clone(), judgment));
        }

        Ok(results)
    }

    /// 閾値超過率を計算
    pub fn calculate_excess_ratio(amount: &Amount, threshold: &Amount) -> DomainResult<f64> {
        if threshold.is_zero() {
            return Err(DomainError::InvalidMateriality);
        }

        if let (Some(amt), Some(thr)) = (amount.abs().to_f64(), threshold.to_f64()) {
            Ok(amt / thr)
        } else {
            Err(DomainError::InvalidMateriality)
        }
    }

    /// 承認ルートを決定
    pub fn determine_approval_route(judgment: &MaterialityJudgment) -> Vec<String> {
        use super::entities::ApprovalLevel;

        match judgment.approval_level() {
            ApprovalLevel::Staff => vec![],
            ApprovalLevel::Manager => vec!["課長".to_string()],
            ApprovalLevel::Director => vec!["課長".to_string(), "部長".to_string()],
            ApprovalLevel::CFO => {
                vec!["課長".to_string(), "部長".to_string(), "CFO".to_string()]
            }
            ApprovalLevel::Board => vec![
                "課長".to_string(),
                "部長".to_string(),
                "CFO".to_string(),
                "取締役会".to_string(),
            ],
        }
    }

    /// 重要性判定の整合性を検証
    pub fn verify_judgment_consistency(judgment: &MaterialityJudgment) -> DomainResult<bool> {
        // 承認が必要な判定は承認されているか
        if judgment.requires_approval() && !judgment.is_approved() {
            return Ok(false);
        }

        // 見積判定の場合、感度分析が実施されているか
        if judgment.materiality_type() == &super::values::MaterialityType::Estimate
            && judgment.sensitivity_results().is_empty()
        {
            return Ok(false);
        }

        Ok(true)
    }

    /// 質的要因の重大性を評価
    pub fn assess_qualitative_severity(factors: &[QualitativeFactor]) -> u32 {
        let mut severity_score = 0;

        for factor in factors {
            severity_score += match factor {
                QualitativeFactor::ManagementFraud => 100,
                QualitativeFactor::LegalViolation => 90,
                QualitativeFactor::GoingConcernUncertainty => 95,
                QualitativeFactor::Litigation => 60,
                QualitativeFactor::AccountingPolicyChange => 50,
                QualitativeFactor::RelatedPartyTransaction => 40,
                QualitativeFactor::SubsequentEvent => 30,
                QualitativeFactor::Other(_) => 20,
            };
        }

        severity_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_financials() -> (Amount, Amount, Amount, Amount) {
        (
            Amount::from_i64(1_000_000_000), // 税引前利益10億円
            Amount::from_i64(10_000_000_000), // 総資産100億円
            Amount::from_i64(5_000_000_000), // 売上高50億円
            Amount::from_i64(3_000_000_000), // 純資産30億円
        )
    }

    #[test]
    fn test_judge_quantitative() {
        let (pretax_income, total_assets, revenue, equity) = create_test_financials();
        let amount = Amount::from_i64(30_000_000);

        let judgment = MaterialityService::judge_quantitative(
            &amount,
            &pretax_income,
            &total_assets,
            &revenue,
            &equity,
            "補正仕訳".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        assert!(judgment.requires_adjustment());
        assert!(judgment.requires_approval());
    }

    #[test]
    fn test_judge_qualitative() {
        let factors = vec![
            QualitativeFactor::AccountingPolicyChange,
            QualitativeFactor::RelatedPartyTransaction,
        ];

        let judgment = MaterialityService::judge_qualitative(
            factors,
            "開示強化".to_string(),
            "経理部長".to_string(),
        )
        .unwrap();

        assert!(judgment.requires_adjustment());
        assert!(judgment.requires_approval());
    }

    #[test]
    fn test_judge_estimate_with_sensitivity() {
        let params = vec![
            EstimateParameter::new("割引率".to_string(), Amount::from_i64(5)).unwrap(),
            EstimateParameter::new("成長率".to_string(), Amount::from_i64(3)).unwrap(),
        ];

        // 簡単な計算関数（割引率 × 成長率 × 1億円）
        let calculation_fn = |values: &[Amount]| {
            let discount_rate = &values[0];
            let growth_rate = &values[1];
            let base = Amount::from_i64(100_000_000);
            &(&base * discount_rate) * growth_rate / &Amount::from_i64(100)
        };

        let (pretax_income, total_assets, revenue, equity) = create_test_financials();
        let threshold = QuantitativeThreshold::new(&pretax_income, &total_assets, &revenue, &equity);

        let judgment = MaterialityService::judge_estimate_with_sensitivity(
            params,
            calculation_fn,
            &threshold,
            "感度分析".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        assert_eq!(judgment.sensitivity_results().len(), 2);
    }

    #[test]
    fn test_batch_judge_quantitative() {
        let (pretax_income, total_assets, revenue, equity) = create_test_financials();

        let amounts = vec![
            ("売掛金評価損".to_string(), Amount::from_i64(30_000_000)),
            ("棚卸資産評価損".to_string(), Amount::from_i64(20_000_000)),
            ("減損損失".to_string(), Amount::from_i64(50_000_000)),
        ];

        let results = MaterialityService::batch_judge_quantitative(
            &amounts,
            &pretax_income,
            &total_assets,
            &revenue,
            &equity,
            "担当者A".to_string(),
        )
        .unwrap();

        assert_eq!(results.len(), 3);

        // 売掛金評価損: 3,000万円 → 重要
        assert!(results[0].1.requires_adjustment());

        // 棚卸資産評価損: 2,000万円 → 非重要
        assert!(!results[1].1.requires_adjustment());

        // 減損損失: 5,000万円 → 重要
        assert!(results[2].1.requires_adjustment());
    }

    #[test]
    fn test_calculate_excess_ratio() {
        let amount = Amount::from_i64(50_000_000);
        let threshold = Amount::from_i64(25_000_000);

        let ratio = MaterialityService::calculate_excess_ratio(&amount, &threshold).unwrap();

        assert_eq!(ratio, 2.0);
    }

    #[test]
    fn test_calculate_excess_ratio_zero_threshold() {
        let amount = Amount::from_i64(50_000_000);
        let threshold = Amount::zero();

        assert!(MaterialityService::calculate_excess_ratio(&amount, &threshold).is_err());
    }

    #[test]
    fn test_determine_approval_route() {
        let (pretax_income, total_assets, revenue, equity) = create_test_financials();

        // Manager level
        let amount1 = Amount::from_i64(30_000_000);
        let judgment1 = MaterialityService::judge_quantitative(
            &amount1,
            &pretax_income,
            &total_assets,
            &revenue,
            &equity,
            "補正".to_string(),
            "担当者".to_string(),
        )
        .unwrap();
        let route1 = MaterialityService::determine_approval_route(&judgment1);
        assert_eq!(route1.len(), 1);
        assert_eq!(route1[0], "課長");

        // Director level
        let amount2 = Amount::from_i64(60_000_000);
        let judgment2 = MaterialityService::judge_quantitative(
            &amount2,
            &pretax_income,
            &total_assets,
            &revenue,
            &equity,
            "補正".to_string(),
            "担当者".to_string(),
        )
        .unwrap();
        let route2 = MaterialityService::determine_approval_route(&judgment2);
        assert_eq!(route2.len(), 2);

        // Board level
        let amount3 = Amount::from_i64(300_000_000);
        let judgment3 = MaterialityService::judge_quantitative(
            &amount3,
            &pretax_income,
            &total_assets,
            &revenue,
            &equity,
            "補正".to_string(),
            "担当者".to_string(),
        )
        .unwrap();
        let route3 = MaterialityService::determine_approval_route(&judgment3);
        assert_eq!(route3.len(), 4);
        assert_eq!(route3[3], "取締役会");
    }

    #[test]
    fn test_verify_judgment_consistency() {
        let (pretax_income, total_assets, revenue, equity) = create_test_financials();
        let amount = Amount::from_i64(30_000_000);

        let mut judgment = MaterialityService::judge_quantitative(
            &amount,
            &pretax_income,
            &total_assets,
            &revenue,
            &equity,
            "補正".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        // 承認前は整合性なし
        assert!(!MaterialityService::verify_judgment_consistency(&judgment).unwrap());

        // 承認後は整合性あり
        judgment.approve("課長B".to_string()).unwrap();
        assert!(MaterialityService::verify_judgment_consistency(&judgment).unwrap());
    }

    #[test]
    fn test_assess_qualitative_severity() {
        let factors1 = vec![QualitativeFactor::ManagementFraud];
        let severity1 = MaterialityService::assess_qualitative_severity(&factors1);
        assert_eq!(severity1, 100);

        let factors2 = vec![
            QualitativeFactor::AccountingPolicyChange,
            QualitativeFactor::RelatedPartyTransaction,
        ];
        let severity2 = MaterialityService::assess_qualitative_severity(&factors2);
        assert_eq!(severity2, 90); // 50 + 40

        let factors3 = vec![
            QualitativeFactor::LegalViolation,
            QualitativeFactor::GoingConcernUncertainty,
        ];
        let severity3 = MaterialityService::assess_qualitative_severity(&factors3);
        assert_eq!(severity3, 185); // 90 + 95
    }

    #[test]
    fn test_verify_estimate_judgment_consistency() {
        let params = vec![EstimateParameter::new("割引率".to_string(), Amount::from_i64(1000)).unwrap()];

        let judgment = MaterialityJudgment::new_estimate(
            params,
            "感度分析".to_string(),
            "担当者A".to_string(),
        )
        .unwrap();

        // 感度分析未実施は整合性なし
        assert!(!MaterialityService::verify_judgment_consistency(&judgment).unwrap());
    }
}
