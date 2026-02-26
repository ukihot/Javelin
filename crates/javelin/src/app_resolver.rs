// PageStateResolver - ルートからPageStateへの解決
// 責務: Route → PageState のマッピング

use std::sync::Arc;

use javelin_adapter::{
    HomePageState, PageState, PresenterRegistry, Route, SearchPageState, navigation::Controllers,
};

use crate::app_error::{AppError, AppResult};

/// PageStateの解決を担当
pub struct PageStateResolver {
    presenter_registry: Arc<PresenterRegistry>,
    controllers: Arc<Controllers>,
}

impl PageStateResolver {
    pub fn new(presenter_registry: Arc<PresenterRegistry>, controllers: Arc<Controllers>) -> Self {
        Self { presenter_registry, controllers }
    }

    /// ルートからPageStateを解決
    pub fn resolve(&self, route: Route) -> AppResult<Box<dyn PageState>> {
        match route {
            // ========== TOP ==========
            Route::Home => Ok(Box::new(HomePageState::new())),

            // ========== A. Primary Records ==========
            Route::PrimaryRecordsMenu => {
                Ok(Box::new(javelin_adapter::PrimaryRecordsMenuPageState::new()))
            }
            Route::JournalEntry => Ok(Box::new(javelin_adapter::JournalEntryPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),
            Route::JournalList => {
                Ok(Box::new(SearchPageState::new(Arc::clone(&self.presenter_registry))))
            }
            Route::JournalDetail => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::JournalDetail,
                "A-04: Journal Detail",
                "仕訳詳細画面",
            ))),
            Route::DocumentManagement => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::DocumentManagement,
                "A-05: Document Management",
                "証憑管理画面",
            ))),
            Route::CashLogInput => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::CashLogInput,
                "A-06: Cash Log Input",
                "キャッシュログ入力画面",
            ))),
            Route::CashLogList => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::CashLogList,
                "A-07: Cash Log List",
                "キャッシュログ一覧画面",
            ))),

            // ========== B. Ledger Management ==========
            Route::LedgerMenu => Ok(Box::new(javelin_adapter::LedgerMenuPageState::new())),
            Route::LedgerAggregationExecution => {
                Ok(Box::new(javelin_adapter::LedgerConsolidationExecutionPageState::new()))
            }
            Route::GeneralLedger => Ok(Box::new(javelin_adapter::LedgerPageState::new())),
            Route::AccountDetail => Ok(Box::new(javelin_adapter::LedgerDetailPageState::new())),
            Route::ArLedger => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::ArLedger,
                "B-05: AR Ledger",
                "売掛金補助元帳画面",
            ))),
            Route::ArDetail => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::ArDetail,
                "B-06: AR Detail",
                "売掛金明細画面",
            ))),
            Route::ApLedger => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::ApLedger,
                "B-07: AP Ledger",
                "買掛金補助元帳画面",
            ))),
            Route::ApDetail => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::ApDetail,
                "B-08: AP Detail",
                "買掛金明細画面",
            ))),

            // ========== C. Fixed Assets & Lease ==========
            Route::FixedAssetsMenu => {
                Ok(Box::new(javelin_adapter::FixedAssetsMenuPageState::new()))
            }
            Route::FixedAssetList => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::FixedAssetList,
                "C-02: Fixed Asset List",
                "固定資産一覧画面",
            ))),
            Route::AssetDetail => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::AssetDetail,
                "C-03: Asset Detail",
                "資産詳細画面",
            ))),
            Route::AssetRegistration => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::AssetRegistration,
                "C-04: Asset Registration",
                "資産登録実行画面",
            ))),
            Route::DepreciationExecution => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::DepreciationExecution,
                "C-05: Depreciation Execution",
                "減価償却計算実行画面",
            ))),
            Route::DepreciationResult => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::DepreciationResult,
                "C-06: Depreciation Result",
                "償却計算結果一覧画面",
            ))),
            Route::LeaseContractList => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::LeaseContractList,
                "C-07: Lease Contract List",
                "リース契約一覧画面",
            ))),
            Route::LeaseContractDetail => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::LeaseContractDetail,
                "C-08: Lease Contract Detail",
                "リース契約詳細画面",
            ))),
            Route::LeaseSchedule => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::LeaseSchedule,
                "C-09: Lease Schedule",
                "リース負債スケジュール画面",
            ))),
            Route::RouAssetList => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::RouAssetList,
                "C-10: ROU Asset List",
                "使用権資産台帳画面",
            ))),

            // ========== D. Monthly Closing ==========
            Route::ClosingMenu => Ok(Box::new(javelin_adapter::ClosingMenuPageState::new())),
            Route::ClosingPreparationExecution => {
                Ok(Box::new(javelin_adapter::ClosingPreparationExecutionPageState::new()))
            }
            Route::ClosingPreparationResult => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::ClosingPreparationResult,
                "D-03: Closing Preparation Result",
                "締準備処理結果画面",
            ))),
            Route::ClosingLockExecution => {
                Ok(Box::new(javelin_adapter::ClosingLockPageState::new()))
            }
            Route::TrialBalanceGenerationExecution => {
                Ok(Box::new(javelin_adapter::StubPageState::new(
                    Route::TrialBalanceGenerationExecution,
                    "D-05: Trial Balance Generation Execution",
                    "試算表生成実行画面",
                )))
            }
            Route::TrialBalance => Ok(Box::new(javelin_adapter::TrialBalancePageState::new())),
            Route::AccountAdjustmentExecution => {
                Ok(Box::new(javelin_adapter::AccountAdjustmentExecutionPageState::new()))
            }
            Route::AdjustmentJournalList => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::AdjustmentJournalList,
                "D-08: Adjustment Journal List",
                "補正仕訳一覧画面",
            ))),
            Route::ValuationExecution => {
                Ok(Box::new(javelin_adapter::IfrsValuationExecutionPageState::new()))
            }
            Route::ValuationResult => Ok(Box::new(javelin_adapter::StubPageState::new(
                Route::ValuationResult,
                "D-10: Valuation Result",
                "評価結果一覧画面",
            ))),
            Route::NotesDraftGenerationExecution => {
                Ok(Box::new(javelin_adapter::StubPageState::new(
                    Route::NotesDraftGenerationExecution,
                    "D-11: Notes Draft Generation Execution",
                    "注記草案生成実行画面",
                )))
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

            // ========== Legacy Routes (deprecated) ==========
            Route::Search => {
                Ok(Box::new(SearchPageState::new(Arc::clone(&self.presenter_registry))))
            }
            Route::Ledger => Ok(Box::new(javelin_adapter::LedgerPageState::new())),
            Route::LedgerDetail => Ok(Box::new(javelin_adapter::LedgerDetailPageState::new())),
            Route::LedgerConsolidation => {
                Ok(Box::new(javelin_adapter::LedgerConsolidationPageState::new(&self.controllers)))
            }
            Route::LedgerConsolidationExecution => {
                Ok(Box::new(javelin_adapter::LedgerConsolidationExecutionPageState::new()))
            }
            Route::ClosingPreparation => {
                Ok(Box::new(javelin_adapter::ClosingPreparationPageState::new(&self.controllers)))
            }
            Route::ClosingLock => Ok(Box::new(javelin_adapter::ClosingLockPageState::new())),
            Route::AccountAdjustment => {
                Ok(Box::new(javelin_adapter::AccountAdjustmentPageState::new(&self.controllers)))
            }
            Route::NoteDraft => Ok(Box::new(javelin_adapter::NoteDraftPageState::new())),
            Route::IfrsValuation => {
                Ok(Box::new(javelin_adapter::IfrsValuationPageState::new(&self.controllers)))
            }
            Route::FinancialStatement => {
                Ok(Box::new(javelin_adapter::FinancialStatementPageState::new(&self.controllers)))
            }
            Route::AccountMaster => Ok(Box::new(javelin_adapter::AccountMasterPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),
            Route::SubsidiaryAccountMaster => {
                Ok(Box::new(javelin_adapter::SubsidiaryAccountMasterPageState::new(Arc::clone(
                    &self.presenter_registry,
                ))))
            }
            Route::ApplicationSettings => {
                Ok(Box::new(javelin_adapter::ApplicationSettingsPageState::new(Arc::clone(
                    &self.presenter_registry,
                ))))
            }
            Route::DataImport | Route::DataExport => {
                Err(AppError::NotImplemented("Legacy route no longer supported".to_string()))
            }
        }
    }
}
