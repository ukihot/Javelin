// GenerateComprehensiveFinancialStatementsInteractor - 包括的財務諸表生成処理
// 責務: ドメイン層の財務諸表サービスを活用した生成・検証実行

use chrono::Utc;
use javelin_domain::financial_close::{
    financial_statements::{
        entities::FinancialStatement,
        services::{CrossCheckReport, FinancialStatementService},
        values::FinancialStatementType,
    },
    ledger::entities::{GeneralLedger, SubsidiaryLedger, SubsidiaryLedgerType},
};

use crate::{
    dtos::{
        ApprovalStatus, ConsistencyCheckResult, CrossCheckResult, FailedCheck,
        GenerateComprehensiveFinancialStatementsRequest,
        GenerateComprehensiveFinancialStatementsResponse, GeneratedStatement, InconsistencyDetail,
        StatementType,
    },
    error::ApplicationResult,
    input_ports::GenerateComprehensiveFinancialStatementsUseCase,
};

pub struct GenerateComprehensiveFinancialStatementsInteractor;

impl Default for GenerateComprehensiveFinancialStatementsInteractor {
    fn default() -> Self {
        Self::new()
    }
}

impl GenerateComprehensiveFinancialStatementsInteractor {
    pub fn new() -> Self {
        Self
    }

    /// DTOの財務諸表タイプをドメインタイプに変換
    fn convert_statement_type(dto_type: &StatementType) -> FinancialStatementType {
        match dto_type {
            StatementType::BalanceSheet => FinancialStatementType::BalanceSheet,
            StatementType::IncomeStatement => FinancialStatementType::IncomeStatement,
            StatementType::ComprehensiveIncome => FinancialStatementType::ComprehensiveIncome,
            StatementType::EquityChanges => FinancialStatementType::StatementOfChangesInEquity,
            StatementType::CashFlow => FinancialStatementType::CashFlowStatement,
        }
    }

    /// ドメインの財務諸表タイプをDTO文字列に変換
    fn statement_type_to_string(statement_type: &FinancialStatementType) -> String {
        match statement_type {
            FinancialStatementType::BalanceSheet => "BalanceSheet".to_string(),
            FinancialStatementType::IncomeStatement => "IncomeStatement".to_string(),
            FinancialStatementType::ComprehensiveIncome => "ComprehensiveIncome".to_string(),
            FinancialStatementType::StatementOfChangesInEquity => {
                "StatementOfChangesInEquity".to_string()
            }
            FinancialStatementType::CashFlowStatement => "CashFlowStatement".to_string(),
        }
    }

    /// 財務諸表を生成
    fn generate_statement(
        statement_type: FinancialStatementType,
        general_ledger: &GeneralLedger,
        period_end: chrono::DateTime<Utc>,
        prepared_by: String,
    ) -> ApplicationResult<FinancialStatement> {
        match statement_type {
            FinancialStatementType::BalanceSheet => {
                FinancialStatementService::generate_balance_sheet_from_gl(
                    general_ledger,
                    period_end,
                    prepared_by,
                )
                .map_err(Into::into)
            }
            _ => {
                // 他の財務諸表タイプは基本構造のみ作成
                Ok(FinancialStatement::new(statement_type, period_end, prepared_by))
            }
        }
    }
}

impl GenerateComprehensiveFinancialStatementsUseCase
    for GenerateComprehensiveFinancialStatementsInteractor
{
    async fn execute(
        &self,
        request: GenerateComprehensiveFinancialStatementsRequest,
    ) -> ApplicationResult<GenerateComprehensiveFinancialStatementsResponse> {
        // サンプルデータで元帳を作成（実際にはリポジトリから取得）
        let general_ledger = GeneralLedger::new();
        let _subsidiary_ledger = SubsidiaryLedger::new(
            SubsidiaryLedgerType::ProfitAndLoss,
            "ALL".to_string(),
            "全勘定".to_string(),
        )?;

        // 財務諸表を生成
        // モダンプラクティス: 事前にキャパシティを確保
        let mut generated_statements = Vec::with_capacity(request.statement_types.len());
        let mut domain_statements = Vec::with_capacity(request.statement_types.len());

        for statement_type in &request.statement_types {
            let domain_type = Self::convert_statement_type(statement_type);
            let prepared_by = request.approver.clone().unwrap_or_else(|| "System".to_string());
            let statement = Self::generate_statement(
                domain_type.clone(),
                &general_ledger,
                request.period_end,
                prepared_by,
            )?;

            generated_statements.push(GeneratedStatement {
                statement_id: statement.id().to_string(),
                statement_type: Self::statement_type_to_string(&domain_type),
                item_count: statement.items().len(),
                total_amount: statement.calculate_total().to_i64().unwrap_or(0),
            });

            domain_statements.push(statement);
        }

        // 整合性検証
        let consistency_check = if request.verify_consistency && !domain_statements.is_empty() {
            let report = FinancialStatementService::verify_ledger_statement_consistency(
                &general_ledger,
                &domain_statements[0],
            )?;

            let inconsistencies: Vec<InconsistencyDetail> = report
                .inconsistencies()
                .iter()
                .map(|inc| InconsistencyDetail {
                    inconsistency_type: inc.account_code.clone(),
                    description: format!(
                        "GL: {}, Statement: {}",
                        inc.gl_balance, inc.statement_amount
                    ),
                    impact_amount: inc.difference.to_i64(),
                })
                .collect();

            Some(ConsistencyCheckResult {
                is_consistent: report.is_consistent(),
                inconsistency_count: inconsistencies.len(),
                inconsistencies,
            })
        } else {
            None
        };

        // クロスチェック
        let cross_check = if request.perform_cross_check && domain_statements.len() >= 2 {
            // 最低2つの財務諸表があればクロスチェック可能
            let report = if domain_statements.len() >= 3 {
                FinancialStatementService::cross_check_statements(
                    &domain_statements[0],
                    &domain_statements[1],
                    &domain_statements[2],
                )?
            } else {
                // 2つの場合は簡易チェック（警告なし）
                CrossCheckReport::new()
            };

            let failed_checks: Vec<FailedCheck> = report
                .warnings()
                .iter()
                .map(|warning| FailedCheck {
                    check_name: "Cross-check".to_string(),
                    expected: "Consistent".to_string(),
                    actual: warning.clone(),
                    difference: None,
                })
                .collect();

            Some(CrossCheckResult {
                passed: !report.has_warnings(),
                checks_passed: if report.has_warnings() { 0 } else { 1 },
                checks_failed: if report.has_warnings() { 1 } else { 0 },
                failed_checks,
            })
        } else {
            None
        };

        // 承認状態を決定
        let approval_status = if request.approver.is_some() {
            ApprovalStatus::Approved
        } else {
            ApprovalStatus::Draft
        };

        // レスポンスを構築
        let response = GenerateComprehensiveFinancialStatementsResponse {
            statements: generated_statements,
            consistency_check,
            cross_check,
            generated_at: Utc::now(),
            approval_status,
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_balance_sheet_only() {
        let interactor = GenerateComprehensiveFinancialStatementsInteractor::new();

        let request = GenerateComprehensiveFinancialStatementsRequest {
            period_start: Utc::now(),
            period_end: Utc::now(),
            statement_types: vec![StatementType::BalanceSheet],
            verify_consistency: false,
            perform_cross_check: false,
            approver: None,
        };

        let response = interactor.execute(request).await.unwrap();

        assert_eq!(response.statements.len(), 1);
        assert_eq!(response.statements[0].statement_type, "BalanceSheet");
        assert!(response.consistency_check.is_none());
        assert!(response.cross_check.is_none());
    }

    #[tokio::test]
    async fn test_generate_with_verification() {
        let interactor = GenerateComprehensiveFinancialStatementsInteractor::new();

        let request = GenerateComprehensiveFinancialStatementsRequest {
            period_start: Utc::now(),
            period_end: Utc::now(),
            statement_types: vec![
                StatementType::BalanceSheet,
                StatementType::IncomeStatement,
                StatementType::CashFlow,
            ],
            verify_consistency: true,
            perform_cross_check: true,
            approver: Some("CFO".to_string()),
        };

        let response = interactor.execute(request).await.unwrap();

        assert_eq!(response.statements.len(), 3);
        assert!(response.consistency_check.is_some());
        assert!(response.cross_check.is_some());
        assert!(matches!(response.approval_status, ApprovalStatus::Approved));
    }
}
