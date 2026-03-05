// PageStateResolver - ルートからPageStateへの解決
// 責務: Route → PageState のマッピング

use std::sync::Arc;

use javelin_adapter::{HomePageState, PageState, PresenterRegistry, Route};

use crate::app_error::AppResult;

/// PageStateの解決を担当
pub struct PageStateResolver {
    presenter_registry: Arc<PresenterRegistry>,
}

impl PageStateResolver {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { presenter_registry }
    }

    /// ルートからPageStateを解決
    pub fn resolve(&self, route: Route) -> AppResult<Box<dyn PageState>> {
        match route {
            // ========== TOP ==========
            Route::Home => Ok(Box::new(HomePageState::new())),
            Route::MaintenanceHome => {
                Ok(Box::new(javelin_adapter::MaintenanceHomePageState::new()))
            }
            Route::MaintenanceMenu => {
                Ok(Box::new(javelin_adapter::MaintenanceMenuPageState::new()))
            }
            Route::MaintenanceRebuildProjections => {
                Ok(Box::new(javelin_adapter::RebuildProjectionsPageState::new()))
            }
            Route::MaintenanceCleanEventStore => {
                Ok(Box::new(javelin_adapter::CleanEventStorePageState::new()))
            }

            // ========== 販売管理 ==========
            Route::InvoicePrint => Ok(Box::new(javelin_adapter::InvoicePrintPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),

            // ========== A. Primary Records ==========
            Route::PrimaryRecordsMenu => {
                Ok(Box::new(javelin_adapter::PrimaryRecordsMenuPageState::new()))
            }
            Route::JournalEntry => Ok(Box::new(javelin_adapter::JournalEntryPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),
            Route::JournalList => Ok(Box::new(javelin_adapter::JournalListPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),
            Route::JournalDetail => Ok(Box::new(javelin_adapter::JournalDetailPageState::new(
                Arc::clone(&self.presenter_registry),
                "dummy-entry-id".to_string(),
            ))),
            Route::DocumentManagement => {
                Ok(Box::new(javelin_adapter::DocumentManagementPageState::new()))
            }
            Route::CashLogInput => Ok(Box::new(javelin_adapter::CashLogInputPageState::new())),
            Route::CashLogList => Ok(Box::new(javelin_adapter::CashLogListPageState::new())),

            // ========== B. Ledger Management ==========
            Route::LedgerMenu => Ok(Box::new(javelin_adapter::LedgerMenuPageState::new())),
            Route::LedgerAggregationExecution => {
                Ok(Box::new(javelin_adapter::LedgerAggregationExecutionPageState::new()))
            }
            Route::GeneralLedger => Ok(Box::new(javelin_adapter::GeneralLedgerPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),
            Route::AccountDetail => Ok(Box::new(javelin_adapter::AccountDetailPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),
            Route::ArLedger => Ok(Box::new(javelin_adapter::ArLedgerPageState::new(Arc::clone(
                &self.presenter_registry,
            )))),
            Route::ArDetail => Ok(Box::new(javelin_adapter::ArDetailPageState::new(Arc::clone(
                &self.presenter_registry,
            )))),
            Route::ApLedger => Ok(Box::new(javelin_adapter::ApLedgerPageState::new(Arc::clone(
                &self.presenter_registry,
            )))),
            Route::ApDetail => Ok(Box::new(javelin_adapter::ApDetailPageState::new(Arc::clone(
                &self.presenter_registry,
            )))),

            // ========== C. Fixed Assets & Lease ==========
            Route::FixedAssetsMenu => {
                Ok(Box::new(javelin_adapter::FixedAssetsMenuPageState::new()))
            }
            Route::FixedAssetList => Ok(Box::new(javelin_adapter::FixedAssetListPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),
            Route::AssetDetail => Ok(Box::new(javelin_adapter::AssetDetailPageState::new())),
            Route::AssetRegistration => {
                Ok(Box::new(javelin_adapter::AssetRegistrationPageState::new()))
            }
            Route::DepreciationExecution => {
                Ok(Box::new(javelin_adapter::DepreciationExecutionPageState::new()))
            }
            Route::DepreciationResult => {
                Ok(Box::new(javelin_adapter::DepreciationResultPageState::new(Arc::clone(
                    &self.presenter_registry,
                ))))
            }
            Route::LeaseContractList => {
                Ok(Box::new(javelin_adapter::LeaseContractListPageState::new()))
            }
            Route::LeaseContractDetail => {
                Ok(Box::new(javelin_adapter::LeaseContractDetailPageState::new()))
            }
            Route::LeaseSchedule => Ok(Box::new(javelin_adapter::LeaseSchedulePageState::new())),
            Route::RouAssetList => Ok(Box::new(javelin_adapter::RouAssetListPageState::new())),

            // ========== D. Monthly Closing ==========
            Route::ClosingMenu => Ok(Box::new(javelin_adapter::ClosingMenuPageState::new())),
            Route::ClosingPreparationExecution => {
                Ok(Box::new(javelin_adapter::ClosingPreparationExecutionPageState::new()))
            }
            Route::ClosingPreparationResult => {
                Ok(Box::new(javelin_adapter::PreparationResultPageState::new()))
            }
            Route::ClosingLockExecution => {
                Ok(Box::new(javelin_adapter::ClosingLockExecutionPageState::new()))
            }
            Route::TrialBalanceGenerationExecution => {
                Ok(Box::new(javelin_adapter::TrialBalanceGenerationExecutionPageState::new()))
            }
            Route::TrialBalance => Ok(Box::new(javelin_adapter::TrialBalancePageState::new())),
            Route::AccountAdjustmentExecution => {
                Ok(Box::new(javelin_adapter::AccountAdjustmentExecutionPageState::new()))
            }
            Route::AdjustmentJournalList => {
                Ok(Box::new(javelin_adapter::AdjustmentJournalListPageState::new()))
            }
            Route::ValuationExecution => {
                Ok(Box::new(javelin_adapter::IfrsValuationExecutionPageState::new()))
            }
            Route::ValuationResult => {
                Ok(Box::new(javelin_adapter::ValuationResultPageState::new()))
            }
            Route::NotesDraftGenerationExecution => {
                Ok(Box::new(javelin_adapter::NotesDraftGenerationExecutionPageState::new()))
            }
            Route::NotesDraft => Ok(Box::new(javelin_adapter::NoteDraftPageState::new())),
            Route::FinancialStatementGenerationExecution => {
                Ok(Box::new(javelin_adapter::FinancialStatementExecutionPageState::new()))
            }

            // ========== E. Financial Statements ==========
            Route::FinancialStatementsMenu => {
                Ok(Box::new(javelin_adapter::FinancialStatementsMenuPageState::new()))
            }
            Route::BalanceSheet => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::BalanceSheet,
                "E-02: Balance Sheet",
                "財政状態計算書（BS）画面",
            ))),
            Route::PlAndOci => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::PlAndOci,
                "E-03: P/L and OCI",
                "損益及びその他の包括利益計算書画面",
            ))),
            Route::CashFlowStatement => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::CashFlowStatement,
                "E-04: Cash Flow Statement",
                "キャッシュフロー計算書（SCF）画面",
            ))),
            Route::StatementOfChangesInEquity => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::StatementOfChangesInEquity,
                "E-05: Statement of Changes in Equity",
                "持分変動計算書（SCE）画面",
            ))),
            Route::NotesMenu => Ok(Box::new(javelin_adapter::NotesMenuPageState::new())),
            Route::AccountingPolicies => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::AccountingPolicies,
                "E-07: Accounting Policies",
                "注記：会計方針画面",
            ))),
            Route::RevenueBreakdown => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::RevenueBreakdown,
                "E-08: Revenue Breakdown",
                "注記：収益分解（IFRS 15）画面",
            ))),
            Route::FixedAssetsNotes => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::FixedAssetsNotes,
                "E-09: Fixed Assets Notes",
                "注記：固定資産・使用権資産画面",
            ))),
            Route::LeaseNotes => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::LeaseNotes,
                "E-10: Lease Notes",
                "注記：リース（IFRS 16）画面",
            ))),
            Route::FinancialInstrumentsNotes => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::FinancialInstrumentsNotes,
                "E-11: Financial Instruments Notes",
                "注記：金融商品・ECL（IFRS 9）画面",
            ))),

            // ========== F. Management Accounting ==========
            Route::ManagementAccountingMenu => {
                Ok(Box::new(javelin_adapter::ManagementAccountingMenuPageState::new()))
            }
            Route::ManagementAccountingConversionExecution => {
                Ok(Box::new(javelin_adapter::StubPageState::new(
                    Route::ManagementAccountingConversionExecution,
                    "F-02: Management Accounting Conversion Execution",
                    "管理会計変換実行画面",
                )))
            }
            Route::ConversionResult => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::ConversionResult,
                "F-03: Conversion Result",
                "変換差異検証画面",
            ))),
            Route::BusinessStatusReport => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::BusinessStatusReport,
                "F-04: Business Status Report",
                "月次業況表画面",
            ))),
            Route::FluxAnalysis => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::FluxAnalysis,
                "F-05: Flux Analysis",
                "差異分析画面",
            ))),
            Route::KpiTrends => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::KpiTrends,
                "F-06: KPI Trends",
                "KPI推移画面",
            ))),
            Route::FinancialSafetyReport => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::FinancialSafetyReport,
                "F-07: Financial Safety Report",
                "財務安全性レポート画面",
            ))),
            Route::ProfitabilityReport => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::ProfitabilityReport,
                "F-08: Profitability Report",
                "収益性・投資効率レポート画面",
            ))),

            // ========== G. Judgment Log & Audit Trail ==========
            Route::JudgmentLogList => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::JudgmentLogList,
                "G-01: Judgment Log List",
                "判断ログ一覧画面",
            ))),
            Route::JudgmentLogDetail => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::JudgmentLogDetail,
                "G-02: Judgment Log Detail",
                "判断ログ詳細画面",
            ))),
            Route::JudgmentLogInput => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::JudgmentLogInput,
                "G-03: Judgment Log Input",
                "判断ログ入力画面",
            ))),
            Route::AuditLogList => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::AuditLogList,
                "G-04: Audit Log List",
                "監査ログ一覧画面",
            ))),
            Route::AuditLogDetail => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::AuditLogDetail,
                "G-05: Audit Log Detail",
                "監査ログ詳細画面",
            ))),
            Route::PeriodManagement => {
                Ok(Box::new(javelin_adapter::ApplicationSettingsPageState::new(Arc::clone(
                    &self.presenter_registry,
                ))))
            }

            // ========== H. Master Management ==========
            Route::MasterManagementMenu => {
                Ok(Box::new(javelin_adapter::MasterManagementMenuPageState::new()))
            }
            Route::ChartOfAccounts => Ok(Box::new(javelin_adapter::AccountMasterPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),
            Route::SubsidiaryAccounts => {
                Ok(Box::new(javelin_adapter::SubsidiaryAccountMasterPageState::new(Arc::clone(
                    &self.presenter_registry,
                ))))
            }
            Route::BusinessPartners => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::BusinessPartners,
                "H-04: Business Partners",
                "取引先マスタ画面",
            ))),

            // Phase 3 Routes
            Route::MaterialityEvaluation => {
                Ok(Box::new(javelin_adapter::MaterialityEvaluationPageState::new()))
            }
            Route::LedgerConsistencyVerification => {
                Ok(Box::new(javelin_adapter::LedgerConsistencyVerificationPageState::new()))
            }
            Route::ComprehensiveFinancialStatements => {
                Ok(Box::new(javelin_adapter::ComprehensiveFinancialStatementsPageState::new()))
            }
        }
    }
}
