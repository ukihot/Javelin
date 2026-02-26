// Domain層ユニットテスト: JournalEntryLine
// 正常系・異常系・値オブジェクト検証

#[cfg(test)]
mod tests {
    use crate::financial_close::{
        AccountCode,
        journal_entry::{Amount, Currency, DebitCredit, JournalEntryLine, LineNumber, TaxType},
    };

    #[test]
    fn test_journal_entry_line_valid() {
        let line = JournalEntryLine::new(
            LineNumber::new(1).unwrap(),
            DebitCredit::Debit,
            AccountCode::new("1010".to_string()).unwrap(),
            None,
            None,
            Amount::new(100000.0, Currency::JPY).unwrap(),
            TaxType::NonTaxable,
            Amount::new(0.0, Currency::JPY).unwrap(),
            None,
        );
        assert!(line.is_ok());
    }

    #[test]
    fn test_journal_entry_line_invalid_amount() {
        // negative amount should be rejected by Amount value object
        let amt = Amount::new(-100000.0, Currency::JPY);
        assert!(amt.is_err(), "Amount creation should fail for negative values");
    }
}
