// 固定資産台帳のエンティティ

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use super::values::FixedAssetId as FixedAssetIdExport;
use super::values::{
    AcquisitionDate, AssetCategory, AssetStatus, ComponentId, DepreciationMethod, FixedAssetId,
    MeasurementModel, UsefulLife,
};
use crate::{
    entity::Entity,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 固定資産エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedAsset {
    /// 資産管理番号
    id: FixedAssetId,
    /// 資産区分
    category: AssetCategory,
    /// 資産名称
    name: String,
    /// 勘定科目コード
    account_code: String,
    /// 財務諸表表示区分
    balance_sheet_category: String,
    /// 資金生成単位（CGU）
    cash_generating_unit: Option<String>,
    /// 取得日
    acquisition_date: AcquisitionDate,
    /// 取得原価
    acquisition_cost: i64,
    /// 測定モデル
    measurement_model: MeasurementModel,
    /// 再評価額（再評価モデルの場合）
    revaluation_amount: Option<i64>,
    /// 再評価差額累計
    accumulated_revaluation_surplus: i64,
    /// 減損損失累計
    accumulated_impairment_loss: i64,
    /// 減損戻入累計
    accumulated_impairment_reversal: i64,
    /// 資産ステータス
    status: AssetStatus,
    /// コンポーネント
    components: Vec<Component>,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// 更新日時
    updated_at: DateTime<Utc>,
}

impl FixedAsset {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: FixedAssetId,
        category: AssetCategory,
        name: String,
        account_code: String,
        balance_sheet_category: String,
        acquisition_date: AcquisitionDate,
        acquisition_cost: i64,
        measurement_model: MeasurementModel,
    ) -> DomainResult<Self> {
        if name.is_empty() {
            return Err(DomainError::InvalidAssetName);
        }
        if account_code.is_empty() {
            return Err(DomainError::InvalidAccountCode);
        }
        if acquisition_cost < 0 {
            return Err(DomainError::InvalidAcquisitionCost);
        }

        acquisition_date.validate()?;

        let now = Utc::now();
        Ok(Self {
            id,
            category,
            name,
            account_code,
            balance_sheet_category,
            cash_generating_unit: None,
            acquisition_date,
            acquisition_cost,
            measurement_model,
            revaluation_amount: None,
            accumulated_revaluation_surplus: 0,
            accumulated_impairment_loss: 0,
            accumulated_impairment_reversal: 0,
            status: AssetStatus::InUse,
            components: Vec::new(),
            created_at: now,
            updated_at: now,
        })
    }

    /// 帳簿価額を計算
    pub fn carrying_amount(&self) -> i64 {
        let base_amount = match self.measurement_model {
            MeasurementModel::CostModel => self.acquisition_cost,
            MeasurementModel::RevaluationModel => {
                self.revaluation_amount.unwrap_or(self.acquisition_cost)
            }
        };

        let total_depreciation: i64 =
            self.components.iter().map(|c| c.accumulated_depreciation()).sum();

        base_amount - total_depreciation - self.accumulated_impairment_loss
            + self.accumulated_impairment_reversal
    }

    /// コンポーネントを追加
    pub fn add_component(&mut self, component: Component) -> DomainResult<()> {
        // 同じIDのコンポーネントが既に存在しないかチェック
        if self.components.iter().any(|c| c.id() == component.id()) {
            return Err(DomainError::DuplicateComponent);
        }

        self.components.push(component);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 再評価を実施
    pub fn revaluate(&mut self, new_amount: i64) -> DomainResult<()> {
        if !matches!(self.measurement_model, MeasurementModel::RevaluationModel) {
            return Err(DomainError::RevaluationNotAllowed);
        }

        if new_amount < 0 {
            return Err(DomainError::InvalidRevaluationAmount);
        }

        let old_amount = self.revaluation_amount.unwrap_or(self.acquisition_cost);
        let surplus = new_amount - old_amount;

        self.revaluation_amount = Some(new_amount);
        self.accumulated_revaluation_surplus += surplus;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 減損損失を計上
    pub fn recognize_impairment(&mut self, impairment_loss: i64) -> DomainResult<()> {
        if impairment_loss < 0 {
            return Err(DomainError::InvalidImpairmentLoss);
        }

        self.accumulated_impairment_loss += impairment_loss;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 減損戻入を計上
    pub fn reverse_impairment(&mut self, reversal_amount: i64) -> DomainResult<()> {
        if reversal_amount < 0 {
            return Err(DomainError::InvalidImpairmentReversal);
        }

        // 戻入額が過去の減損損失を超えないようにチェック
        if self.accumulated_impairment_reversal + reversal_amount > self.accumulated_impairment_loss
        {
            return Err(DomainError::ExcessiveImpairmentReversal);
        }

        self.accumulated_impairment_reversal += reversal_amount;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// ステータスを変更
    pub fn change_status(&mut self, new_status: AssetStatus) -> DomainResult<()> {
        // ステータス遷移のバリデーション
        if let (AssetStatus::Disposed, _) = (&self.status, &new_status) {
            return Err(DomainError::CannotChangeDisposedAssetStatus);
        }

        self.status = new_status;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// CGUを設定
    pub fn set_cash_generating_unit(&mut self, cgu: String) {
        self.cash_generating_unit = Some(cgu);
        self.updated_at = Utc::now();
    }

    // Getters
    pub fn id(&self) -> &FixedAssetId {
        &self.id
    }

    pub fn category(&self) -> &AssetCategory {
        &self.category
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn account_code(&self) -> &str {
        &self.account_code
    }

    pub fn balance_sheet_category(&self) -> &str {
        &self.balance_sheet_category
    }

    pub fn cash_generating_unit(&self) -> Option<&str> {
        self.cash_generating_unit.as_deref()
    }

    pub fn acquisition_date(&self) -> &AcquisitionDate {
        &self.acquisition_date
    }

    pub fn acquisition_cost(&self) -> i64 {
        self.acquisition_cost
    }

    pub fn measurement_model(&self) -> &MeasurementModel {
        &self.measurement_model
    }

    pub fn revaluation_amount(&self) -> Option<i64> {
        self.revaluation_amount
    }

    pub fn accumulated_revaluation_surplus(&self) -> i64 {
        self.accumulated_revaluation_surplus
    }

    pub fn accumulated_impairment_loss(&self) -> i64 {
        self.accumulated_impairment_loss
    }

    pub fn accumulated_impairment_reversal(&self) -> i64 {
        self.accumulated_impairment_reversal
    }

    pub fn status(&self) -> &AssetStatus {
        &self.status
    }

    pub fn components(&self) -> &[Component] {
        &self.components
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Entity for FixedAsset {
    type Id = FixedAssetId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

/// コンポーネント（構成要素）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// コンポーネントID
    id: ComponentId,
    /// コンポーネント名称
    name: String,
    /// 取得原価
    cost: i64,
    /// 耐用年数
    useful_life: UsefulLife,
    /// 残存価額
    residual_value: i64,
    /// 償却方法
    depreciation_method: DepreciationMethod,
    /// 当期償却額
    current_depreciation: i64,
    /// 累計償却額
    accumulated_depreciation: i64,
    /// 償却開始日
    depreciation_start_date: DateTime<Utc>,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// 更新日時
    updated_at: DateTime<Utc>,
}

impl Component {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ComponentId,
        name: String,
        cost: i64,
        useful_life: UsefulLife,
        residual_value: i64,
        depreciation_method: DepreciationMethod,
        depreciation_start_date: DateTime<Utc>,
    ) -> DomainResult<Self> {
        if name.is_empty() {
            return Err(DomainError::InvalidComponentName);
        }
        if cost < 0 {
            return Err(DomainError::InvalidComponentCost);
        }
        if residual_value < 0 || residual_value >= cost {
            return Err(DomainError::InvalidResidualValue);
        }

        let now = Utc::now();
        Ok(Self {
            id,
            name,
            cost,
            useful_life,
            residual_value,
            depreciation_method,
            current_depreciation: 0,
            accumulated_depreciation: 0,
            depreciation_start_date,
            created_at: now,
            updated_at: now,
        })
    }

    /// 償却額を計算（定額法）
    pub fn calculate_straight_line_depreciation(&self, months: u32) -> i64 {
        let depreciable_amount = self.cost - self.residual_value;
        let total_months = self.useful_life.total_months();
        if total_months == 0 {
            return 0;
        }
        (depreciable_amount * i64::from(months)) / i64::from(total_months)
    }

    /// 償却を実施
    pub fn depreciate(&mut self, amount: i64) -> DomainResult<()> {
        if amount < 0 {
            return Err(DomainError::InvalidDepreciationAmount);
        }

        let depreciable_amount = self.cost - self.residual_value;
        if self.accumulated_depreciation + amount > depreciable_amount {
            return Err(DomainError::ExcessiveDepreciation);
        }

        self.current_depreciation = amount;
        self.accumulated_depreciation += amount;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 帳簿価額を計算
    pub fn carrying_amount(&self) -> i64 {
        self.cost - self.accumulated_depreciation
    }

    // Getters
    pub fn id(&self) -> &ComponentId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cost(&self) -> i64 {
        self.cost
    }

    pub fn useful_life(&self) -> &UsefulLife {
        &self.useful_life
    }

    pub fn residual_value(&self) -> i64 {
        self.residual_value
    }

    pub fn depreciation_method(&self) -> &DepreciationMethod {
        &self.depreciation_method
    }

    pub fn current_depreciation(&self) -> i64 {
        self.current_depreciation
    }

    pub fn accumulated_depreciation(&self) -> i64 {
        self.accumulated_depreciation
    }

    pub fn depreciation_start_date(&self) -> DateTime<Utc> {
        self.depreciation_start_date
    }
}

impl Entity for Component {
    type Id = ComponentId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_asset() -> FixedAsset {
        let id = FixedAssetId::new();
        let acquisition_date = AcquisitionDate::new(Utc::now() - chrono::Duration::days(365));
        FixedAsset::new(
            id,
            AssetCategory::TangibleAsset,
            "Test Asset".to_string(),
            "1000".to_string(),
            "Property, Plant and Equipment".to_string(),
            acquisition_date,
            1_000_000,
            MeasurementModel::CostModel,
        )
        .unwrap()
    }

    fn create_test_component() -> Component {
        let id = ComponentId::new();
        let useful_life = UsefulLife::new(5, 0).unwrap();
        Component::new(
            id,
            "Main Component".to_string(),
            800_000,
            useful_life,
            50_000,
            DepreciationMethod::StraightLine,
            Utc::now(),
        )
        .unwrap()
    }

    #[test]
    fn test_fixed_asset_creation() {
        let asset = create_test_asset();
        assert_eq!(asset.name(), "Test Asset");
        assert_eq!(asset.acquisition_cost(), 1_000_000);
        assert_eq!(asset.carrying_amount(), 1_000_000);
    }

    #[test]
    fn test_fixed_asset_invalid_name() {
        let id = FixedAssetId::new();
        let acquisition_date = AcquisitionDate::new(Utc::now());
        let result = FixedAsset::new(
            id,
            AssetCategory::TangibleAsset,
            "".to_string(),
            "1000".to_string(),
            "PPE".to_string(),
            acquisition_date,
            1_000_000,
            MeasurementModel::CostModel,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_add_component() {
        let mut asset = create_test_asset();
        let component = create_test_component();

        assert!(asset.add_component(component).is_ok());
        assert_eq!(asset.components().len(), 1);
    }

    #[test]
    fn test_carrying_amount_with_depreciation() {
        let mut asset = create_test_asset();
        let mut component = create_test_component();

        // 償却を実施
        component.depreciate(150_000).unwrap();
        asset.add_component(component).unwrap();

        // 帳簿価額 = 取得原価 - 累計償却額
        assert_eq!(asset.carrying_amount(), 1_000_000 - 150_000);
    }

    #[test]
    fn test_revaluation() {
        let id = FixedAssetId::new();
        let acquisition_date = AcquisitionDate::new(Utc::now());
        let mut asset = FixedAsset::new(
            id,
            AssetCategory::TangibleAsset,
            "Test Asset".to_string(),
            "1000".to_string(),
            "PPE".to_string(),
            acquisition_date,
            1_000_000,
            MeasurementModel::RevaluationModel,
        )
        .unwrap();

        assert!(asset.revaluate(1_200_000).is_ok());
        assert_eq!(asset.revaluation_amount(), Some(1_200_000));
        assert_eq!(asset.accumulated_revaluation_surplus(), 200_000);
    }

    #[test]
    fn test_revaluation_not_allowed_for_cost_model() {
        let mut asset = create_test_asset();
        assert!(asset.revaluate(1_200_000).is_err());
    }

    #[test]
    fn test_impairment() {
        let mut asset = create_test_asset();
        assert!(asset.recognize_impairment(100_000).is_ok());
        assert_eq!(asset.accumulated_impairment_loss(), 100_000);
        assert_eq!(asset.carrying_amount(), 1_000_000 - 100_000);
    }

    #[test]
    fn test_impairment_reversal() {
        let mut asset = create_test_asset();
        asset.recognize_impairment(100_000).unwrap();
        assert!(asset.reverse_impairment(30_000).is_ok());
        assert_eq!(asset.accumulated_impairment_reversal(), 30_000);
        assert_eq!(asset.carrying_amount(), 1_000_000 - 100_000 + 30_000);
    }

    #[test]
    fn test_excessive_impairment_reversal() {
        let mut asset = create_test_asset();
        asset.recognize_impairment(100_000).unwrap();
        assert!(asset.reverse_impairment(150_000).is_err());
    }

    #[test]
    fn test_status_change() {
        let mut asset = create_test_asset();
        assert!(asset.change_status(AssetStatus::Idle).is_ok());
        assert_eq!(asset.status(), &AssetStatus::Idle);
    }

    #[test]
    fn test_cannot_change_disposed_asset_status() {
        let mut asset = create_test_asset();
        asset.change_status(AssetStatus::Disposed).unwrap();
        assert!(asset.change_status(AssetStatus::InUse).is_err());
    }

    #[test]
    fn test_component_creation() {
        let component = create_test_component();
        assert_eq!(component.name(), "Main Component");
        assert_eq!(component.cost(), 800_000);
        assert_eq!(component.carrying_amount(), 800_000);
    }

    #[test]
    fn test_component_straight_line_depreciation() {
        let component = create_test_component();
        // 5年（60ヶ月）で750,000円を償却
        // 12ヶ月分 = 750,000 * 12 / 60 = 150,000
        let depreciation = component.calculate_straight_line_depreciation(12);
        assert_eq!(depreciation, 150_000);
    }

    #[test]
    fn test_component_depreciate() {
        let mut component = create_test_component();
        assert!(component.depreciate(150_000).is_ok());
        assert_eq!(component.current_depreciation(), 150_000);
        assert_eq!(component.accumulated_depreciation(), 150_000);
        assert_eq!(component.carrying_amount(), 800_000 - 150_000);
    }

    #[test]
    fn test_component_excessive_depreciation() {
        let mut component = create_test_component();
        // 償却可能額 = 800,000 - 50,000 = 750,000
        assert!(component.depreciate(800_000).is_err());
    }

    #[test]
    fn test_component_invalid_residual_value() {
        let id = ComponentId::new();
        let useful_life = UsefulLife::new(5, 0).unwrap();
        // 残存価額が取得原価以上
        let result = Component::new(
            id,
            "Test".to_string(),
            800_000,
            useful_life,
            800_000,
            DepreciationMethod::StraightLine,
            Utc::now(),
        );
        assert!(result.is_err());
    }
}
