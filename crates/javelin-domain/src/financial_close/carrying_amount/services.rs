// 帳簿価額のドメインサービス

use super::{entities::CarryingAmount, values::ComponentType};
use crate::{common::Amount, error::DomainResult};

/// 帳簿価額ドメインサービス
pub struct CarryingAmountService;

impl CarryingAmountService {
    /// 補助元帳、総勘定元帳、財務諸表表示額の整合性を検証
    pub fn verify_consistency(
        carrying_amount: &CarryingAmount,
        subsidiary_ledger_balance: &Amount,
        general_ledger_balance: &Amount,
        financial_statement_amount: &Amount,
    ) -> DomainResult<bool> {
        let calculated_amount = carrying_amount.calculate_carrying_amount();

        // 補助元帳との整合性
        if &calculated_amount != subsidiary_ledger_balance {
            return Ok(false);
        }

        // 総勘定元帳との整合性
        if &calculated_amount != general_ledger_balance {
            return Ok(false);
        }

        // 財務諸表表示額との整合性（表示調整がある場合は考慮）
        let expected_statement_amount =
            carrying_amount.presentation_amount().unwrap_or(&calculated_amount);

        if expected_statement_amount != financial_statement_amount {
            return Ok(false);
        }

        Ok(true)
    }

    /// 帳簿価額形成過程のブリッジを再構成
    pub fn reconstruct_bridge(carrying_amount: &CarryingAmount) -> Vec<(String, Amount, Amount)> {
        let mut bridge = Vec::new();
        let mut running_total = Amount::zero();

        for component in carrying_amount.components() {
            let contribution = component.contribution();
            running_total = &running_total + &contribution;

            bridge.push((
                component.component_type().as_str().to_string(),
                contribution,
                running_total.clone(),
            ));
        }

        bridge
    }

    /// 測定前残高から最終帳簿価額までのブリッジを生成
    pub fn generate_measurement_bridge(
        initial_balance: Amount,
        carrying_amount: &CarryingAmount,
    ) -> Vec<(String, Amount)> {
        let mut bridge = vec![("Initial Balance".to_string(), initial_balance)];

        for component in carrying_amount.components() {
            if component.component_type() != &ComponentType::AcquisitionCost {
                bridge.push((
                    component.component_type().as_str().to_string(),
                    component.contribution(),
                ));
            }
        }

        let final_amount = carrying_amount.calculate_carrying_amount();
        bridge.push(("Final Carrying Amount".to_string(), final_amount));

        bridge
    }

    /// 表示調整の妥当性を検証
    pub fn validate_presentation_adjustment(
        carrying_amount: &CarryingAmount,
        max_adjustment_percentage: f64,
    ) -> DomainResult<bool> {
        let calculated_amount = carrying_amount.calculate_carrying_amount();

        if let Some(presentation_amount) = carrying_amount.presentation_amount() {
            let difference = (presentation_amount - &calculated_amount).abs();
            let percentage = if !calculated_amount.is_zero() {
                if let (Some(diff_f64), Some(calc_f64)) =
                    (difference.to_f64(), calculated_amount.abs().to_f64())
                {
                    (diff_f64 / calc_f64) * 100.0
                } else {
                    0.0
                }
            } else {
                0.0
            };

            if percentage > max_adjustment_percentage {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// 測定変更と見積変更を区別
    pub fn classify_change(
        is_measurement_basis_change: bool,
        is_policy_change: bool,
    ) -> ChangeClassification {
        if is_measurement_basis_change && is_policy_change {
            ChangeClassification::MeasurementChangePolicyChange
        } else if is_measurement_basis_change {
            ChangeClassification::MeasurementChangeVoluntary
        } else {
            ChangeClassification::EstimateChange
        }
    }

    /// 帳簿価額の構成要素別分析
    pub fn analyze_components(carrying_amount: &CarryingAmount) -> ComponentAnalysis {
        let mut total_additive = Amount::zero();
        let mut total_subtractive = Amount::zero();

        for component in carrying_amount.components() {
            if component.component_type().is_additive() {
                total_additive = &total_additive + component.amount();
            } else {
                total_subtractive = &total_subtractive + component.amount();
            }
        }

        let net_amount = &total_additive - &total_subtractive;

        ComponentAnalysis {
            total_additive,
            total_subtractive,
            net_amount,
            component_count: carrying_amount.components().len(),
        }
    }
}

/// 変更分類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeClassification {
    /// 測定変更（会計方針変更）
    MeasurementChangePolicyChange,
    /// 測定変更（任意変更）
    MeasurementChangeVoluntary,
    /// 見積変更
    EstimateChange,
}

/// 構成要素分析結果
#[derive(Debug, Clone)]
pub struct ComponentAnalysis {
    pub total_additive: Amount,
    pub total_subtractive: Amount,
    pub net_amount: Amount,
    pub component_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::financial_close::carrying_amount::{
        entities::MeasurementComponent,
        values::{CarryingAmountId, MeasurementBasis},
    };

    fn create_test_carrying_amount() -> CarryingAmount {
        let id = CarryingAmountId::new();
        let mut carrying_amount =
            CarryingAmount::new(id, "ASSET001".to_string(), "1500".to_string()).unwrap();

        let acquisition = MeasurementComponent::new(
            ComponentType::AcquisitionCost,
            Amount::from_i64(1_000_000),
            MeasurementBasis::HistoricalCost,
            "Initial acquisition".to_string(),
        )
        .unwrap();
        carrying_amount.add_component(acquisition).unwrap();

        let depreciation = MeasurementComponent::new(
            ComponentType::AccumulatedDepreciation,
            Amount::from_i64(200_000),
            MeasurementBasis::HistoricalCost,
            "Accumulated depreciation".to_string(),
        )
        .unwrap();
        carrying_amount.add_component(depreciation).unwrap();

        carrying_amount
    }

    #[test]
    fn test_verify_consistency_all_match() {
        let carrying_amount = create_test_carrying_amount();
        let calculated = carrying_amount.calculate_carrying_amount(); // 800,000

        let is_consistent = CarryingAmountService::verify_consistency(
            &carrying_amount,
            &calculated,
            &calculated,
            &calculated,
        )
        .unwrap();

        assert!(is_consistent);
    }

    #[test]
    fn test_verify_consistency_mismatch() {
        let carrying_amount = create_test_carrying_amount();
        let calculated = carrying_amount.calculate_carrying_amount();
        let mismatch = &calculated + &Amount::from_i64(1000);

        let is_consistent = CarryingAmountService::verify_consistency(
            &carrying_amount,
            &calculated,
            &mismatch, // Mismatch
            &calculated,
        )
        .unwrap();

        assert!(!is_consistent);
    }

    #[test]
    fn test_reconstruct_bridge() {
        let carrying_amount = create_test_carrying_amount();
        let bridge = CarryingAmountService::reconstruct_bridge(&carrying_amount);

        assert_eq!(bridge.len(), 2);
        assert_eq!(bridge[0].0, "AcquisitionCost");
        assert_eq!(bridge[0].1.to_i64(), Some(1_000_000));
        assert_eq!(bridge[0].2.to_i64(), Some(1_000_000));
        assert_eq!(bridge[1].0, "AccumulatedDepreciation");
        assert_eq!(bridge[1].1.to_i64(), Some(-200_000));
        assert_eq!(bridge[1].2.to_i64(), Some(800_000));
    }

    #[test]
    fn test_generate_measurement_bridge() {
        let carrying_amount = create_test_carrying_amount();
        let bridge =
            CarryingAmountService::generate_measurement_bridge(Amount::zero(), &carrying_amount);

        assert_eq!(bridge.len(), 3); // Initial + Depreciation + Final
        assert_eq!(bridge[0].0, "Initial Balance");
        assert!(bridge[0].1.is_zero());
        assert_eq!(bridge[1].0, "AccumulatedDepreciation");
        assert_eq!(bridge[1].1.to_i64(), Some(-200_000));
        assert_eq!(bridge[2].0, "Final Carrying Amount");
        assert_eq!(bridge[2].1.to_i64(), Some(800_000));
    }

    #[test]
    fn test_validate_presentation_adjustment_within_limit() {
        let mut carrying_amount = create_test_carrying_amount();
        let calculated = carrying_amount.calculate_carrying_amount();

        // 1% adjustment
        let adjusted = &calculated + &(&calculated / &Amount::from_i64(100));
        carrying_amount
            .set_presentation_amount(adjusted, "Rounding".to_string())
            .unwrap();

        let is_valid =
            CarryingAmountService::validate_presentation_adjustment(&carrying_amount, 5.0).unwrap();

        assert!(is_valid);
    }

    #[test]
    fn test_validate_presentation_adjustment_exceeds_limit() {
        let mut carrying_amount = create_test_carrying_amount();
        let calculated = carrying_amount.calculate_carrying_amount();

        // 10% adjustment
        let adjusted = &calculated + &(&calculated / &Amount::from_i64(10));
        carrying_amount
            .set_presentation_amount(adjusted, "Large adjustment".to_string())
            .unwrap();

        let is_valid =
            CarryingAmountService::validate_presentation_adjustment(&carrying_amount, 5.0).unwrap();

        assert!(!is_valid);
    }

    #[test]
    fn test_classify_change() {
        assert_eq!(
            CarryingAmountService::classify_change(true, true),
            ChangeClassification::MeasurementChangePolicyChange
        );
        assert_eq!(
            CarryingAmountService::classify_change(true, false),
            ChangeClassification::MeasurementChangeVoluntary
        );
        assert_eq!(
            CarryingAmountService::classify_change(false, false),
            ChangeClassification::EstimateChange
        );
    }

    #[test]
    fn test_analyze_components() {
        let carrying_amount = create_test_carrying_amount();
        let analysis = CarryingAmountService::analyze_components(&carrying_amount);

        assert_eq!(analysis.total_additive.to_i64(), Some(1_000_000));
        assert_eq!(analysis.total_subtractive.to_i64(), Some(200_000));
        assert_eq!(analysis.net_amount.to_i64(), Some(800_000));
        assert_eq!(analysis.component_count, 2);
    }
}
