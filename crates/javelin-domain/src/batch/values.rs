// Batch Value Objects

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

/// バッチ実行ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BatchExecutionId(String);

impl BatchExecutionId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BatchExecutionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// バッチ種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchType {
    /// 元帳集計
    LedgerConsolidation,
    /// 決算準備
    ClosingPreparation,
    /// 勘定調整
    AccountAdjustment,
    /// IFRS評価
    IfrsValuation,
    /// 財務諸表生成
    FinancialStatement,
}

impl BatchType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LedgerConsolidation => "LedgerConsolidation",
            Self::ClosingPreparation => "ClosingPreparation",
            Self::AccountAdjustment => "AccountAdjustment",
            Self::IfrsValuation => "IfrsValuation",
            Self::FinancialStatement => "FinancialStatement",
        }
    }
}

impl fmt::Display for BatchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for BatchType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LedgerConsolidation" => Ok(Self::LedgerConsolidation),
            "ClosingPreparation" => Ok(Self::ClosingPreparation),
            "AccountAdjustment" => Ok(Self::AccountAdjustment),
            "IfrsValuation" => Ok(Self::IfrsValuation),
            "FinancialStatement" => Ok(Self::FinancialStatement),
            _ => Err(format!("Invalid batch type: {}", s)),
        }
    }
}

/// バッチ実行ステータス
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchStatus {
    /// 実行中
    Running,
    /// 完了
    Completed,
    /// 失敗
    Failed,
}

impl BatchStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "Running",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
        }
    }
}

impl fmt::Display for BatchStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for BatchStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Running" => Ok(Self::Running),
            "Completed" => Ok(Self::Completed),
            "Failed" => Ok(Self::Failed),
            _ => Err(format!("Invalid batch status: {}", s)),
        }
    }
}
