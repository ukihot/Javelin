// 収益認識のエンティティ

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{
    ContractId, ContractStatus, PerformanceObligationId, ProgressRate, RecognitionPattern,
    RecognitionTiming, StandaloneSellingPrice, TransactionPrice, VariableConsiderationMethod,
};
pub use super::values::{
    ContractId as ContractIdExport, PerformanceObligationId as PerformanceObligationIdExport,
};
use crate::{
    entity::Entity,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 契約エンティティ（IFRS 15 Step 1）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// 契約ID
    id: ContractId,
    /// 顧客ID
    customer_id: String,
    /// 契約日
    contract_date: DateTime<Utc>,
    /// 契約ステータス
    status: ContractStatus,
    /// 取引価格
    transaction_price: TransactionPrice,
    /// 変動対価見積方法
    variable_consideration_method: Option<VariableConsiderationMethod>,
    /// 履行義務
    performance_obligations: Vec<PerformanceObligation>,
    /// 契約結合判定結果
    is_combined: bool,
    /// 結合元契約ID（結合の場合）
    combined_from_contracts: Vec<ContractId>,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// 更新日時
    updated_at: DateTime<Utc>,
}

impl Contract {
    pub fn new(
        id: ContractId,
        customer_id: String,
        contract_date: DateTime<Utc>,
        transaction_price: TransactionPrice,
    ) -> DomainResult<Self> {
        if customer_id.is_empty() {
            return Err(DomainError::InvalidContract);
        }

        transaction_price.validate()?;

        let now = Utc::now();
        Ok(Self {
            id,
            customer_id,
            contract_date,
            status: ContractStatus::Identified,
            transaction_price,
            variable_consideration_method: None,
            performance_obligations: Vec::new(),
            is_combined: false,
            combined_from_contracts: Vec::new(),
            created_at: now,
            updated_at: now,
        })
    }

    /// 履行義務を追加（IFRS 15 Step 2）
    pub fn add_performance_obligation(
        &mut self,
        obligation: PerformanceObligation,
    ) -> DomainResult<()> {
        // 同じIDの履行義務が既に存在しないかチェック
        if self.performance_obligations.iter().any(|po| po.id() == obligation.id()) {
            return Err(DomainError::InvalidPerformanceObligation);
        }

        self.performance_obligations.push(obligation);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 取引価格を配分（IFRS 15 Step 4）
    pub fn allocate_transaction_price(&mut self) -> DomainResult<()> {
        if self.performance_obligations.is_empty() {
            return Err(DomainError::InvalidPerformanceObligation);
        }

        let total_transaction_price = self.transaction_price.total();

        // 独立販売価格の合計を計算
        let total_ssp: i64 = self
            .performance_obligations
            .iter()
            .map(|po| po.standalone_selling_price().amount())
            .sum();

        if total_ssp == 0 {
            return Err(DomainError::InvalidStandaloneSellingPrice);
        }

        // 独立販売価格比率法で配分
        for obligation in &mut self.performance_obligations {
            let ssp = obligation.standalone_selling_price().amount();
            let allocated_amount = (total_transaction_price * ssp) / total_ssp;
            obligation.set_allocated_price(allocated_amount)?;
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    /// 契約を有効化
    pub fn activate(&mut self) -> DomainResult<()> {
        if !matches!(self.status, ContractStatus::Identified) {
            return Err(DomainError::InvalidContract);
        }

        self.status = ContractStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 契約を変更
    pub fn modify(&mut self, new_transaction_price: TransactionPrice) -> DomainResult<()> {
        new_transaction_price.validate()?;

        self.transaction_price = new_transaction_price;
        self.status = ContractStatus::Modified;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 契約を完了
    pub fn complete(&mut self) -> DomainResult<()> {
        // すべての履行義務が完了しているかチェック
        if !self.performance_obligations.iter().all(|po| po.is_satisfied()) {
            return Err(DomainError::InvalidContract);
        }

        self.status = ContractStatus::Completed;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 契約結合を設定
    pub fn set_combined(&mut self, combined_from: Vec<ContractId>) {
        self.is_combined = true;
        self.combined_from_contracts = combined_from;
        self.updated_at = Utc::now();
    }

    /// 変動対価見積方法を設定
    pub fn set_variable_consideration_method(&mut self, method: VariableConsiderationMethod) {
        self.variable_consideration_method = Some(method);
        self.updated_at = Utc::now();
    }

    // Getters
    pub fn id(&self) -> &ContractId {
        &self.id
    }

    pub fn customer_id(&self) -> &str {
        &self.customer_id
    }

    pub fn contract_date(&self) -> DateTime<Utc> {
        self.contract_date
    }

    pub fn status(&self) -> &ContractStatus {
        &self.status
    }

    pub fn transaction_price(&self) -> &TransactionPrice {
        &self.transaction_price
    }

    pub fn variable_consideration_method(&self) -> Option<&VariableConsiderationMethod> {
        self.variable_consideration_method.as_ref()
    }

    pub fn performance_obligations(&self) -> &[PerformanceObligation] {
        &self.performance_obligations
    }

    pub fn is_combined(&self) -> bool {
        self.is_combined
    }

    pub fn combined_from_contracts(&self) -> &[ContractId] {
        &self.combined_from_contracts
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Entity for Contract {
    type Id = ContractId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

/// 履行義務エンティティ（IFRS 15 Step 2）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceObligation {
    /// 履行義務ID
    id: PerformanceObligationId,
    /// 履行義務の説明
    description: String,
    /// 独立販売価格
    standalone_selling_price: StandaloneSellingPrice,
    /// 配分された取引価格
    allocated_price: Option<i64>,
    /// 収益認識タイミング
    recognition_timing: RecognitionTiming,
    /// 収益認識パターン（期間認識の場合）
    recognition_pattern: Option<RecognitionPattern>,
    /// 進捗度
    progress_rate: ProgressRate,
    /// 認識済収益額
    recognized_revenue: i64,
    /// 履行義務充足済フラグ
    is_satisfied: bool,
    /// 別個の財・サービスか
    is_distinct: bool,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// 更新日時
    updated_at: DateTime<Utc>,
}

impl PerformanceObligation {
    pub fn new(
        id: PerformanceObligationId,
        description: String,
        standalone_selling_price: StandaloneSellingPrice,
        recognition_timing: RecognitionTiming,
        is_distinct: bool,
    ) -> DomainResult<Self> {
        if description.is_empty() {
            return Err(DomainError::InvalidPerformanceObligation);
        }

        standalone_selling_price.validate()?;

        let now = Utc::now();
        Ok(Self {
            id,
            description,
            standalone_selling_price,
            allocated_price: None,
            recognition_timing,
            recognition_pattern: None,
            progress_rate: ProgressRate::new(0)?,
            recognized_revenue: 0,
            is_satisfied: false,
            is_distinct,
            created_at: now,
            updated_at: now,
        })
    }

    /// 配分価格を設定
    pub fn set_allocated_price(&mut self, price: i64) -> DomainResult<()> {
        if price < 0 {
            return Err(DomainError::InvalidTransactionPrice);
        }

        self.allocated_price = Some(price);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 収益認識パターンを設定
    pub fn set_recognition_pattern(&mut self, pattern: RecognitionPattern) {
        self.recognition_pattern = Some(pattern);
        self.updated_at = Utc::now();
    }

    /// 進捗度を更新
    pub fn update_progress(&mut self, progress_rate: ProgressRate) -> DomainResult<()> {
        progress_rate.validate()?;

        self.progress_rate = progress_rate;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 収益を認識（IFRS 15 Step 5）
    pub fn recognize_revenue(&mut self, amount: i64) -> DomainResult<()> {
        if amount < 0 {
            return Err(DomainError::InvalidTransactionPrice);
        }

        let allocated_price =
            self.allocated_price.ok_or(DomainError::InvalidPerformanceObligation)?;

        if self.recognized_revenue + amount > allocated_price {
            return Err(DomainError::InvalidTransactionPrice);
        }

        self.recognized_revenue += amount;

        // 全額認識済の場合は履行義務充足
        if self.recognized_revenue == allocated_price {
            self.is_satisfied = true;
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    /// 残存収益額を計算
    pub fn remaining_revenue(&self) -> i64 {
        self.allocated_price.unwrap_or(0) - self.recognized_revenue
    }

    // Getters
    pub fn id(&self) -> &PerformanceObligationId {
        &self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn standalone_selling_price(&self) -> &StandaloneSellingPrice {
        &self.standalone_selling_price
    }

    pub fn allocated_price(&self) -> Option<i64> {
        self.allocated_price
    }

    pub fn recognition_timing(&self) -> &RecognitionTiming {
        &self.recognition_timing
    }

    pub fn recognition_pattern(&self) -> Option<&RecognitionPattern> {
        self.recognition_pattern.as_ref()
    }

    pub fn progress_rate(&self) -> &ProgressRate {
        &self.progress_rate
    }

    pub fn recognized_revenue(&self) -> i64 {
        self.recognized_revenue
    }

    pub fn is_satisfied(&self) -> bool {
        self.is_satisfied
    }

    pub fn is_distinct(&self) -> bool {
        self.is_distinct
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Entity for PerformanceObligation {
    type Id = PerformanceObligationId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_contract() -> Contract {
        let id = ContractId::new();
        let transaction_price = TransactionPrice::new(1_000_000, 0, 0, 0).unwrap();
        Contract::new(id, "CUST001".to_string(), Utc::now(), transaction_price).unwrap()
    }

    fn create_test_performance_obligation() -> PerformanceObligation {
        let id = PerformanceObligationId::new();
        let ssp = StandaloneSellingPrice::new(500_000, None).unwrap();
        let timing = RecognitionTiming::PointInTime { transfer_date: Utc::now() };
        PerformanceObligation::new(id, "Product Delivery".to_string(), ssp, timing, true).unwrap()
    }

    #[test]
    fn test_contract_creation() {
        let contract = create_test_contract();
        assert_eq!(contract.customer_id(), "CUST001");
        assert_eq!(contract.status(), &ContractStatus::Identified);
        assert_eq!(contract.transaction_price().total(), 1_000_000);
    }

    #[test]
    fn test_contract_invalid_customer_id() {
        let id = ContractId::new();
        let transaction_price = TransactionPrice::new(1_000_000, 0, 0, 0).unwrap();
        let result = Contract::new(id, "".to_string(), Utc::now(), transaction_price);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_performance_obligation() {
        let mut contract = create_test_contract();
        let obligation = create_test_performance_obligation();

        assert!(contract.add_performance_obligation(obligation).is_ok());
        assert_eq!(contract.performance_obligations().len(), 1);
    }

    #[test]
    fn test_allocate_transaction_price() {
        let mut contract = create_test_contract();

        let id1 = PerformanceObligationId::new();
        let ssp1 = StandaloneSellingPrice::new(600_000, None).unwrap();
        let timing1 = RecognitionTiming::PointInTime { transfer_date: Utc::now() };
        let obligation1 =
            PerformanceObligation::new(id1, "Product".to_string(), ssp1, timing1, true).unwrap();

        let id2 = PerformanceObligationId::new();
        let ssp2 = StandaloneSellingPrice::new(400_000, None).unwrap();
        let timing2 = RecognitionTiming::PointInTime { transfer_date: Utc::now() };
        let obligation2 =
            PerformanceObligation::new(id2, "Service".to_string(), ssp2, timing2, true).unwrap();

        contract.add_performance_obligation(obligation1).unwrap();
        contract.add_performance_obligation(obligation2).unwrap();

        assert!(contract.allocate_transaction_price().is_ok());

        // 取引価格1,000,000を600,000:400,000の比率で配分
        assert_eq!(contract.performance_obligations()[0].allocated_price(), Some(600_000));
        assert_eq!(contract.performance_obligations()[1].allocated_price(), Some(400_000));
    }

    #[test]
    fn test_contract_activate() {
        let mut contract = create_test_contract();
        assert!(contract.activate().is_ok());
        assert_eq!(contract.status(), &ContractStatus::Active);
    }

    #[test]
    fn test_contract_modify() {
        let mut contract = create_test_contract();
        let new_price = TransactionPrice::new(1_200_000, 0, 0, 0).unwrap();

        assert!(contract.modify(new_price).is_ok());
        assert_eq!(contract.status(), &ContractStatus::Modified);
        assert_eq!(contract.transaction_price().total(), 1_200_000);
    }

    #[test]
    fn test_contract_complete() {
        let mut contract = create_test_contract();
        let mut obligation = create_test_performance_obligation();

        obligation.set_allocated_price(500_000).unwrap();
        obligation.recognize_revenue(500_000).unwrap();

        contract.add_performance_obligation(obligation).unwrap();

        assert!(contract.complete().is_ok());
        assert_eq!(contract.status(), &ContractStatus::Completed);
    }

    #[test]
    fn test_performance_obligation_creation() {
        let obligation = create_test_performance_obligation();
        assert_eq!(obligation.description(), "Product Delivery");
        assert_eq!(obligation.standalone_selling_price().amount(), 500_000);
        assert!(obligation.is_distinct());
    }

    #[test]
    fn test_performance_obligation_recognize_revenue() {
        let mut obligation = create_test_performance_obligation();
        obligation.set_allocated_price(500_000).unwrap();

        assert!(obligation.recognize_revenue(300_000).is_ok());
        assert_eq!(obligation.recognized_revenue(), 300_000);
        assert_eq!(obligation.remaining_revenue(), 200_000);
        assert!(!obligation.is_satisfied());

        assert!(obligation.recognize_revenue(200_000).is_ok());
        assert_eq!(obligation.recognized_revenue(), 500_000);
        assert_eq!(obligation.remaining_revenue(), 0);
        assert!(obligation.is_satisfied());
    }

    #[test]
    fn test_performance_obligation_excessive_revenue() {
        let mut obligation = create_test_performance_obligation();
        obligation.set_allocated_price(500_000).unwrap();

        assert!(obligation.recognize_revenue(600_000).is_err());
    }

    #[test]
    fn test_performance_obligation_update_progress() {
        let mut obligation = create_test_performance_obligation();
        let progress = ProgressRate::new(50).unwrap();

        assert!(obligation.update_progress(progress).is_ok());
        assert_eq!(obligation.progress_rate().percentage(), 50);
    }
}
