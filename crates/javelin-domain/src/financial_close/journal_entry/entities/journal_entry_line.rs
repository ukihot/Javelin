// 仕訳明細エンティティ

use super::super::values::{
    Amount, DebitCredit, DepartmentCode, Description, LineNumber, SubAccountCode, TaxType,
};
use crate::{error::DomainResult, financial_close::AccountCode, value_object::ValueObject};

/// 仕訳明細
///
/// 仕訳伝票の1行を表すエンティティ。
/// 借方または貸方の勘定科目、金額、税情報、摘要を保持する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntryLine {
    /// 行番号
    line_number: LineNumber,
    /// 借方/貸方区分
    side: DebitCredit,
    /// 勘定科目コード
    account_code: AccountCode,
    /// 補助科目コード（オプション）
    sub_account_code: Option<SubAccountCode>,
    /// 部門コード（オプション）
    department_code: Option<DepartmentCode>,
    /// 金額
    amount: Amount,
    /// 税区分
    tax_type: TaxType,
    /// 税額
    tax_amount: Amount,
    /// 摘要（オプション）
    description: Option<Description>,
}

/// 仕訳明細ビルダー
pub struct JournalEntryLineBuilder {
    line_number: LineNumber,
    side: DebitCredit,
    account_code: AccountCode,
    sub_account_code: Option<SubAccountCode>,
    department_code: Option<DepartmentCode>,
    amount: Amount,
    tax_type: TaxType,
    tax_amount: Amount,
    description: Option<Description>,
}

impl JournalEntryLineBuilder {
    pub fn new(
        line_number: LineNumber,
        side: DebitCredit,
        account_code: AccountCode,
        amount: Amount,
    ) -> Self {
        let currency = amount.currency().clone();
        Self {
            line_number,
            side,
            account_code,
            sub_account_code: None,
            department_code: None,
            amount,
            tax_type: TaxType::NonTaxable,
            tax_amount: Amount::zero(currency),
            description: None,
        }
    }

    pub fn sub_account_code(mut self, code: SubAccountCode) -> Self {
        self.sub_account_code = Some(code);
        self
    }

    pub fn department_code(mut self, code: DepartmentCode) -> Self {
        self.department_code = Some(code);
        self
    }

    pub fn tax(mut self, tax_type: TaxType, tax_amount: Amount) -> Self {
        self.tax_type = tax_type;
        self.tax_amount = tax_amount;
        self
    }

    pub fn description(mut self, description: Description) -> Self {
        self.description = Some(description);
        self
    }

    pub fn build(self) -> DomainResult<JournalEntryLine> {
        JournalEntryLine::new(
            self.line_number,
            self.side,
            self.account_code,
            self.sub_account_code,
            self.department_code,
            self.amount,
            self.tax_type,
            self.tax_amount,
            self.description,
        )
    }
}

impl JournalEntryLine {
    /// 新しい仕訳明細を作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        line_number: LineNumber,
        side: DebitCredit,
        account_code: AccountCode,
        sub_account_code: Option<SubAccountCode>,
        department_code: Option<DepartmentCode>,
        amount: Amount,
        tax_type: TaxType,
        tax_amount: Amount,
        description: Option<Description>,
    ) -> DomainResult<Self> {
        let line = Self {
            line_number,
            side,
            account_code,
            sub_account_code,
            department_code,
            amount,
            tax_type,
            tax_amount,
            description,
        };
        line.validate()?;
        Ok(line)
    }

    /// ビルダーを作成
    pub fn builder(
        line_number: LineNumber,
        side: DebitCredit,
        account_code: AccountCode,
        amount: Amount,
    ) -> JournalEntryLineBuilder {
        JournalEntryLineBuilder::new(line_number, side, account_code, amount)
    }

    /// 行番号を取得
    pub fn line_number(&self) -> &LineNumber {
        &self.line_number
    }

    /// 借方/貸方区分を取得
    pub fn side(&self) -> &DebitCredit {
        &self.side
    }

    /// 勘定科目コードを取得
    pub fn account_code(&self) -> &AccountCode {
        &self.account_code
    }

    /// 補助科目コードを取得
    pub fn sub_account_code(&self) -> Option<&SubAccountCode> {
        self.sub_account_code.as_ref()
    }

    /// 部門コードを取得
    pub fn department_code(&self) -> Option<&DepartmentCode> {
        self.department_code.as_ref()
    }

    /// 金額を取得
    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    /// 税区分を取得
    pub fn tax_type(&self) -> &TaxType {
        &self.tax_type
    }

    /// 税額を取得
    pub fn tax_amount(&self) -> &Amount {
        &self.tax_amount
    }

    /// 摘要を取得
    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }

    /// 借方かどうかを判定
    pub fn is_debit(&self) -> bool {
        matches!(self.side, DebitCredit::Debit)
    }

    /// 貸方かどうかを判定
    pub fn is_credit(&self) -> bool {
        matches!(self.side, DebitCredit::Credit)
    }
}

impl ValueObject for JournalEntryLine {
    fn validate(&self) -> DomainResult<()> {
        // 会計の鉄則: 仕訳明細行の金額は必ず正の値（非ゼロ、非負）
        self.amount.validate_as_journal_entry_line_amount()?;

        // 金額と税額の通貨が一致していることを確認
        if self.amount.currency() != self.tax_amount.currency() {
            return Err(crate::error::DomainError::InvalidAmount(
                "Amount and tax amount must have the same currency".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::financial_close::journal_entry::values::Currency;

    #[test]
    fn test_journal_entry_line_creation() {
        let line_number = LineNumber::new(1).unwrap();
        let side = DebitCredit::Debit;
        let account_code = AccountCode::new("1000".to_string()).unwrap();
        let amount = Amount::new(100000.0, Currency::JPY).unwrap();
        let tax_amount = Amount::new(10000.0, Currency::JPY).unwrap();

        let line = JournalEntryLine::new(
            line_number,
            side,
            account_code,
            None,
            None,
            amount,
            TaxType::Taxable,
            tax_amount,
            None,
        );

        assert!(line.is_ok());
        let line = line.unwrap();
        assert_eq!(line.line_number().value(), 1);
        assert!(line.is_debit());
        assert!(!line.is_credit());
        assert_eq!(line.amount().value(), 100000.0);
        assert_eq!(line.tax_amount().value(), 10000.0);
    }

    #[test]
    fn test_journal_entry_line_with_sub_account() {
        let line_number = LineNumber::new(1).unwrap();
        let side = DebitCredit::Credit;
        let account_code = AccountCode::new("2000".to_string()).unwrap();
        let sub_account_code = Some(SubAccountCode::new("2001".to_string()).unwrap());
        let amount = Amount::new(50000.0, Currency::JPY).unwrap();
        let tax_amount = Amount::zero(Currency::JPY);

        let line = JournalEntryLine::new(
            line_number,
            side,
            account_code,
            sub_account_code,
            None,
            amount,
            TaxType::NonTaxable,
            tax_amount,
            None,
        );

        assert!(line.is_ok());
        let line = line.unwrap();
        assert!(line.sub_account_code().is_some());
        assert!(line.is_credit());
        assert!(!line.is_debit());
    }

    #[test]
    fn test_journal_entry_line_with_department() {
        let line_number = LineNumber::new(2).unwrap();
        let side = DebitCredit::Debit;
        let account_code = AccountCode::new("3000".to_string()).unwrap();
        let department_code = Some(DepartmentCode::new("DEPT01".to_string()).unwrap());
        let amount = Amount::new(75000.0, Currency::JPY).unwrap();
        let tax_amount = Amount::zero(Currency::JPY);

        let line = JournalEntryLine::new(
            line_number,
            side,
            account_code,
            None,
            department_code,
            amount,
            TaxType::OutOfScope,
            tax_amount,
            None,
        );

        assert!(line.is_ok());
        let line = line.unwrap();
        assert!(line.department_code().is_some());
    }

    #[test]
    fn test_journal_entry_line_currency_mismatch() {
        let line_number = LineNumber::new(1).unwrap();
        let side = DebitCredit::Debit;
        let account_code = AccountCode::new("1000".to_string()).unwrap();
        let amount = Amount::new(100000.0, Currency::JPY).unwrap();
        let tax_amount = Amount::new(10000.0, Currency::USD).unwrap(); // 異なる通貨

        let line = JournalEntryLine::new(
            line_number,
            side,
            account_code,
            None,
            None,
            amount,
            TaxType::Taxable,
            tax_amount,
            None,
        );

        assert!(line.is_err());
    }

    #[test]
    fn test_journal_entry_line_zero_amount_rejected() {
        // 会計の鉄則: 仕訳明細行の金額はゼロであってはならない
        let line_number = LineNumber::new(1).unwrap();
        let side = DebitCredit::Debit;
        let account_code = AccountCode::new("1000".to_string()).unwrap();
        let zero_amount = Amount::zero(Currency::JPY);
        let tax_amount = Amount::zero(Currency::JPY);

        let line = JournalEntryLine::new(
            line_number,
            side,
            account_code,
            None,
            None,
            zero_amount,
            TaxType::NonTaxable,
            tax_amount,
            None,
        );

        assert!(line.is_err());
    }

    #[test]
    fn test_journal_entry_line_minimum_positive_amount() {
        // 最小の正の金額（0.01）は有効
        let line_number = LineNumber::new(1).unwrap();
        let side = DebitCredit::Debit;
        let account_code = AccountCode::new("1000".to_string()).unwrap();
        let min_amount = Amount::new(0.01, Currency::JPY).unwrap();
        let tax_amount = Amount::zero(Currency::JPY);

        let line = JournalEntryLine::new(
            line_number,
            side,
            account_code,
            None,
            None,
            min_amount,
            TaxType::NonTaxable,
            tax_amount,
            None,
        );

        assert!(line.is_ok());
    }

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        // 有効な金額生成戦略（正の値のみ: 0.01以上）
        fn positive_amount_strategy() -> impl Strategy<Value = f64> {
            (1i64..=100_000_000_i64).prop_map(|cents| cents as f64 / 100.0)
        }

        proptest! {
            // プロパティ1: 正の金額を持つ仕訳明細行は常に作成可能
            #[test]
            fn prop_positive_amount_line_creation(
                line_num in 1u32..=9999u32,
                amount_value in positive_amount_strategy(),
            ) {
                let line_number = LineNumber::new(line_num).unwrap();
                let side = DebitCredit::Debit;
                let account_code = AccountCode::new("1000".to_string()).unwrap();
                let amount = Amount::new(amount_value, Currency::JPY).unwrap();
                let tax_amount = Amount::zero(Currency::JPY);

                let line = JournalEntryLine::new(
                    line_number,
                    side,
                    account_code,
                    None,
                    None,
                    amount,
                    TaxType::NonTaxable,
                    tax_amount,
                    None,
                );

                prop_assert!(line.is_ok());
            }

            // プロパティ2: ゼロ金額の仕訳明細行は常に作成失敗
            #[test]
            fn prop_zero_amount_line_creation_fails(
                line_num in 1u32..=9999u32,
            ) {
                let line_number = LineNumber::new(line_num).unwrap();
                let side = DebitCredit::Credit;
                let account_code = AccountCode::new("2000".to_string()).unwrap();
                let zero_amount = Amount::zero(Currency::JPY);
                let tax_amount = Amount::zero(Currency::JPY);

                let line = JournalEntryLine::new(
                    line_number,
                    side,
                    account_code,
                    None,
                    None,
                    zero_amount,
                    TaxType::NonTaxable,
                    tax_amount,
                    None,
                );

                prop_assert!(line.is_err());
            }

            // プロパティ3: 借方・貸方どちらも正の金額が必要
            #[test]
            fn prop_both_sides_require_positive_amount(
                line_num in 1u32..=9999u32,
                amount_value in positive_amount_strategy(),
                is_debit in prop::bool::ANY,
            ) {
                let line_number = LineNumber::new(line_num).unwrap();
                let side = if is_debit { DebitCredit::Debit } else { DebitCredit::Credit };
                let account_code = AccountCode::new("1000".to_string()).unwrap();
                let amount = Amount::new(amount_value, Currency::JPY).unwrap();
                let tax_amount = Amount::zero(Currency::JPY);

                let line = JournalEntryLine::new(
                    line_number,
                    side,
                    account_code,
                    None,
                    None,
                    amount,
                    TaxType::NonTaxable,
                    tax_amount,
                    None,
                );

                prop_assert!(line.is_ok());
                let line = line.unwrap();
                prop_assert!(line.amount().value() > 0.0);
            }
        }
    }
}
