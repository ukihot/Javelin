// ApplyIfrsValuationInteractor JudgmentLog統合テスト

#[cfg(test)]
mod tests {
    use crate::dtos::{ApplyIfrsValuationRequest, request::JudgmentLogEntry};

    #[test]
    fn test_judgment_log_entry_ecl() {
        let entry = JudgmentLogEntry::new(
            "IFRS 9 Expected Credit Loss".to_string(),
            "Aging-based ECL with weighted probability".to_string(),
            vec![
                "Current (0-30d): 0.5% loss rate".to_string(),
                "30-60d: 2.0% loss rate".to_string(),
                "60-90d: 5.0% loss rate".to_string(),
            ],
            vec!["If loss rates +10%: allowance increases by 10%".to_string()],
            "user1".to_string(),
        );

        assert_eq!(entry.accounting_standard, "IFRS 9 Expected Credit Loss");
        assert_eq!(entry.assumptions.len(), 3);
        assert_eq!(entry.sensitivity_analysis.len(), 1);
    }

    #[test]
    fn test_judgment_log_entry_impairment() {
        let entry = JudgmentLogEntry::new(
            "IAS 36 Impairment of Assets".to_string(),
            "DCF using Discounted Cash Flow method".to_string(),
            vec!["Market price declined 35%".to_string(), "Discount rate: 5%".to_string()],
            vec!["If discount rate = 4%: Recoverable = 450k".to_string()],
            "user2".to_string(),
        );

        assert_eq!(entry.accounting_standard, "IAS 36 Impairment of Assets");
        assert_eq!(entry.model_used, "DCF using Discounted Cash Flow method");
        assert_eq!(entry.responsible_party, "user2");
    }

    #[test]
    fn test_apply_ifrs_valuation_request_with_user_id() {
        let request = ApplyIfrsValuationRequest {
            fiscal_year: 2024,
            period: 1,
            user_id: "user123".to_string(),
        };

        assert_eq!(request.fiscal_year, 2024);
        assert_eq!(request.period, 1);
        assert_eq!(request.user_id, "user123");
    }
}
