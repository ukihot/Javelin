// 評価関連の値オブジェクト（IFRS 9, IAS 36, IAS 37, IAS 2）

use crate::{error::DomainResult, value_object::ValueObject};

/// ECL（期待信用損失）計算モデル
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EclCalculationModel {
    /// 簡易法：全期間ECL
    Simplified,
    /// 一般法：段階的評価
    General,
}

impl ValueObject for EclCalculationModel {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

/// 債権年齢分類
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReceivableAge {
    /// 期日内
    Current,
    /// 30日以上60日未満
    Days30to60,
    /// 60日以上90日未満
    Days60to90,
    /// 90日以上180日未満
    Days90to180,
    /// 180日以上
    Over180Days,
}

impl ReceivableAge {
    /// 貸倒確率のデフォルト値を取得（%）
    pub fn default_loss_rate(&self) -> f64 {
        match self {
            ReceivableAge::Current => 0.5,
            ReceivableAge::Days30to60 => 2.0,
            ReceivableAge::Days60to90 => 5.0,
            ReceivableAge::Days90to180 => 15.0,
            ReceivableAge::Over180Days => 50.0,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            ReceivableAge::Current => "期日内",
            ReceivableAge::Days30to60 => "30～60日超過",
            ReceivableAge::Days60to90 => "60～90日超過",
            ReceivableAge::Days90to180 => "90～180日超過",
            ReceivableAge::Over180Days => "180日以上超過",
        }
    }
}

impl ValueObject for ReceivableAge {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

/// 減損兆候
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImpairmentIndicator {
    /// 市場価格の著しい下落
    SignificantMarketPriceDeclination,
    /// 回収困難の兆候
    CollectionDifficultIndicator,
    /// 契約条件の不利な変更
    UnfavorableContractModification,
    /// 技術的陳腐化
    TechnicalObsolescence,
    /// 市場シェアの喪失
    MarketShareLoss,
}

impl ImpairmentIndicator {
    pub fn display_name(&self) -> &str {
        match self {
            ImpairmentIndicator::SignificantMarketPriceDeclination => "市場価格の著しい下落",
            ImpairmentIndicator::CollectionDifficultIndicator => "回収困難の兆候",
            ImpairmentIndicator::UnfavorableContractModification => "契約条件の不利な変更",
            ImpairmentIndicator::TechnicalObsolescence => "技術的陳腐化",
            ImpairmentIndicator::MarketShareLoss => "市場シェアの喪失",
        }
    }
}

impl ValueObject for ImpairmentIndicator {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

/// 減損計算方法
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImpairmentCalculationMethod {
    /// 使用価値法（キャッシュ・フロー割引）
    UseValueMethod,
    /// 公正価値マイナス処分費用
    FairValueLessDisposalCosts,
}

impl ValueObject for ImpairmentCalculationMethod {
    fn validate(&self) -> DomainResult<()> {
        // 減損計算方法は常に有効です
        Ok(())
    }
}

/// 棚卸資産評価方法
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InventoryValuationMethod {
    /// 先入先出法（FIFO）
    Fifo,
    /// 後入先出法（LIFO）
    Lifo,
    /// 平均原価法
    WeightedAverageCost,
    /// 標準原価法
    StandardCost,
}

impl InventoryValuationMethod {
    pub fn display_name(&self) -> &str {
        match self {
            InventoryValuationMethod::Fifo => "先入先出法",
            InventoryValuationMethod::Lifo => "後入先出法",
            InventoryValuationMethod::WeightedAverageCost => "平均原価法",
            InventoryValuationMethod::StandardCost => "標準原価法",
        }
    }
}

impl ValueObject for InventoryValuationMethod {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

/// 引当金分類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvisionType {
    /// 製品保証引当金
    ProductWarranty,
    /// 訴訟引当金
    LitigationRisk,
    /// 契約履行義務引当金
    ContractualObligation,
    /// 環境整備引当金
    EnvironmentalRestoration,
    /// リストラクチャリング引当金
    Restructuring,
}

impl ProvisionType {
    pub fn display_name(&self) -> &str {
        match self {
            ProvisionType::ProductWarranty => "製品保証",
            ProvisionType::LitigationRisk => "訴訟",
            ProvisionType::ContractualObligation => "契約履行義務",
            ProvisionType::EnvironmentalRestoration => "環境整備",
            ProvisionType::Restructuring => "リストラクチャリング",
        }
    }
}

impl ValueObject for ProvisionType {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================
    // ReceivableAge テスト
    // ============================================

    #[test]
    fn test_receivable_age_loss_rates() {
        assert_eq!(ReceivableAge::Current.default_loss_rate(), 0.5);
        assert_eq!(ReceivableAge::Days30to60.default_loss_rate(), 2.0);
        assert_eq!(ReceivableAge::Days60to90.default_loss_rate(), 5.0);
        assert_eq!(ReceivableAge::Days90to180.default_loss_rate(), 15.0);
        assert_eq!(ReceivableAge::Over180Days.default_loss_rate(), 50.0);
    }

    #[test]
    fn test_receivable_age_ordering() {
        // ReceivableAge は順序付け可能（Ord実装）
        assert!(ReceivableAge::Current < ReceivableAge::Days30to60);
        assert!(ReceivableAge::Days30to60 < ReceivableAge::Days60to90);
        assert!(ReceivableAge::Days60to90 < ReceivableAge::Days90to180);
        assert!(ReceivableAge::Days90to180 < ReceivableAge::Over180Days);
    }

    #[test]
    fn test_receivable_age_extreme_loss_rates() {
        // 最小損失率（Current）
        assert_eq!(ReceivableAge::Current.default_loss_rate(), 0.5);
        // 最大損失率（Over180Days）
        assert_eq!(ReceivableAge::Over180Days.default_loss_rate(), 50.0);
        // レートが単調増加
        let rates: Vec<f64> = vec![
            ReceivableAge::Current.default_loss_rate(),
            ReceivableAge::Days30to60.default_loss_rate(),
            ReceivableAge::Days60to90.default_loss_rate(),
            ReceivableAge::Days90to180.default_loss_rate(),
            ReceivableAge::Over180Days.default_loss_rate(),
        ];
        for i in 0..rates.len() - 1 {
            assert!(rates[i] < rates[i + 1], "Loss rates should be monotonically increasing");
        }
    }

    // ============================================
    // EclCalculationModel テスト
    // ============================================

    #[test]
    fn test_ecl_model_validation() {
        assert!(EclCalculationModel::Simplified.validate().is_ok());
        assert!(EclCalculationModel::General.validate().is_ok());
    }

    #[test]
    fn test_ecl_model_as_value_object() {
        let model1 = EclCalculationModel::Simplified;
        let model2 = EclCalculationModel::Simplified;
        assert_eq!(model1, model2);

        let model3 = EclCalculationModel::General;
        assert_ne!(model1, model3);
    }

    // ============================================
    // ImpairmentIndicator テスト
    // ============================================

    #[test]
    fn test_impairment_indicator_display_names() {
        assert!(!ImpairmentIndicator::SignificantMarketPriceDeclination.display_name().is_empty());
        assert!(!ImpairmentIndicator::CollectionDifficultIndicator.display_name().is_empty());
        assert!(!ImpairmentIndicator::UnfavorableContractModification.display_name().is_empty());
        assert!(!ImpairmentIndicator::TechnicalObsolescence.display_name().is_empty());
        assert!(!ImpairmentIndicator::MarketShareLoss.display_name().is_empty());
    }

    #[test]
    fn test_impairment_indicator_enum() {
        // 各バリアントが区別可能であることを確認
        let indicators = [
            ImpairmentIndicator::SignificantMarketPriceDeclination,
            ImpairmentIndicator::CollectionDifficultIndicator,
            ImpairmentIndicator::UnfavorableContractModification,
            ImpairmentIndicator::TechnicalObsolescence,
            ImpairmentIndicator::MarketShareLoss,
        ];
        assert_eq!(indicators.len(), 5);

        // バリアント間が等しくないことを確認
        assert_ne!(
            ImpairmentIndicator::SignificantMarketPriceDeclination,
            ImpairmentIndicator::CollectionDifficultIndicator
        );
    }

    // ============================================
    // ImpairmentCalculationMethod テスト
    // ============================================

    #[test]
    fn test_impairment_calculation_method_validation() {
        let method_1 = ImpairmentCalculationMethod::UseValueMethod;
        assert!(method_1.validate().is_ok());

        let method_2 = ImpairmentCalculationMethod::FairValueLessDisposalCosts;
        assert!(method_2.validate().is_ok());
    }

    #[test]
    fn test_impairment_calculation_method_equality() {
        let method_1 = ImpairmentCalculationMethod::UseValueMethod;
        let method_2 = ImpairmentCalculationMethod::UseValueMethod;
        assert_eq!(method_1, method_2);

        let method_3 = ImpairmentCalculationMethod::FairValueLessDisposalCosts;
        assert_ne!(method_1, method_3);
    }

    // ============================================
    // InventoryValuationMethod テスト
    // ============================================

    #[test]
    fn test_inventory_valuation_methods() {
        let methods = [
            InventoryValuationMethod::Fifo,
            InventoryValuationMethod::Lifo,
            InventoryValuationMethod::WeightedAverageCost,
            InventoryValuationMethod::StandardCost,
        ];
        assert_eq!(methods.len(), 4);
    }

    #[test]
    fn test_inventory_valuation_method_display_names() {
        assert!(!InventoryValuationMethod::Fifo.display_name().is_empty());
        assert!(!InventoryValuationMethod::Lifo.display_name().is_empty());
        assert!(!InventoryValuationMethod::WeightedAverageCost.display_name().is_empty());
        assert!(!InventoryValuationMethod::StandardCost.display_name().is_empty());
    }

    #[test]
    fn test_inventory_valuation_method_equality() {
        assert_eq!(InventoryValuationMethod::Fifo, InventoryValuationMethod::Fifo);
        assert_ne!(InventoryValuationMethod::Fifo, InventoryValuationMethod::Lifo);
    }

    // ============================================
    // ProvisionType テスト
    // ============================================

    #[test]
    fn test_provision_type_all_variants() {
        let types = [
            ProvisionType::ProductWarranty,
            ProvisionType::LitigationRisk,
            ProvisionType::ContractualObligation,
            ProvisionType::EnvironmentalRestoration,
            ProvisionType::Restructuring,
        ];
        assert_eq!(types.len(), 5);
    }

    #[test]
    fn test_provision_type_display_names() {
        assert_eq!(ProvisionType::ProductWarranty.display_name(), "製品保証");
        assert_eq!(ProvisionType::LitigationRisk.display_name(), "訴訟");
        assert_eq!(ProvisionType::ContractualObligation.display_name(), "契約履行義務");
        assert_eq!(ProvisionType::EnvironmentalRestoration.display_name(), "環境整備");
        assert_eq!(ProvisionType::Restructuring.display_name(), "リストラクチャリング");
    }

    #[test]
    fn test_provision_type_validation() {
        assert!(ProvisionType::ProductWarranty.validate().is_ok());
        assert!(ProvisionType::LitigationRisk.validate().is_ok());
        assert!(ProvisionType::ContractualObligation.validate().is_ok());
        assert!(ProvisionType::EnvironmentalRestoration.validate().is_ok());
        assert!(ProvisionType::Restructuring.validate().is_ok());
    }

    #[test]
    fn test_provision_type_equality() {
        assert_eq!(ProvisionType::ProductWarranty, ProvisionType::ProductWarranty);
        assert_ne!(ProvisionType::ProductWarranty, ProvisionType::LitigationRisk);
    }
}
