// Domain層ユニットテスト: AccountingPeriod
// 正常系・異常系・値オブジェクト検証

#[cfg(test)]
mod tests {
    use crate::financial_close::AccountingPeriod;

    #[test]
    fn test_period_valid() {
        let period = AccountingPeriod::new(2024, 1);
        assert!(period.is_ok());
    }

    #[test]
    fn test_period_invalid_month() {
        let period = AccountingPeriod::new(2024, 13);
        assert!(period.is_err());
    }
}
