// 帳簿価額のエンティティ

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{
    CarryingAmountId, ComponentType, EstimateChange, MeasurementBasis, MeasurementChange,
};
use crate::{
    common::Amount,
    entity::Entity,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 測定構成要素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementComponent {
    /// 構成要素タイプ
    component_type: ComponentType,
    /// 金額
    amount: Amount,
    /// 測定基礎
    measurement_basis: MeasurementBasis,
    /// 測定日
    measurement_date: DateTime<Utc>,
    /// 測定根拠
    measurement_rationale: String,
}

impl MeasurementComponent {
    pub fn new(
        component_type: ComponentType,
        amount: Amount,
        measurement_basis: MeasurementBasis,
        measurement_rationale: String,
    ) -> DomainResult<Self> {
        if measurement_rationale.is_empty() {
            return Err(DomainError::InvalidMeasurementComponent);
        }

        Ok(Self {
            component_type,
            amount,
            measurement_basis,
            measurement_date: Utc::now(),
            measurement_rationale,
        })
    }

    pub fn component_type(&self) -> &ComponentType {
        &self.component_type
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn measurement_basis(&self) -> &MeasurementBasis {
        &self.measurement_basis
    }

    pub fn measurement_date(&self) -> DateTime<Utc> {
        self.measurement_date
    }

    pub fn measurement_rationale(&self) -> &str {
        &self.measurement_rationale
    }

    /// 帳簿価額への寄与額を計算（符号を考慮）
    pub fn contribution(&self) -> Amount {
        if self.component_type.is_additive() {
            self.amount.clone()
        } else {
            -&self.amount
        }
    }
}

/// 帳簿価額エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarryingAmount {
    /// 帳簿価額ID
    id: CarryingAmountId,
    /// 資産・負債の識別子
    asset_liability_id: String,
    /// 勘定科目コード
    account_code: String,
    /// 測定構成要素
    components: Vec<MeasurementComponent>,
    /// 測定変更履歴
    measurement_changes: Vec<MeasurementChange>,
    /// 見積変更履歴
    estimate_changes: Vec<EstimateChange>,
    /// 表示額（財務諸表表示用）
    presentation_amount: Option<Amount>,
    /// 表示調整理由
    presentation_adjustment_reason: Option<String>,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// 更新日時
    updated_at: DateTime<Utc>,
}

impl CarryingAmount {
    pub fn new(
        id: CarryingAmountId,
        asset_liability_id: String,
        account_code: String,
    ) -> DomainResult<Self> {
        if asset_liability_id.is_empty() || account_code.is_empty() {
            return Err(DomainError::InvalidCarryingAmount);
        }

        let now = Utc::now();
        Ok(Self {
            id,
            asset_liability_id,
            account_code,
            components: Vec::new(),
            measurement_changes: Vec::new(),
            estimate_changes: Vec::new(),
            presentation_amount: None,
            presentation_adjustment_reason: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// 測定構成要素を追加
    pub fn add_component(&mut self, component: MeasurementComponent) -> DomainResult<()> {
        self.components.push(component);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 帳簿価額を計算
    pub fn calculate_carrying_amount(&self) -> Amount {
        self.components.iter().fold(Amount::zero(), |acc, c| acc + c.contribution())
    }

    /// 測定変更を記録
    pub fn record_measurement_change(&mut self, change: MeasurementChange) -> DomainResult<()> {
        change.validate()?;
        self.measurement_changes.push(change);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 見積変更を記録
    pub fn record_estimate_change(&mut self, change: EstimateChange) -> DomainResult<()> {
        change.validate()?;
        self.estimate_changes.push(change);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 表示額を設定
    pub fn set_presentation_amount(&mut self, amount: Amount, reason: String) -> DomainResult<()> {
        if reason.is_empty() {
            return Err(DomainError::InvalidPresentationAmount);
        }

        self.presentation_amount = Some(amount);
        self.presentation_adjustment_reason = Some(reason);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 表示額と測定額の差異を計算
    pub fn calculate_presentation_difference(&self) -> Amount {
        let carrying_amount = self.calculate_carrying_amount();
        self.presentation_amount
            .as_ref()
            .map(|pa| pa - &carrying_amount)
            .unwrap_or_else(Amount::zero)
    }

    /// 帳簿価額形成過程を取得
    pub fn get_formation_process(&self) -> Vec<(ComponentType, Amount, MeasurementBasis)> {
        self.components
            .iter()
            .map(|c| {
                (c.component_type().clone(), c.amount().clone(), c.measurement_basis().clone())
            })
            .collect()
    }

    /// 測定変更履歴を取得
    pub fn measurement_change_history(&self) -> &[MeasurementChange] {
        &self.measurement_changes
    }

    /// 見積変更履歴を取得
    pub fn estimate_change_history(&self) -> &[EstimateChange] {
        &self.estimate_changes
    }

    // Getters
    pub fn id(&self) -> &CarryingAmountId {
        &self.id
    }

    pub fn asset_liability_id(&self) -> &str {
        &self.asset_liability_id
    }

    pub fn account_code(&self) -> &str {
        &self.account_code
    }

    pub fn components(&self) -> &[MeasurementComponent] {
        &self.components
    }

    pub fn presentation_amount(&self) -> Option<&Amount> {
        self.presentation_amount.as_ref()
    }

    pub fn presentation_adjustment_reason(&self) -> Option<&str> {
        self.presentation_adjustment_reason.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Entity for CarryingAmount {
    type Id = CarryingAmountId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_carrying_amount() -> CarryingAmount {
        let id = CarryingAmountId::new();
        CarryingAmount::new(id, "ASSET001".to_string(), "1500".to_string()).unwrap()
    }

    #[test]
    fn test_carrying_amount_creation() {
        let carrying_amount = create_test_carrying_amount();
        assert_eq!(carrying_amount.asset_liability_id(), "ASSET001");
        assert_eq!(carrying_amount.account_code(), "1500");
        assert!(carrying_amount.calculate_carrying_amount().is_zero());
    }

    #[test]
    fn test_carrying_amount_invalid_id() {
        let id = CarryingAmountId::new();
        let result = CarryingAmount::new(id, "".to_string(), "1500".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_add_component() {
        let mut carrying_amount = create_test_carrying_amount();

        let component = MeasurementComponent::new(
            ComponentType::AcquisitionCost,
            Amount::from_i64(1_000_000),
            MeasurementBasis::HistoricalCost,
            "Initial acquisition".to_string(),
        )
        .unwrap();

        assert!(carrying_amount.add_component(component).is_ok());
        assert_eq!(carrying_amount.components().len(), 1);
        assert_eq!(carrying_amount.calculate_carrying_amount().to_i64(), Some(1_000_000));
    }

    #[test]
    fn test_calculate_carrying_amount_with_multiple_components() {
        let mut carrying_amount = create_test_carrying_amount();

        // 取得原価
        let acquisition = MeasurementComponent::new(
            ComponentType::AcquisitionCost,
            Amount::from_i64(1_000_000),
            MeasurementBasis::HistoricalCost,
            "Initial acquisition".to_string(),
        )
        .unwrap();
        carrying_amount.add_component(acquisition).unwrap();

        // 償却累計額
        let depreciation = MeasurementComponent::new(
            ComponentType::AccumulatedDepreciation,
            Amount::from_i64(200_000),
            MeasurementBasis::HistoricalCost,
            "Accumulated depreciation".to_string(),
        )
        .unwrap();
        carrying_amount.add_component(depreciation).unwrap();

        // 減損損失
        let impairment = MeasurementComponent::new(
            ComponentType::AccumulatedImpairmentLoss,
            Amount::from_i64(50_000),
            MeasurementBasis::ValueInUse,
            "Impairment loss".to_string(),
        )
        .unwrap();
        carrying_amount.add_component(impairment).unwrap();

        // 帳簿価額 = 1,000,000 - 200,000 - 50,000 = 750,000
        assert_eq!(carrying_amount.calculate_carrying_amount().to_i64(), Some(750_000));
    }

    #[test]
    fn test_record_measurement_change() {
        let mut carrying_amount = create_test_carrying_amount();

        let change = MeasurementChange::new(
            MeasurementBasis::HistoricalCost,
            MeasurementBasis::FairValue,
            "Change to fair value model".to_string(),
            true,
            false,
        )
        .unwrap();

        assert!(carrying_amount.record_measurement_change(change).is_ok());
        assert_eq!(carrying_amount.measurement_change_history().len(), 1);
    }

    #[test]
    fn test_record_estimate_change() {
        let mut carrying_amount = create_test_carrying_amount();

        let change = EstimateChange::new(
            "Useful life".to_string(),
            Amount::from_i64(10),
            Amount::from_i64(8),
            "Revised based on actual usage".to_string(),
        )
        .unwrap();

        assert!(carrying_amount.record_estimate_change(change).is_ok());
        assert_eq!(carrying_amount.estimate_change_history().len(), 1);
    }

    #[test]
    fn test_set_presentation_amount() {
        let mut carrying_amount = create_test_carrying_amount();

        let acquisition = MeasurementComponent::new(
            ComponentType::AcquisitionCost,
            Amount::from_i64(1_000_000),
            MeasurementBasis::HistoricalCost,
            "Initial acquisition".to_string(),
        )
        .unwrap();
        carrying_amount.add_component(acquisition).unwrap();

        assert!(
            carrying_amount
                .set_presentation_amount(
                    Amount::from_i64(950_000),
                    "Rounding adjustment".to_string()
                )
                .is_ok()
        );

        assert_eq!(carrying_amount.presentation_amount().unwrap().to_i64(), Some(950_000));
        assert_eq!(carrying_amount.calculate_presentation_difference().to_i64(), Some(-50_000));
    }

    #[test]
    fn test_get_formation_process() {
        let mut carrying_amount = create_test_carrying_amount();

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

        let process = carrying_amount.get_formation_process();
        assert_eq!(process.len(), 2);
        assert_eq!(process[0].0, ComponentType::AcquisitionCost);
        assert_eq!(process[0].1.to_i64(), Some(1_000_000));
        assert_eq!(process[1].0, ComponentType::AccumulatedDepreciation);
        assert_eq!(process[1].1.to_i64(), Some(200_000));
    }

    #[test]
    fn test_component_contribution() {
        let additive = MeasurementComponent::new(
            ComponentType::AcquisitionCost,
            Amount::from_i64(1_000_000),
            MeasurementBasis::HistoricalCost,
            "Initial acquisition".to_string(),
        )
        .unwrap();
        assert_eq!(additive.contribution().to_i64(), Some(1_000_000));

        let subtractive = MeasurementComponent::new(
            ComponentType::AccumulatedDepreciation,
            Amount::from_i64(200_000),
            MeasurementBasis::HistoricalCost,
            "Accumulated depreciation".to_string(),
        )
        .unwrap();
        assert_eq!(subtractive.contribution().to_i64(), Some(-200_000));
    }
}
