// 識別子関連の値オブジェクト

use crate::{
    entity::EntityId,
    error::{DomainError, DomainResult},
    value_object::ValueObject,
};

/// 伝票番号
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryNumber(String);

impl ValueObject for EntryNumber {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount("Entry number cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl EntryNumber {
    pub fn new(number: String) -> DomainResult<Self> {
        let entry_number = Self(number);
        entry_number.validate()?;
        Ok(entry_number)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 証憑番号
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoucherNumber(String);

impl ValueObject for VoucherNumber {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(DomainError::InvalidAmount("Voucher number cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl VoucherNumber {
    pub fn new(number: String) -> DomainResult<Self> {
        let voucher_number = Self(number);
        voucher_number.validate()?;
        Ok(voucher_number)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// 行番号
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineNumber(u32);

impl ValueObject for LineNumber {
    fn validate(&self) -> DomainResult<()> {
        if self.0 == 0 {
            return Err(DomainError::InvalidAmount(
                "Line number must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

impl LineNumber {
    pub fn new(number: u32) -> DomainResult<Self> {
        let line_number = Self(number);
        line_number.validate()?;
        Ok(line_number)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

/// ユーザーID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(String);

impl EntityId for UserId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl UserId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_number() {
        let number = EntryNumber::new("JE-2024-001".to_string());
        assert!(number.is_ok());
        assert_eq!(number.unwrap().value(), "JE-2024-001");

        let empty = EntryNumber::new("".to_string());
        assert!(empty.is_err());
    }

    #[test]
    fn test_line_number() {
        let line = LineNumber::new(1);
        assert!(line.is_ok());
        assert_eq!(line.unwrap().value(), 1);

        let zero = LineNumber::new(0);
        assert!(zero.is_err());
    }

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            // プロパティ1: 非空文字列の伝票番号は常に作成可能
            #[test]
            fn prop_valid_entry_number(number in "[a-zA-Z0-9-]{1,50}") {
                let entry_number = EntryNumber::new(number.clone());
                prop_assert!(entry_number.is_ok());
                let entry_number = entry_number.unwrap();
                prop_assert_eq!(entry_number.value(), number.as_str());
            }

            // プロパティ2: 空文字列の伝票番号は常にエラー
            #[test]
            fn prop_empty_entry_number_fails(_unit in Just(())) {
                let entry_number = EntryNumber::new("".to_string());
                prop_assert!(entry_number.is_err());
            }

            // プロパティ3: 非空文字列の証憑番号は常に作成可能
            #[test]
            fn prop_valid_voucher_number(number in "[a-zA-Z0-9-]{1,50}") {
                let voucher_number = VoucherNumber::new(number.clone());
                prop_assert!(voucher_number.is_ok());
                let voucher_number = voucher_number.unwrap();
                prop_assert_eq!(voucher_number.value(), number.as_str());
            }

            // プロパティ4: 空文字列の証憑番号は常にエラー
            #[test]
            fn prop_empty_voucher_number_fails(_unit in Just(())) {
                let voucher_number = VoucherNumber::new("".to_string());
                prop_assert!(voucher_number.is_err());
            }

            // プロパティ5: 正の整数の行番号は常に作成可能
            #[test]
            fn prop_valid_line_number(number in 1u32..=1000u32) {
                let line_number = LineNumber::new(number);
                prop_assert!(line_number.is_ok());
                prop_assert_eq!(line_number.unwrap().value(), number);
            }

            // プロパティ6: ゼロの行番号は常にエラー
            #[test]
            fn prop_zero_line_number_fails(_unit in Just(())) {
                let line_number = LineNumber::new(0);
                prop_assert!(line_number.is_err());
            }

            // プロパティ7: 行番号の順序性
            #[test]
            fn prop_line_number_ordering(n1 in 1u32..=1000u32, n2 in 1u32..=1000u32) {
                let line1 = LineNumber::new(n1).unwrap();
                let line2 = LineNumber::new(n2).unwrap();

                if n1 < n2 {
                    prop_assert!(line1 < line2);
                } else if n1 > n2 {
                    prop_assert!(line1 > line2);
                } else {
                    prop_assert_eq!(line1, line2);
                }
            }

            // プロパティ8: ユーザーIDは任意の文字列で作成可能
            #[test]
            fn prop_user_id_creation(id in "[a-zA-Z0-9-_]{1,50}") {
                let user_id = UserId::new(id.clone());
                prop_assert_eq!(user_id.value(), id.as_str());
            }
        }
    }
}
