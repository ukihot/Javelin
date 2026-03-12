// JournalEntryLine Entity - 仕訳明細エンティティ

use crate::{
    chart_of_accounts::values::AccountCode,
    common::Money,
    error::DomainResult,
    journal_entry::values::{
        DebitCredit, DepartmentCode, Description, LineNumber, SubAccountCode, TaxType,
    },
    value_object::ValueObject,
};

/// 仕訳明細
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntryLine {
    line_number: LineNumber,
    side: DebitCredit,
    account_code: AccountCode,
    sub_account_code: Option<SubAccountCode>,
    department_code: Option<DepartmentCode>,
    amount: Money,
    tax_type: TaxType,
    tax_amount: Money,
    description: Option<Description>,
}

impl JournalEntryLine {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        line_number: LineNumber,
        side: DebitCredit,
        account_code: AccountCode,
        sub_account_code: Option<SubAccountCode>,
        department_code: Option<DepartmentCode>,
        amount: Money,
        tax_type: TaxType,
        tax_amount: Money,
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

    pub fn line_number(&self) -> &LineNumber {
        &self.line_number
    }

    pub fn side(&self) -> &DebitCredit {
        &self.side
    }

    pub fn account_code(&self) -> &AccountCode {
        &self.account_code
    }

    pub fn sub_account_code(&self) -> Option<&SubAccountCode> {
        self.sub_account_code.as_ref()
    }

    pub fn department_code(&self) -> Option<&DepartmentCode> {
        self.department_code.as_ref()
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn tax_type(&self) -> &TaxType {
        &self.tax_type
    }

    pub fn tax_amount(&self) -> &Money {
        &self.tax_amount
    }

    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }

    pub fn is_debit(&self) -> bool {
        matches!(self.side, DebitCredit::Debit)
    }

    pub fn is_credit(&self) -> bool {
        matches!(self.side, DebitCredit::Credit)
    }
}

impl ValueObject for JournalEntryLine {
    fn validate(&self) -> DomainResult<()> {
        // Money は常に有効な金額を保持するため、個別の検証は不要
        // 金額が正であることを確認
        if !self.amount.is_positive() {
            return Err(crate::error::DomainError::InvalidAmount(
                "Journal entry line amount must be positive".to_string(),
            ));
        }

        if self.amount.currency() != self.tax_amount.currency() {
            return Err(crate::error::DomainError::InvalidAmount(
                "Amount and tax amount must have the same currency".to_string(),
            ));
        }
        Ok(())
    }
}
