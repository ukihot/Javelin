// Domain層ユニットテスト: Amount, Currency, TaxType
// 正常系・異常系・値オブジェクト検証

#[cfg(test)]
mod tests {
    use crate::financial_close::journal_entry::{Amount, Currency, TaxType};

    #[test]
    fn test_amount_valid() {
        let amount = Amount::new(1000.0, Currency::JPY);
        assert!(amount.is_ok());
    }

    #[test]
    fn test_amount_invalid_negative() {
        let amount = Amount::new(-1000.0, Currency::JPY);
        assert!(amount.is_err());
    }

    #[test]
    fn test_currency_parse() {
        let currency = "JPY".parse::<Currency>();
        assert!(currency.is_ok());
    }

    #[test]
    fn test_tax_type_parse() {
        let tax_type = "NonTaxable".parse::<TaxType>();
        assert!(tax_type.is_ok());
    }
}
