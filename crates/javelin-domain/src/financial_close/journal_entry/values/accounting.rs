// 会計関連の値オブジェクト

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
    /// 文字列に変換
    pub fn as_str(&self) -> &str {
        match self {
            DebitCredit::Debit => "Debit",
            DebitCredit::Credit => "Credit",
        }
    }

    /// 表示名を取得
    pub fn display_name(&self) -> &str {
        match self {
            DebitCredit::Debit => "借方",
            DebitCredit::Credit => "貸方",
        }
    }

    /// 借方かどうかを判定
    pub fn is_debit(&self) -> bool {
        matches!(self, DebitCredit::Debit)
    }

    /// 貸方かどうかを判定
    pub fn is_credit(&self) -> bool {
        matches!(self, DebitCredit::Credit)
    }
}

impl ValueObject for DebitCredit {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

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
    /// 文字列に変換
    pub fn as_str(&self) -> &str {
        match self {
            TaxType::Taxable => "Taxable",
            TaxType::NonTaxable => "NonTaxable",
            TaxType::TaxExempt => "TaxExempt",
            TaxType::OutOfScope => "OutOfScope",
        }
    }

    /// 表示名を取得
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

/// 仕訳行為区分
///
/// 経済事象の認識および修正行為の意味を明確化するための区分。
/// 修正履歴の解釈可能性、監査証跡の明確化および会計判断過程の再現性を担保する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JournalEntryType {
    /// 新規起票仕訳 - 経済事象の第一次認識として記録される仕訳
    NewEntry,
    /// 取消仕訳 - 既存仕訳の効力を無効化する目的で計上される仕訳
    Cancellation,
    /// 反対仕訳 - 既存残高または期間帰属を反転させるための仕訳
    Reversal,
    /// 追加仕訳 - 計上不足または後日判明事項を補正する仕訳
    Additional,
    /// 再分類仕訳 - 測定額を変更せず表示区分のみ変更する仕訳
    Reclassification,
    /// 洗替仕訳 - 既存評価額を一旦消去し再評価する仕訳
    Replacement,
}

impl JournalEntryType {
    /// 既存伝票の参照が必要かどうか
    pub fn requires_reference(&self) -> bool {
        !matches!(self, JournalEntryType::NewEntry)
    }

    /// 表示名を取得
    pub fn display_name(&self) -> &str {
        match self {
            JournalEntryType::NewEntry => "新規起票",
            JournalEntryType::Cancellation => "取消",
            JournalEntryType::Reversal => "反対",
            JournalEntryType::Additional => "追加",
            JournalEntryType::Reclassification => "再分類",
            JournalEntryType::Replacement => "洗替",
        }
    }
}

impl ValueObject for JournalEntryType {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debit_credit() {
        assert!(DebitCredit::Debit.is_debit());
        assert!(!DebitCredit::Debit.is_credit());
        assert!(DebitCredit::Credit.is_credit());
        assert!(!DebitCredit::Credit.is_debit());

        assert_eq!(DebitCredit::Debit.as_str(), "Debit");
        assert_eq!(DebitCredit::Credit.as_str(), "Credit");
    }

    #[test]
    fn test_tax_type() {
        assert_eq!(TaxType::Taxable.as_str(), "Taxable");
        assert_eq!(TaxType::NonTaxable.as_str(), "NonTaxable");
        assert_eq!(TaxType::TaxExempt.as_str(), "TaxExempt");
        assert_eq!(TaxType::OutOfScope.as_str(), "OutOfScope");
    }

    #[test]
    fn test_journal_entry_type() {
        assert!(!JournalEntryType::NewEntry.requires_reference());
        assert!(JournalEntryType::Cancellation.requires_reference());
        assert!(JournalEntryType::Reversal.requires_reference());
        assert!(JournalEntryType::Additional.requires_reference());
        assert!(JournalEntryType::Reclassification.requires_reference());
        assert!(JournalEntryType::Replacement.requires_reference());
    }

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            // プロパティ1: DebitCreditの相互排他性
            #[test]
            fn prop_debit_credit_mutual_exclusivity(
                dc in prop_oneof![Just(DebitCredit::Debit), Just(DebitCredit::Credit)]
            ) {
                if dc.is_debit() {
                    prop_assert!(!dc.is_credit());
                } else {
                    prop_assert!(!dc.is_debit());
                    prop_assert!(dc.is_credit());
                }
            }

            // プロパティ2: DebitCreditのFromStr往復変換
            #[test]
            fn prop_debit_credit_from_str_roundtrip(
                dc in prop_oneof![Just(DebitCredit::Debit), Just(DebitCredit::Credit)]
            ) {
                let str_repr = dc.as_str();
                let parsed = DebitCredit::from_str(str_repr);
                prop_assert!(parsed.is_ok());
                prop_assert_eq!(parsed.unwrap(), dc);
            }

            // プロパティ3: TaxTypeのFromStr往復変換
            #[test]
            fn prop_tax_type_from_str_roundtrip(
                tax_type in prop_oneof![
                    Just(TaxType::Taxable),
                    Just(TaxType::NonTaxable),
                    Just(TaxType::TaxExempt),
                    Just(TaxType::OutOfScope),
                ]
            ) {
                let str_repr = tax_type.as_str();
                let parsed = TaxType::from_str(str_repr);
                prop_assert!(parsed.is_ok());
                prop_assert_eq!(parsed.unwrap(), tax_type);
            }

            // プロパティ4: JournalEntryTypeの参照要件の一貫性
            #[test]
            fn prop_journal_entry_type_reference_requirement(
                entry_type in prop_oneof![
                    Just(JournalEntryType::NewEntry),
                    Just(JournalEntryType::Cancellation),
                    Just(JournalEntryType::Reversal),
                    Just(JournalEntryType::Additional),
                    Just(JournalEntryType::Reclassification),
                    Just(JournalEntryType::Replacement),
                ]
            ) {
                match entry_type {
                    JournalEntryType::NewEntry => {
                        prop_assert!(!entry_type.requires_reference());
                    }
                    _ => {
                        prop_assert!(entry_type.requires_reference());
                    }
                }
            }

            // プロパティ5: すべての値オブジェクトは検証を通過
            #[test]
            fn prop_all_value_objects_validate(
                dc in prop_oneof![Just(DebitCredit::Debit), Just(DebitCredit::Credit)],
                tax_type in prop_oneof![
                    Just(TaxType::Taxable),
                    Just(TaxType::NonTaxable),
                    Just(TaxType::TaxExempt),
                    Just(TaxType::OutOfScope),
                ],
                entry_type in prop_oneof![
                    Just(JournalEntryType::NewEntry),
                    Just(JournalEntryType::Cancellation),
                    Just(JournalEntryType::Reversal),
                    Just(JournalEntryType::Additional),
                    Just(JournalEntryType::Reclassification),
                    Just(JournalEntryType::Replacement),
                ]
            ) {
                prop_assert!(dc.validate().is_ok());
                prop_assert!(tax_type.validate().is_ok());
                prop_assert!(entry_type.validate().is_ok());
            }
        }
    }
}
