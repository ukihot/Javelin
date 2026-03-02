// 固定資産台帳のドメインサービス

use chrono::{DateTime, Utc};

use super::{
    entities::{Component, FixedAsset},
    values::{AssetCategory, DepreciationMethod},
};
use crate::{
    common::Amount,
    error::{DomainError, DomainResult},
};

/// 固定資産ドメインサービス
pub struct FixedAssetDomainService;

impl FixedAssetDomainService {
    /// 総勘定元帳との整合性を検証
    pub fn verify_ledger_consistency(
        asset: &FixedAsset,
        ledger_balance: &Amount,
    ) -> DomainResult<()> {
        let carrying_amount = asset.carrying_amount();
        if &carrying_amount != ledger_balance {
            return Err(DomainError::LedgerInconsistency {
                asset_carrying_amount: carrying_amount.to_i64().unwrap_or(0),
                ledger_balance: ledger_balance.to_i64().unwrap_or(0),
            });
        }
        Ok(())
    }

    /// 月次償却額を計算
    pub fn calculate_monthly_depreciation(
        component: &Component,
        target_month: DateTime<Utc>,
    ) -> DomainResult<Amount> {
        // 償却開始日より前の場合は0
        if target_month < component.depreciation_start_date() {
            return Ok(Amount::zero());
        }

        // 償却方法に応じて計算
        match component.depreciation_method() {
            DepreciationMethod::StraightLine => {
                Ok(component.calculate_straight_line_depreciation(1))
            }
            DepreciationMethod::DecliningBalance => {
                // 定率法の実装（簡易版）
                let remaining_value = component.carrying_amount();
                let rate = Self::calculate_declining_balance_rate(component.useful_life().years());
                if let Some(rv_f64) = remaining_value.to_f64() {
                    Ok(Amount::from_i64((rv_f64 * rate) as i64))
                } else {
                    Ok(Amount::zero())
                }
            }
            DepreciationMethod::UnitsOfProduction => {
                // 生産高比例法は別途生産量データが必要
                Err(DomainError::UnsupportedDepreciationMethod)
            }
        }
    }

    /// 定率法の償却率を計算
    fn calculate_declining_balance_rate(useful_life_years: u32) -> f64 {
        if useful_life_years == 0 {
            return 0.0;
        }
        // 簡易的な定率法償却率: 1 / 耐用年数 * 2
        2.0 / useful_life_years as f64
    }

    /// 年度末の償却額を計算
    pub fn calculate_annual_depreciation(
        component: &Component,
        _fiscal_year: u32,
    ) -> DomainResult<Amount> {
        match component.depreciation_method() {
            DepreciationMethod::StraightLine => {
                Ok(component.calculate_straight_line_depreciation(12))
            }
            DepreciationMethod::DecliningBalance => {
                let remaining_value = component.carrying_amount();
                let rate = Self::calculate_declining_balance_rate(component.useful_life().years());
                if let Some(rv_f64) = remaining_value.to_f64() {
                    Ok(Amount::from_i64((rv_f64 * rate) as i64))
                } else {
                    Ok(Amount::zero())
                }
            }
            DepreciationMethod::UnitsOfProduction => {
                Err(DomainError::UnsupportedDepreciationMethod)
            }
        }
    }

    /// 減損兆候をチェック
    pub fn check_impairment_indicators(
        asset: &FixedAsset,
        market_value: Option<&Amount>,
        performance_decline: bool,
    ) -> bool {
        // 市場価格が著しく低下している
        if let Some(mv) = market_value {
            let carrying_amount = asset.carrying_amount();
            if let (Some(mv_i64), Some(ca_i64)) = (mv.to_i64(), carrying_amount.to_i64())
                && mv_i64 < (ca_i64 * 70) / 100
            {
                // 30%以上の低下
                return true;
            }
        }

        // 業績が著しく悪化している
        if performance_decline {
            return true;
        }

        // 遊休状態
        if matches!(asset.status(), super::values::AssetStatus::Idle) {
            return true;
        }

        false
    }

    /// 回収可能価額を計算（使用価値法）
    pub fn calculate_recoverable_amount(
        future_cash_flows: &[Amount],
        discount_rate: f64,
    ) -> DomainResult<Amount> {
        if !(0.0..=1.0).contains(&discount_rate) {
            return Err(DomainError::InvalidDiscountRate);
        }

        let mut present_value = 0.0;
        for (year, cash_flow) in future_cash_flows.iter().enumerate() {
            if let Some(cf_f64) = cash_flow.to_f64() {
                let discount_factor = 1.0 / (1.0 + discount_rate).powi((year + 1) as i32);
                present_value += cf_f64 * discount_factor;
            }
        }

        Ok(Amount::from_i64(present_value as i64))
    }

    /// 減損損失を計算
    pub fn calculate_impairment_loss(
        carrying_amount: &Amount,
        recoverable_amount: &Amount,
    ) -> DomainResult<Amount> {
        if recoverable_amount.is_negative() {
            return Err(DomainError::InvalidRecoverableAmount);
        }

        if carrying_amount <= recoverable_amount {
            return Ok(Amount::zero());
        }

        Ok(carrying_amount - recoverable_amount)
    }

    /// 建設仮勘定から本勘定への振替可否を判定
    pub fn can_transfer_from_cip(asset: &FixedAsset) -> bool {
        matches!(asset.category(), AssetCategory::ConstructionInProgress)
    }

    /// コンポーネント単位での償却可否を判定
    pub fn can_depreciate_component(
        component: &Component,
        asset_status: &super::values::AssetStatus,
    ) -> bool {
        // 資産が使用中であること
        if !asset_status.can_depreciate() {
            return false;
        }

        // 償却可能額が残っていること
        let depreciable_amount = component.cost() - component.residual_value();
        component.accumulated_depreciation() < &depreciable_amount
    }

    /// 複数コンポーネントの合計帳簿価額を計算
    pub fn calculate_total_carrying_amount(components: &[Component]) -> Amount {
        components.iter().fold(Amount::zero(), |acc, c| acc + c.carrying_amount())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            entities::Component,
            values::{ComponentId, UsefulLife},
        },
        *,
    };

    fn create_test_component() -> Component {
        let id = ComponentId::new();
        let useful_life = UsefulLife::new(5, 0).unwrap();
        Component::new(
            id,
            "Test Component".to_string(),
            Amount::from_i64(1_000_000),
            useful_life,
            Amount::from_i64(100_000),
            DepreciationMethod::StraightLine,
            Utc::now(),
        )
        .unwrap()
    }

    #[test]
    fn test_calculate_monthly_depreciation_straight_line() {
        let component = create_test_component();
        let target_month = Utc::now();

        let depreciation =
            FixedAssetDomainService::calculate_monthly_depreciation(&component, target_month)
                .unwrap();

        // (1,000,000 - 100,000) / 60ヶ月 = 15,000
        assert_eq!(depreciation.to_i64(), Some(15_000));
    }

    #[test]
    fn test_calculate_annual_depreciation_straight_line() {
        let component = create_test_component();

        let depreciation =
            FixedAssetDomainService::calculate_annual_depreciation(&component, 2024).unwrap();

        // (1,000,000 - 100,000) / 5年 = 180,000
        assert_eq!(depreciation.to_i64(), Some(180_000));
    }

    #[test]
    fn test_calculate_declining_balance_rate() {
        let rate = FixedAssetDomainService::calculate_declining_balance_rate(5);
        assert_eq!(rate, 0.4); // 2 / 5 = 0.4
    }

    #[test]
    fn test_check_impairment_indicators_market_value_decline() {
        let id = super::super::values::FixedAssetId::new();
        let acquisition_date = super::super::values::AcquisitionDate::new(Utc::now());
        let asset = FixedAsset::new(
            id,
            AssetCategory::TangibleAsset,
            "Test Asset".to_string(),
            "1000".to_string(),
            "PPE".to_string(),
            acquisition_date,
            Amount::from_i64(1_000_000),
            super::super::values::MeasurementModel::CostModel,
        )
        .unwrap();

        // 市場価格が30%以上低下
        let has_indicator = FixedAssetDomainService::check_impairment_indicators(
            &asset,
            Some(&Amount::from_i64(600_000)),
            false,
        );
        assert!(has_indicator);

        // 市場価格が30%未満の低下
        let has_indicator = FixedAssetDomainService::check_impairment_indicators(
            &asset,
            Some(&Amount::from_i64(800_000)),
            false,
        );
        assert!(!has_indicator);
    }

    #[test]
    fn test_check_impairment_indicators_performance_decline() {
        let id = super::super::values::FixedAssetId::new();
        let acquisition_date = super::super::values::AcquisitionDate::new(Utc::now());
        let asset = FixedAsset::new(
            id,
            AssetCategory::TangibleAsset,
            "Test Asset".to_string(),
            "1000".to_string(),
            "PPE".to_string(),
            acquisition_date,
            Amount::from_i64(1_000_000),
            super::super::values::MeasurementModel::CostModel,
        )
        .unwrap();

        let has_indicator =
            FixedAssetDomainService::check_impairment_indicators(&asset, None, true);
        assert!(has_indicator);
    }

    #[test]
    fn test_calculate_recoverable_amount() {
        let future_cash_flows = vec![
            Amount::from_i64(200_000),
            Amount::from_i64(200_000),
            Amount::from_i64(200_000),
            Amount::from_i64(200_000),
            Amount::from_i64(200_000),
        ];
        let discount_rate = 0.05;

        let recoverable_amount = FixedAssetDomainService::calculate_recoverable_amount(
            &future_cash_flows,
            discount_rate,
        )
        .unwrap();

        // 現在価値の合計（概算）
        let ra_i64 = recoverable_amount.to_i64().unwrap();
        assert!(ra_i64 > 800_000);
        assert!(ra_i64 < 1_000_000);
    }

    #[test]
    fn test_calculate_impairment_loss() {
        let carrying_amount = Amount::from_i64(1_000_000);
        let recoverable_amount = Amount::from_i64(800_000);

        let impairment_loss = FixedAssetDomainService::calculate_impairment_loss(
            &carrying_amount,
            &recoverable_amount,
        )
        .unwrap();

        assert_eq!(impairment_loss.to_i64(), Some(200_000));
    }

    #[test]
    fn test_calculate_impairment_loss_no_impairment() {
        let carrying_amount = Amount::from_i64(1_000_000);
        let recoverable_amount = Amount::from_i64(1_200_000);

        let impairment_loss = FixedAssetDomainService::calculate_impairment_loss(
            &carrying_amount,
            &recoverable_amount,
        )
        .unwrap();

        assert!(impairment_loss.is_zero());
    }

    #[test]
    fn test_can_transfer_from_cip() {
        let id = super::super::values::FixedAssetId::new();
        let acquisition_date = super::super::values::AcquisitionDate::new(Utc::now());
        let asset = FixedAsset::new(
            id,
            AssetCategory::ConstructionInProgress,
            "CIP Asset".to_string(),
            "1500".to_string(),
            "CIP".to_string(),
            acquisition_date,
            Amount::from_i64(1_000_000),
            super::super::values::MeasurementModel::CostModel,
        )
        .unwrap();

        assert!(FixedAssetDomainService::can_transfer_from_cip(&asset));
    }

    #[test]
    fn test_can_depreciate_component() {
        let component = create_test_component();
        let status = super::super::values::AssetStatus::InUse;

        assert!(FixedAssetDomainService::can_depreciate_component(&component, &status));

        let status = super::super::values::AssetStatus::Idle;
        assert!(!FixedAssetDomainService::can_depreciate_component(&component, &status));
    }

    #[test]
    fn test_calculate_total_carrying_amount() {
        let component1 = create_test_component();
        let component2 = create_test_component();

        let total =
            FixedAssetDomainService::calculate_total_carrying_amount(&[component1, component2]);
        assert_eq!(total.to_i64(), Some(2_000_000));
    }
}
