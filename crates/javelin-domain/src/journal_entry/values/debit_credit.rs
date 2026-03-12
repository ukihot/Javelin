// DebitCredit - 借方/貸方区分値オブジェクト

use std::str::FromStr;

use crate::{error::DomainResult, value_object::ValueObject};

/// 借方/貸方区分
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebitCredit {
    /// 借方
    Debit,
    /// 貸方
    Credit,
}

impl FromStr for DebitCredit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Debit" => Ok(DebitCredit::Debit),
            "Credit" => Ok(DebitCredit::Credit),
            _ => Err(format!("Invalid DebitCredit: {}", s)),
        }
    }
}

impl DebitCredit {
    pub fn as_str(&self) -> &str {
        match self {
            DebitCredit::Debit => "Debit",
            DebitCredit::Credit => "Credit",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            DebitCredit::Debit => "借方",
            DebitCredit::Credit => "貸方",
        }
    }

    pub fn is_debit(&self) -> bool {
        matches!(self, DebitCredit::Debit)
    }

    pub fn is_credit(&self) -> bool {
        matches!(self, DebitCredit::Credit)
    }
}

impl ValueObject for DebitCredit {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}
