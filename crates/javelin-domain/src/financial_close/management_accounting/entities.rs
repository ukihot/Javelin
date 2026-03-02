// 管理会計のエンティティ

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    common::Amount,
    entity::{Entity, EntityId},
    error::{DomainError, DomainResult},
};

use super::values::{ConversionLogicId, ConversionType};

/// 業況表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessConditionReport {
    /// 会計期間
    period: String,
    /// 売上高
    sales: Amount,
    /// 売上総利益
    gross_profit: Amount,
    /// 限界利益
    contribution_margin: Amount,
    /// 固定費合計
    total_fixed_costs: Amount,
    /// 営業利益
    operating_profit: Amount,
    /// 部門別限界利益
    department_margins: Vec<DepartmentMargin>,
    /// 受注残高
    order_backlog: Amount,
    /// キャッシュ残高
    cash_balance: Amount,
    /// 損益分岐点売上高
    break_even_sales: Amount,
    /// 安全余裕率
    safety_margin_rate: f64,
    /// 作成日時
    created_at: DateTime<Utc>,
}

impl BusinessConditionReport {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        period: String,
        sales: Amount,
        gross_profit: Amount,
        contribution_margin: Amount,
        total_fixed_costs: Amount,
        operating_profit: Amount,
        department_margins: Vec<DepartmentMargin>,
        order_backlog: Amount,
        cash_balance: Amount,
    ) -> DomainResult<Self> {
        if sales.is_negative() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        if cash_balance.is_negative() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        let break_even_sales = if !contribution_margin.is_zero() {
            // (total_fixed_costs * sales) / contribution_margin
            let numerator = total_fixed_costs.value() * sales.value();
            let result = numerator / contribution_margin.value();
            Amount::from(result)
        } else {
            Amount::zero()
        };

        let safety_margin_rate = if !sales.is_zero() {
            let diff = sales.value() - break_even_sales.value();
            (diff / sales.value()).to_f64().unwrap_or(0.0) * 100.0
        } else {
            0.0
        };

        Ok(Self {
            period,
            sales,
            gross_profit,
            contribution_margin,
            total_fixed_costs,
            operating_profit,
            department_margins,
            order_backlog,
            cash_balance,
            break_even_sales,
            safety_margin_rate,
            created_at: Utc::now(),
        })
    }

    pub fn period(&self) -> &str {
        &self.period
    }

    pub fn sales(&self) -> &Amount {
        &self.sales
    }

    pub fn gross_profit(&self) -> &Amount {
        &self.gross_profit
    }

    pub fn contribution_margin(&self) -> &Amount {
        &self.contribution_margin
    }

    pub fn total_fixed_costs(&self) -> &Amount {
        &self.total_fixed_costs
    }

    pub fn operating_profit(&self) -> &Amount {
        &self.operating_profit
    }

    pub fn department_margins(&self) -> &[DepartmentMargin] {
        &self.department_margins
    }

    pub fn order_backlog(&self) -> &Amount {
        &self.order_backlog
    }

    pub fn cash_balance(&self) -> &Amount {
        &self.cash_balance
    }

    pub fn break_even_sales(&self) -> &Amount {
        &self.break_even_sales
    }

    pub fn safety_margin_rate(&self) -> f64 {
        self.safety_margin_rate
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// 限界利益率を計算
    pub fn contribution_margin_rate(&self) -> f64 {
        if self.sales.is_zero() {
            0.0
        } else {
            (self.contribution_margin.value() / self.sales.value()).to_f64().unwrap_or(0.0) * 100.0
        }
    }

    /// 営業利益率を計算
    pub fn operating_profit_rate(&self) -> f64 {
        if self.sales.is_zero() {
            0.0
        } else {
            (self.operating_profit.value() / self.sales.value()).to_f64().unwrap_or(0.0) * 100.0
        }
    }
}

/// 部門別限界利益
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentMargin {
    /// 部門コード
    department_code: String,
    /// 部門名
    department_name: String,
    /// 売上高
    sales: Amount,
    /// 変動費
    variable_costs: Amount,
    /// 限界利益
    contribution_margin: Amount,
}

impl DepartmentMargin {
    pub fn new(
        department_code: String,
        department_name: String,
        sales: Amount,
        variable_costs: Amount,
    ) -> DomainResult<Self> {
        if sales.is_negative() || variable_costs.is_negative() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        let contribution_margin = &sales - &variable_costs;

        Ok(Self { department_code, department_name, sales, variable_costs, contribution_margin })
    }

    pub fn department_code(&self) -> &str {
        &self.department_code
    }

    pub fn department_name(&self) -> &str {
        &self.department_name
    }

    pub fn sales(&self) -> &Amount {
        &self.sales
    }

    pub fn variable_costs(&self) -> &Amount {
        &self.variable_costs
    }

    pub fn contribution_margin(&self) -> &Amount {
        &self.contribution_margin
    }

    /// 限界利益率を計算
    pub fn contribution_margin_rate(&self) -> f64 {
        if self.sales.is_zero() {
            0.0
        } else {
            (self.contribution_margin.value() / self.sales.value()).to_f64().unwrap_or(0.0) * 100.0
        }
    }
}

/// 管理会計変換
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagementAccountingConversion {
    /// 変換ロジックID
    id: ConversionLogicId,
    /// 変換タイプ
    conversion_type: ConversionType,
    /// 対象勘定科目
    target_accounts: Vec<String>,
    /// 変換前金額
    amount_before: Amount,
    /// 変換後金額
    amount_after: Amount,
    /// 配賦基準
    allocation_basis: Option<String>,
    /// 変換理由
    reason: String,
    /// 適用期間
    period: String,
    /// 承認者
    approved_by: Option<String>,
    /// 承認日時
    approved_at: Option<DateTime<Utc>>,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// バージョン
    version: u64,
}

impl ManagementAccountingConversion {
    pub fn new(
        conversion_type: ConversionType,
        target_accounts: Vec<String>,
        amount_before: Amount,
        amount_after: Amount,
        allocation_basis: Option<String>,
        reason: String,
        period: String,
    ) -> DomainResult<Self> {
        if target_accounts.is_empty() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        if reason.trim().is_empty() {
            return Err(DomainError::InvalidManagementAccounting);
        }

        Ok(Self {
            id: ConversionLogicId::new(),
            conversion_type,
            target_accounts,
            amount_before,
            amount_after,
            allocation_basis,
            reason,
            period,
            approved_by: None,
            approved_at: None,
            created_at: Utc::now(),
            version: 1,
        })
    }

    pub fn approve(&mut self, approver: String) -> DomainResult<()> {
        if self.approved_by.is_some() {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.approved_by = Some(approver);
        self.approved_at = Some(Utc::now());
        self.version += 1;

        Ok(())
    }

    pub fn id(&self) -> &ConversionLogicId {
        &self.id
    }

    pub fn conversion_type(&self) -> &ConversionType {
        &self.conversion_type
    }

    pub fn target_accounts(&self) -> &[String] {
        &self.target_accounts
    }

    pub fn amount_before(&self) -> &Amount {
        &self.amount_before
    }

    pub fn amount_after(&self) -> &Amount {
        &self.amount_after
    }

    pub fn allocation_basis(&self) -> Option<&str> {
        self.allocation_basis.as_deref()
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn period(&self) -> &str {
        &self.period
    }

    pub fn approved_by(&self) -> Option<&str> {
        self.approved_by.as_deref()
    }

    pub fn approved_at(&self) -> Option<DateTime<Utc>> {
        self.approved_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn is_approved(&self) -> bool {
        self.approved_by.is_some()
    }

    /// 変換差額を計算
    pub fn conversion_difference(&self) -> Amount {
        &self.amount_after - &self.amount_before
    }
}

impl Entity for ManagementAccountingConversion {
    type Id = ConversionLogicId;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn version(&self) -> u64 {
        self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_condition_report_creation() {
        let dept_margins = vec![DepartmentMargin::new(
            "D001".to_string(),
            "営業部".to_string(),
            Amount::from_i64(10_000_000),
            Amount::from_i64(6_000_000),
        )
        .unwrap()];

        let report = BusinessConditionReport::new(
            "2024-01".to_string(),
            Amount::from_i64(10_000_000),
            Amount::from_i64(4_000_000),
            Amount::from_i64(6_000_000),
            Amount::from_i64(3_000_000),
            Amount::from_i64(3_000_000),
            dept_margins,
            Amount::from_i64(5_000_000),
            Amount::from_i64(2_000_000),
        )
        .unwrap();

        assert_eq!(report.sales().to_i64(), Some(10_000_000));
        assert_eq!(report.contribution_margin().to_i64(), Some(6_000_000));
        assert_eq!(report.break_even_sales().to_i64(), Some(5_000_000));
        assert_eq!(report.safety_margin_rate(), 50.0);
    }

    #[test]
    fn test_business_condition_report_invalid_sales() {
        let result = BusinessConditionReport::new(
            "2024-01".to_string(),
            Amount::from_i64(-1),
            Amount::zero(),
            Amount::zero(),
            Amount::zero(),
            Amount::zero(),
            vec![],
            Amount::zero(),
            Amount::zero(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_business_condition_report_contribution_margin_rate() {
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

        assert_eq!(report.contribution_margin_rate(), 60.0);
    }

    #[test]
    fn test_business_condition_report_operating_profit_rate() {
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

        assert_eq!(report.operating_profit_rate(), 30.0);
    }

    #[test]
    fn test_department_margin_creation() {
        let margin = DepartmentMargin::new(
            "D001".to_string(),
            "営業部".to_string(),
            Amount::from_i64(10_000_000),
            Amount::from_i64(6_000_000),
        )
        .unwrap();

        assert_eq!(margin.contribution_margin().to_i64(), Some(4_000_000));
        assert_eq!(margin.contribution_margin_rate(), 40.0);
    }

    #[test]
    fn test_department_margin_invalid_amounts() {
        let result = DepartmentMargin::new(
            "D001".to_string(),
            "営業部".to_string(),
            Amount::from_i64(-1),
            Amount::zero(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_department_margin_zero_sales() {
        let margin = DepartmentMargin::new(
            "D001".to_string(),
            "営業部".to_string(),
            Amount::zero(),
            Amount::zero(),
        )
        .unwrap();

        assert_eq!(margin.contribution_margin_rate(), 0.0);
    }

    #[test]
    fn test_conversion_creation() {
        let conversion = ManagementAccountingConversion::new(
            ConversionType::FixedCostReclassification,
            vec!["6000".to_string(), "6100".to_string()],
            Amount::from_i64(5_000_000),
            Amount::from_i64(5_000_000),
            Some("売上比率".to_string()),
            "固定費再分類".to_string(),
            "2024-01".to_string(),
        )
        .unwrap();

        assert_eq!(conversion.target_accounts().len(), 2);
        assert!(!conversion.is_approved());
    }

    #[test]
    fn test_conversion_invalid_empty_accounts() {
        let result = ManagementAccountingConversion::new(
            ConversionType::FixedCostReclassification,
            vec![],
            Amount::from_i64(5_000_000),
            Amount::from_i64(5_000_000),
            None,
            "固定費再分類".to_string(),
            "2024-01".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_conversion_invalid_empty_reason() {
        let result = ManagementAccountingConversion::new(
            ConversionType::FixedCostReclassification,
            vec!["6000".to_string()],
            Amount::from_i64(5_000_000),
            Amount::from_i64(5_000_000),
            None,
            "".to_string(),
            "2024-01".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_conversion_approve() {
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

        let result = conversion.approve("CFO".to_string());
        assert!(result.is_ok());
        assert!(conversion.is_approved());
        assert_eq!(conversion.approved_by(), Some("CFO"));
    }

    #[test]
    fn test_conversion_double_approve() {
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
        let result = conversion.approve("CEO".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_conversion_difference() {
        let conversion = ManagementAccountingConversion::new(
            ConversionType::CommonCostAllocation,
            vec!["6000".to_string()],
            Amount::from_i64(5_000_000),
            Amount::from_i64(6_000_000),
            Some("人員比率".to_string()),
            "共通費配賦".to_string(),
            "2024-01".to_string(),
        )
        .unwrap();

        assert_eq!(conversion.conversion_difference().to_i64(), Some(1_000_000));
    }

    #[test]
    fn test_break_even_sales_zero_contribution_margin() {
        let report = BusinessConditionReport::new(
            "2024-01".to_string(),
            Amount::from_i64(10_000_000),
            Amount::zero(),
            Amount::zero(),
            Amount::from_i64(3_000_000),
            Amount::from_i64(-3_000_000),
            vec![],
            Amount::zero(),
            Amount::from_i64(1_000_000),
        )
        .unwrap();

        assert_eq!(report.break_even_sales().to_i64(), Some(0));
    }

    #[test]
    fn test_business_condition_report_with_multiple_departments() {
        let dept_margins = vec![
            DepartmentMargin::new(
                "D001".to_string(),
                "営業部".to_string(),
                Amount::from_i64(6_000_000),
                Amount::from_i64(3_600_000),
            )
            .unwrap(),
            DepartmentMargin::new(
                "D002".to_string(),
                "製造部".to_string(),
                Amount::from_i64(4_000_000),
                Amount::from_i64(2_400_000),
            )
            .unwrap(),
        ];

        let report = BusinessConditionReport::new(
            "2024-01".to_string(),
            Amount::from_i64(10_000_000),
            Amount::from_i64(4_000_000),
            Amount::from_i64(6_000_000),
            Amount::from_i64(3_000_000),
            Amount::from_i64(3_000_000),
            dept_margins,
            Amount::from_i64(5_000_000),
            Amount::from_i64(2_000_000),
        )
        .unwrap();

        assert_eq!(report.department_margins().len(), 2);
    }
}
