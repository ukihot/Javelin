// 計算バージョンのエンティティ

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::values::{CalculationVersionId, ParameterType, VersionNumber, VersionStatus};
use crate::{
    entity::Entity,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 計算ロジックバージョンエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationLogicVersion {
    /// バージョンID
    id: CalculationVersionId,
    /// バージョン番号
    version_number: VersionNumber,
    /// ロジック名
    logic_name: String,
    /// ロジック説明
    description: String,
    /// ロジックハッシュ（SHA-256）
    logic_hash: String,
    /// ステータス
    status: VersionStatus,
    /// 有効開始日
    effective_from: DateTime<Utc>,
    /// 有効終了日
    effective_to: Option<DateTime<Utc>>,
    /// 承認履歴
    approval_history: Vec<ApprovalRecord>,
    /// 作成者ID
    created_by: String,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// 更新日時
    updated_at: DateTime<Utc>,
}

impl CalculationLogicVersion {
    pub fn new(
        id: CalculationVersionId,
        version_number: VersionNumber,
        logic_name: String,
        description: String,
        logic_hash: String,
        created_by: String,
    ) -> DomainResult<Self> {
        if logic_name.is_empty() || logic_hash.is_empty() || created_by.is_empty() {
            return Err(DomainError::InvalidCalculationVersion);
        }

        let now = Utc::now();
        Ok(Self {
            id,
            version_number,
            logic_name,
            description,
            logic_hash,
            status: VersionStatus::Draft,
            effective_from: now,
            effective_to: None,
            approval_history: Vec::new(),
            created_by,
            created_at: now,
            updated_at: now,
        })
    }

    /// ステータスを変更
    pub fn change_status(&mut self, new_status: VersionStatus) -> DomainResult<()> {
        if !self.status.can_transition_to(&new_status) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = new_status;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 承認を記録
    pub fn record_approval(&mut self, record: ApprovalRecord) -> DomainResult<()> {
        record.validate()?;
        self.approval_history.push(record);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 有効期間を設定
    pub fn set_effective_period(
        &mut self,
        from: DateTime<Utc>,
        to: Option<DateTime<Utc>>,
    ) -> DomainResult<()> {
        if let Some(end) = to
            && end <= from
        {
            return Err(DomainError::InvalidCalculationVersion);
        }

        self.effective_from = from;
        self.effective_to = to;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 指定日時に有効か確認
    pub fn is_effective_at(&self, date: DateTime<Utc>) -> bool {
        if date < self.effective_from {
            return false;
        }

        if let Some(to) = self.effective_to
            && date > to
        {
            return false;
        }

        true
    }

    // Getters
    pub fn id(&self) -> &CalculationVersionId {
        &self.id
    }

    pub fn version_number(&self) -> &VersionNumber {
        &self.version_number
    }

    pub fn logic_name(&self) -> &str {
        &self.logic_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn logic_hash(&self) -> &str {
        &self.logic_hash
    }

    pub fn status(&self) -> &VersionStatus {
        &self.status
    }

    pub fn effective_from(&self) -> DateTime<Utc> {
        self.effective_from
    }

    pub fn effective_to(&self) -> Option<DateTime<Utc>> {
        self.effective_to
    }

    pub fn approval_history(&self) -> &[ApprovalRecord] {
        &self.approval_history
    }

    pub fn created_by(&self) -> &str {
        &self.created_by
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Entity for CalculationLogicVersion {
    type Id = CalculationVersionId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

/// 承認記録
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRecord {
    /// 承認者ID
    approver_id: String,
    /// 承認日時
    approved_at: DateTime<Utc>,
    /// 承認コメント
    comment: String,
    /// 承認結果
    approved: bool,
}

impl ApprovalRecord {
    pub fn new(approver_id: String, comment: String, approved: bool) -> DomainResult<Self> {
        if approver_id.is_empty() {
            return Err(DomainError::InvalidCalculationVersion);
        }

        Ok(Self { approver_id, approved_at: Utc::now(), comment, approved })
    }

    pub fn approver_id(&self) -> &str {
        &self.approver_id
    }

    pub fn approved_at(&self) -> DateTime<Utc> {
        self.approved_at
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn approved(&self) -> bool {
        self.approved
    }
}

impl ValueObject for ApprovalRecord {
    fn validate(&self) -> DomainResult<()> {
        if self.approver_id.is_empty() {
            return Err(DomainError::InvalidCalculationVersion);
        }
        Ok(())
    }
}

/// 計算パラメータエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationParameter {
    /// パラメータ名
    name: String,
    /// パラメータタイプ
    parameter_type: ParameterType,
    /// パラメータ値
    value: String,
    /// 取得日時
    obtained_at: DateTime<Utc>,
    /// 取得元
    source: String,
    /// メタデータ
    metadata: HashMap<String, String>,
}

impl CalculationParameter {
    pub fn new(
        name: String,
        parameter_type: ParameterType,
        value: String,
        source: String,
    ) -> DomainResult<Self> {
        if name.is_empty() || value.is_empty() || source.is_empty() {
            return Err(DomainError::InvalidCalculationVersion);
        }

        Ok(Self {
            name,
            parameter_type,
            value,
            obtained_at: Utc::now(),
            source,
            metadata: HashMap::new(),
        })
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parameter_type(&self) -> &ParameterType {
        &self.parameter_type
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn obtained_at(&self) -> DateTime<Utc> {
        self.obtained_at
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_version() -> CalculationLogicVersion {
        let id = CalculationVersionId::new();
        let version_number = VersionNumber::new(1, 0, 0);
        CalculationLogicVersion::new(
            id,
            version_number,
            "ECL Calculation".to_string(),
            "Expected Credit Loss calculation logic".to_string(),
            "abc123def456".to_string(),
            "USER001".to_string(),
        )
        .unwrap()
    }

    #[test]
    fn test_calculation_logic_version_creation() {
        let version = create_test_version();
        assert_eq!(version.logic_name(), "ECL Calculation");
        assert_eq!(version.status(), &VersionStatus::Draft);
        assert_eq!(version.created_by(), "USER001");
    }

    #[test]
    fn test_change_status() {
        let mut version = create_test_version();
        assert!(version.change_status(VersionStatus::PendingApproval).is_ok());
        assert_eq!(version.status(), &VersionStatus::PendingApproval);
    }

    #[test]
    fn test_change_status_invalid_transition() {
        let mut version = create_test_version();
        assert!(version.change_status(VersionStatus::Active).is_err());
    }

    #[test]
    fn test_record_approval() {
        let mut version = create_test_version();
        let approval = ApprovalRecord::new(
            "APPROVER001".to_string(),
            "Approved for production".to_string(),
            true,
        )
        .unwrap();

        assert!(version.record_approval(approval).is_ok());
        assert_eq!(version.approval_history().len(), 1);
    }

    #[test]
    fn test_set_effective_period() {
        let mut version = create_test_version();
        let from = Utc::now();
        let to = from + chrono::Duration::days(365);

        assert!(version.set_effective_period(from, Some(to)).is_ok());
        assert_eq!(version.effective_from(), from);
        assert_eq!(version.effective_to(), Some(to));
    }

    #[test]
    fn test_is_effective_at() {
        let mut version = create_test_version();
        let from = Utc::now();
        let to = from + chrono::Duration::days(365);
        version.set_effective_period(from, Some(to)).unwrap();

        assert!(version.is_effective_at(from + chrono::Duration::days(100)));
        assert!(!version.is_effective_at(from - chrono::Duration::days(1)));
        assert!(!version.is_effective_at(to + chrono::Duration::days(1)));
    }

    #[test]
    fn test_calculation_parameter_creation() {
        let param = CalculationParameter::new(
            "discount_rate".to_string(),
            ParameterType::DiscountRate,
            "0.05".to_string(),
            "Central Bank".to_string(),
        )
        .unwrap();

        assert_eq!(param.name(), "discount_rate");
        assert_eq!(param.value(), "0.05");
        assert_eq!(param.source(), "Central Bank");
    }

    #[test]
    fn test_calculation_parameter_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("currency".to_string(), "JPY".to_string());
        metadata.insert("term".to_string(), "10Y".to_string());

        let param = CalculationParameter::new(
            "interest_rate".to_string(),
            ParameterType::InterestRate,
            "0.02".to_string(),
            "Market Data Provider".to_string(),
        )
        .unwrap()
        .with_metadata(metadata);

        assert_eq!(param.metadata().len(), 2);
        assert_eq!(param.metadata().get("currency"), Some(&"JPY".to_string()));
    }
}
