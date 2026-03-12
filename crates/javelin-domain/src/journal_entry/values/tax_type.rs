// TaxType - 税区分値オブジェクト

use std::str::FromStr;

use crate::{error::DomainResult, value_object::ValueObject};

/// 税区分
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaxType {
    /// 課税
    Taxable,
    /// 非課税
    NonTaxable,
    /// 免税
    TaxExempt,
    /// 不課税
    OutOfScope,
}

impl FromStr for TaxType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Taxable" => Ok(TaxType::Taxable),
            "NonTaxable" => Ok(TaxType::NonTaxable),
            "TaxExempt" => Ok(TaxType::TaxExempt),
            "OutOfScope" => Ok(TaxType::OutOfScope),
            _ => Err(format!("Invalid TaxType: {}", s)),
        }
    }
}

impl TaxType {
    pub fn as_str(&self) -> &str {
        match self {
            TaxType::Taxable => "Taxable",
            TaxType::NonTaxable => "NonTaxable",
            TaxType::TaxExempt => "TaxExempt",
            TaxType::OutOfScope => "OutOfScope",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            TaxType::Taxable => "課税",
            TaxType::NonTaxable => "非課税",
            TaxType::TaxExempt => "免税",
            TaxType::OutOfScope => "不課税",
        }
    }
}

impl ValueObject for TaxType {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}
