// 計算バージョンの値オブジェクト

use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::DomainResult, financial_close::DomainError, value_object::ValueObject};

/// 計算バージョンID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CalculationVersionId(Uuid);

impl CalculationVersionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl Default for CalculationVersionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CalculationVersionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl crate::entity::EntityId for CalculationVersionId {
    fn value(&self) -> &str {
        Box::leak(self.0.to_string().into_boxed_str())
    }
}

/// バージョンステータス
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionStatus {
    /// 草案
    Draft,
    /// 承認待ち
    PendingApproval,
    /// 承認済み
    Approved,
    /// 有効
    Active,
    /// 非推奨
    Deprecated,
    /// アーカイブ済み
    Archived,
}

impl VersionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Draft => "Draft",
            Self::PendingApproval => "PendingApproval",
            Self::Approved => "Approved",
            Self::Active => "Active",
            Self::Deprecated => "Deprecated",
            Self::Archived => "Archived",
        }
    }

    pub fn can_transition_to(&self, new_status: &VersionStatus) -> bool {
        match (self, new_status) {
            (Self::Draft, Self::PendingApproval) => true,
            (Self::PendingApproval, Self::Approved) => true,
            (Self::PendingApproval, Self::Draft) => true, // Reject
            (Self::Approved, Self::Active) => true,
            (Self::Active, Self::Deprecated) => true,
            (Self::Deprecated, Self::Archived) => true,
            _ => false,
        }
    }
}

impl fmt::Display for VersionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for VersionStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Draft" => Ok(Self::Draft),
            "PendingApproval" => Ok(Self::PendingApproval),
            "Approved" => Ok(Self::Approved),
            "Active" => Ok(Self::Active),
            "Deprecated" => Ok(Self::Deprecated),
            "Archived" => Ok(Self::Archived),
            _ => Err(DomainError::InvalidCalculationVersion),
        }
    }
}

/// パラメータタイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    /// 為替レート
    ExchangeRate,
    /// 割引率
    DiscountRate,
    /// 金利
    InterestRate,
    /// ECLパラメータ（PD/LGD/EAD）
    EclParameter,
    /// 将来キャッシュフロー予測
    FutureCashFlow,
    /// 市場データ
    MarketData,
    /// 税率
    TaxRate,
    /// その他
    Other(String),
}

impl ParameterType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ExchangeRate => "ExchangeRate",
            Self::DiscountRate => "DiscountRate",
            Self::InterestRate => "InterestRate",
            Self::EclParameter => "EclParameter",
            Self::FutureCashFlow => "FutureCashFlow",
            Self::MarketData => "MarketData",
            Self::TaxRate => "TaxRate",
            Self::Other(s) => s,
        }
    }
}

impl fmt::Display for ParameterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// バージョン番号
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VersionNumber {
    major: u32,
    minor: u32,
    patch: u32,
}

impl VersionNumber {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn increment_major(&self) -> Self {
        Self { major: self.major + 1, minor: 0, patch: 0 }
    }

    pub fn increment_minor(&self) -> Self {
        Self { major: self.major, minor: self.minor + 1, patch: 0 }
    }

    pub fn increment_patch(&self) -> Self {
        Self { major: self.major, minor: self.minor, patch: self.patch + 1 }
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn patch(&self) -> u32 {
        self.patch
    }
}

impl fmt::Display for VersionNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl std::str::FromStr for VersionNumber {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(DomainError::InvalidCalculationVersion);
        }

        let major = parts[0].parse::<u32>().map_err(|_| DomainError::InvalidCalculationVersion)?;
        let minor = parts[1].parse::<u32>().map_err(|_| DomainError::InvalidCalculationVersion)?;
        let patch = parts[2].parse::<u32>().map_err(|_| DomainError::InvalidCalculationVersion)?;

        Ok(Self { major, minor, patch })
    }
}

impl ValueObject for VersionNumber {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation_version_id_creation() {
        let id1 = CalculationVersionId::new();
        let id2 = CalculationVersionId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_version_status_transitions() {
        assert!(VersionStatus::Draft.can_transition_to(&VersionStatus::PendingApproval));
        assert!(VersionStatus::PendingApproval.can_transition_to(&VersionStatus::Approved));
        assert!(VersionStatus::Approved.can_transition_to(&VersionStatus::Active));
        assert!(VersionStatus::Active.can_transition_to(&VersionStatus::Deprecated));
        assert!(VersionStatus::Deprecated.can_transition_to(&VersionStatus::Archived));

        // Invalid transitions
        assert!(!VersionStatus::Draft.can_transition_to(&VersionStatus::Active));
        assert!(!VersionStatus::Active.can_transition_to(&VersionStatus::Draft));
    }

    #[test]
    fn test_version_number_creation() {
        let version = VersionNumber::new(1, 2, 3);
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_number_increment() {
        let version = VersionNumber::new(1, 2, 3);

        let major = version.increment_major();
        assert_eq!(major.to_string(), "2.0.0");

        let minor = version.increment_minor();
        assert_eq!(minor.to_string(), "1.3.0");

        let patch = version.increment_patch();
        assert_eq!(patch.to_string(), "1.2.4");
    }

    #[test]
    fn test_version_number_from_str() {
        let version: VersionNumber = "1.2.3".parse().unwrap();
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
    }

    #[test]
    fn test_version_number_invalid_format() {
        assert!("1.2".parse::<VersionNumber>().is_err());
        assert!("1.2.3.4".parse::<VersionNumber>().is_err());
        assert!("a.b.c".parse::<VersionNumber>().is_err());
    }

    #[test]
    fn test_version_number_comparison() {
        let v1 = VersionNumber::new(1, 0, 0);
        let v2 = VersionNumber::new(1, 1, 0);
        let v3 = VersionNumber::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
    }
}
