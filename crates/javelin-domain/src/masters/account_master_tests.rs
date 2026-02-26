// Domain層ユニットテスト: AccountMaster
// 正常系・異常系・値オブジェクト検証

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_code_valid() {
        let code = AccountCode::new("1010".to_string());
        assert!(code.is_ok());
    }

    #[test]
    fn test_account_code_invalid() {
        let code = AccountCode::new("".to_string());
        assert!(code.is_err());
    }

    #[test]
    fn test_account_master_creation() {
        let code = AccountCode::new("1010".to_string()).unwrap();
        let master = AccountMaster::new(code, "現金".to_string());
        assert_eq!(master.name, "現金");
    }
}
