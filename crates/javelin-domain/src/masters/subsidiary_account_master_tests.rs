// Domain層ユニットテスト: SubsidiaryAccountMaster
// 正常系・異常系・値オブジェクト検証

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsidiary_account_code_valid() {
        let code = SubsidiaryAccountCode::new("S001".to_string());
        assert!(code.is_ok());
    }

    #[test]
    fn test_subsidiary_account_code_invalid() {
        let code = SubsidiaryAccountCode::new("".to_string());
        assert!(code.is_err());
    }

    #[test]
    fn test_subsidiary_account_master_creation() {
        let code = SubsidiaryAccountCode::new("S001".to_string()).unwrap();
        let master = SubsidiaryAccountMaster::new(code, "補助科目".to_string());
        assert_eq!(master.name, "補助科目");
    }
}
