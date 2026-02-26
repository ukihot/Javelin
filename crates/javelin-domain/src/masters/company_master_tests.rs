// Domain層ユニットテスト: CompanyMaster
// 正常系・異常系・値オブジェクト検証

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_code_valid() {
        let code = CompanyCode::new("C001".to_string());
        assert!(code.is_ok());
    }

    #[test]
    fn test_company_code_invalid() {
        let code = CompanyCode::new("".to_string());
        assert!(code.is_err());
    }

    #[test]
    fn test_company_master_creation() {
        let code = CompanyCode::new("C001".to_string()).unwrap();
        let master = CompanyMaster::new(code, "テスト会社".to_string());
        assert_eq!(master.name, "テスト会社");
    }
}
