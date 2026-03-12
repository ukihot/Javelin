// AccountType - 勘定科目区分値オブジェクト

/// 勘定科目タイプ
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl AccountType {
    /// 文字列表現を取得
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountType::Asset => "Asset",
            AccountType::Liability => "Liability",
            AccountType::Equity => "Equity",
            AccountType::Revenue => "Revenue",
            AccountType::Expense => "Expense",
        }
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for AccountType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Asset" => Ok(AccountType::Asset),
            "Liability" => Ok(AccountType::Liability),
            "Equity" => Ok(AccountType::Equity),
            "Revenue" => Ok(AccountType::Revenue),
            "Expense" => Ok(AccountType::Expense),
            _ => Err(format!("Invalid AccountType: {}", s)),
        }
    }
}
