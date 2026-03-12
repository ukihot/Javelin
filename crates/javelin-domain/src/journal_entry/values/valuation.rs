// Valuation - 評価関連の値オブジェクト

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
