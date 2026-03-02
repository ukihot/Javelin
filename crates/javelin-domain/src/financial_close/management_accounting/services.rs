// 管理会計のドメインサービス

use crate::{common::Amount, error::{DomainError, DomainResult}};

use super::{
    entities::{BusinessConditionReport, DepartmentMargin, ManagementAccountingConversion},
    values::ConversionType,
};

/// 管理会計ドメインサービス
pub struct ManagementAccountingService;

impl ManagementAccountingService {
    /// 制度会計から管理会計への変換
    pub fn convert_to_management_accounting(
        financial_accounts: &[(String, Amount)],
        conversion_type: ConversionType,
    ) -> DomainResult<Vec<(String, Amount)>> {
        if financial_accounts.is_empty() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        let mut result = financial_accounts.to_vec();

        match conversion_type {
            ConversionType::FixedCostReclassification => {
                // 固定費再分類ロジック
                for (account, _amount) in result.iter_mut() {
                    if account.starts_with("6") {
                        // 費用勘定を固定費として扱う
                        *account = format!("FIXED_{}", account);
                    }
                }
            }
            ConversionType::VariableCostIdentification => {
                // 変動費識別ロジック
                for (account, _amount) in result.iter_mut() {
                    if account.starts_with("5") {
                        // 売上原価を変動費として扱う
                        *account = format!("VARIABLE_{}", account);
                    }
                }
            }
            _ => {}
        }

        Ok(result)
    }

    /// 限界利益計算
    pub fn calculate_contribution_margin(sales: &Amount, variable_costs: &Amount) -> DomainResult<Amount> {
        if sales.is_negative() || variable_costs.is_negative() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        Ok(sales - variable_costs)
    }

    /// 損益分岐点売上高計算
    pub fn calculate_break_even_sales(
        fixed_costs: &Amount,
        contribution_margin: &Amount,
        sales: &Amount,
    ) -> DomainResult<Amount> {
        if sales.is_negative() || fixed_costs.is_negative() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        if contribution_margin.is_zero() {
            return Ok(Amount::zero());
        }

        // (fixed_costs * sales) / contribution_margin
        let numerator = fixed_costs.value() * sales.value();
        let break_even = numerator / contribution_margin.value();
        Ok(Amount::from(break_even))
    }

    /// 安全余裕率計算
    pub fn calculate_safety_margin_rate(sales: &Amount, break_even_sales: &Amount) -> DomainResult<f64> {
        if sales.is_negative() || break_even_sales.is_negative() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        if sales.is_zero() {
            return Ok(0.0);
        }

        let diff = sales.value() - break_even_sales.value();
        let rate = (diff / sales.value()).to_f64().unwrap_or(0.0) * 100.0;
        Ok(rate)
    }

    /// 共通費配賦
    pub fn allocate_common_costs(
        common_costs: &Amount,
        allocation_ratios: &[(String, f64)],
    ) -> DomainResult<Vec<(String, Amount)>> {
        if common_costs.is_negative() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        if allocation_ratios.is_empty() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        let total_ratio: f64 = allocation_ratios.iter().map(|(_, ratio)| ratio).sum();
        if (total_ratio - 1.0).abs() > 0.01 {
            return Err(DomainError::InvalidManagementAccounting);
        }

        let mut result = Vec::new();
        for (department, ratio) in allocation_ratios {
            let allocated = common_costs.value() * ratio;
            result.push((department.clone(), Amount::from(allocated)));
        }

        Ok(result)
    }

    /// 残高一致検証
    pub fn verify_balance_consistency(
        financial_total: &Amount,
        management_total: &Amount,
    ) -> DomainResult<()> {
        if financial_total != management_total {
            return Err(DomainError::InvalidManagementAccounting);
        }

        Ok(())
    }

    /// 部門別ROI計算
    pub fn calculate_department_roi(
        department_profit: &Amount,
        department_assets: &Amount,
    ) -> DomainResult<f64> {
        if department_assets.is_zero() || department_assets.is_negative() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        let roi = (department_profit.value() / department_assets.value()).to_f64().unwrap_or(0.0) * 100.0;
        Ok(roi)
    }

    /// 変換ロジック検証
    pub fn verify_conversion_logic(
        conversion: &ManagementAccountingConversion,
    ) -> DomainResult<()> {
        if !conversion.is_approved() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        if conversion.target_accounts().is_empty() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        Ok(())
    }

    /// 業況表整合性検証
    pub fn verify_report_consistency(report: &BusinessConditionReport) -> DomainResult<()> {
        // 限界利益 = 売上高 - 変動費
        let expected_operating_profit = report.contribution_margin() - report.total_fixed_costs();
        if expected_operating_profit != *report.operating_profit() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        // 部門別限界利益の合計検証
        let total_dept_margin: Amount = report
            .department_margins()
            .iter()
            .fold(Amount::zero(), |acc, d| &acc + d.contribution_margin());

        // 部門別限界利益の合計は全社限界利益と一致すべき（共通費配賦前）
        // ただし、部門に配賦されない共通費がある場合は差異が生じる
        if !report.department_margins().is_empty() && total_dept_margin > *report.contribution_margin() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_management_accounting_fixed_cost() {
        let accounts = vec![
            ("6000".to_string(), Amount::from_i64(1_000_000)),
            ("6100".to_string(), Amount::from_i64(500_000)),
        ];

        let result = ManagementAccountingService::convert_to_management_accounting(
            &accounts,
            ConversionType::FixedCostReclassification,
        )
        .unwrap();

        assert_eq!(result[0].0, "FIXED_6000");
        assert_eq!(result[1].0, "FIXED_6100");
    }

    #[test]
    fn test_convert_to_management_accounting_variable_cost() {
        let accounts = vec![
            ("5000".to_string(), Amount::from_i64(3_000_000)),
            ("5100".to_string(), Amount::from_i64(2_000_000)),
        ];

        let result = ManagementAccountingService::convert_to_management_accounting(
            &accounts,
            ConversionType::VariableCostIdentification,
        )
        .unwrap();

        assert_eq!(result[0].0, "VARIABLE_5000");
        assert_eq!(result[1].0, "VARIABLE_5100");
    }

    #[test]
    fn test_convert_empty_accounts() {
        let result = ManagementAccountingService::convert_to_management_accounting(
            &[],
            ConversionType::FixedCostReclassification,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_contribution_margin() {
        let margin = ManagementAccountingService::calculate_contribution_margin(
            &Amount::from_i64(10_000_000),
            &Amount::from_i64(6_000_000),
        )
        .unwrap();

        assert_eq!(margin.to_i64(), Some(4_000_000));
    }

    #[test]
    fn test_calculate_contribution_margin_invalid() {
        let result = ManagementAccountingService::calculate_contribution_margin(
            &Amount::from_i64(-1),
            &Amount::zero(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_break_even_sales() {
        let break_even = ManagementAccountingService::calculate_break_even_sales(
            &Amount::from_i64(3_000_000),
            &Amount::from_i64(6_000_000),
            &Amount::from_i64(10_000_000),
        )
        .unwrap();

        assert_eq!(break_even.to_i64(), Some(5_000_000));
    }

    #[test]
    fn test_calculate_break_even_sales_zero_margin() {
        let break_even = ManagementAccountingService::calculate_break_even_sales(
            &Amount::from_i64(3_000_000),
            &Amount::zero(),
            &Amount::from_i64(10_000_000),
        )
        .unwrap();

        assert_eq!(break_even.to_i64(), Some(0));
    }

    #[test]
    fn test_calculate_safety_margin_rate() {
        let rate = ManagementAccountingService::calculate_safety_margin_rate(
            &Amount::from_i64(10_000_000),
            &Amount::from_i64(5_000_000),
        )
        .unwrap();

        assert_eq!(rate, 50.0);
    }

    #[test]
    fn test_calculate_safety_margin_rate_zero_sales() {
        let rate = ManagementAccountingService::calculate_safety_margin_rate(
            &Amount::zero(),
            &Amount::zero(),
        )
        .unwrap();

        assert_eq!(rate, 0.0);
    }

    #[test]
    fn test_allocate_common_costs() {
        let ratios = vec![("D001".to_string(), 0.6), ("D002".to_string(), 0.4)];

        let result = ManagementAccountingService::allocate_common_costs(
            &Amount::from_i64(1_000_000),
            &ratios,
        )
        .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].1.to_i64(), Some(600_000));
        assert_eq!(result[1].1.to_i64(), Some(400_000));
    }

    #[test]
    fn test_allocate_common_costs_invalid_ratio() {
        let ratios = vec![("D001".to_string(), 0.6), ("D002".to_string(), 0.3)];

        let result =
            ManagementAccountingService::allocate_common_costs(&Amount::from_i64(1_000_000), &ratios);

        assert!(result.is_err());
    }

    #[test]
    fn test_verify_balance_consistency() {
        let result = ManagementAccountingService::verify_balance_consistency(
            &Amount::from_i64(10_000_000),
            &Amount::from_i64(10_000_000),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_balance_consistency_mismatch() {
        let result = ManagementAccountingService::verify_balance_consistency(
            &Amount::from_i64(10_000_000),
            &Amount::from_i64(9_000_000),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_department_roi() {
        let roi = ManagementAccountingService::calculate_department_roi(
            &Amount::from_i64(2_000_000),
            &Amount::from_i64(10_000_000),
        )
        .unwrap();

        assert_eq!(roi, 20.0);
    }

    #[test]
    fn test_calculate_department_roi_zero_assets() {
        let result = ManagementAccountingService::calculate_department_roi(
            &Amount::from_i64(2_000_000),
            &Amount::zero(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_verify_conversion_logic() {
        let mut conversion = ManagementAccountingConversion::new(
            ConversionType::FixedCostReclassification,
            vec!["6000".to_string()],
            Amount::from_i64(5_000_000),
            Amount::from_i64(5_000_000),
            None,
            "固定費再分類".to_string(),
            "2024-01".to_string(),
        )
        .unwrap();

        conversion.approve("CFO".to_string()).unwrap();

        let result = ManagementAccountingService::verify_conversion_logic(&conversion);

        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_conversion_logic_not_approved() {
        let conversion = ManagementAccountingConversion::new(
            ConversionType::FixedCostReclassification,
            vec!["6000".to_string()],
            Amount::from_i64(5_000_000),
            Amount::from_i64(5_000_000),
            None,
            "固定費再分類".to_string(),
            "2024-01".to_string(),
        )
        .unwrap();

        let result = ManagementAccountingService::verify_conversion_logic(&conversion);

        assert!(result.is_err());
    }

    #[test]
    fn test_verify_report_consistency() {
        let report = BusinessConditionReport::new(
            "2024-01".to_string(),
            Amount::from_i64(10_000_000),
            Amount::from_i64(4_000_000),
            Amount::from_i64(6_000_000),
            Amount::from_i64(3_000_000),
            Amount::from_i64(3_000_000),
            vec![],
            Amount::zero(),
            Amount::from_i64(1_000_000),
        )
        .unwrap();

        let result = ManagementAccountingService::verify_report_consistency(&report);

        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_report_consistency_invalid() {
        let report = BusinessConditionReport::new(
            "2024-01".to_string(),
            Amount::from_i64(10_000_000),
            Amount::from_i64(4_000_000),
            Amount::from_i64(6_000_000),
            Amount::from_i64(3_000_000),
            Amount::from_i64(2_000_000), // 不整合: 6,000,000 - 3,000,000 = 3,000,000 であるべき
            vec![],
            Amount::zero(),
            Amount::from_i64(1_000_000),
        )
        .unwrap();

        let result = ManagementAccountingService::verify_report_consistency(&report);

        assert!(result.is_err());
    }
}
