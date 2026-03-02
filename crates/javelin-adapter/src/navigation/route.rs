// Route - Screen identifier enum
// Identifies which screen to display (separate from page state)

/// Screen identifier for navigation
///
/// Identifies which screen to display without containing any state.
/// Each Route corresponds to a PageState implementation.
///
/// # Navigation Structure
///
/// The application follows a hierarchical navigation structure:
///
/// ```text
/// [TOP: Dashboard]
///     ├── [A. Primary Records] (Menu)
///     │   ├── Journal Entry (Menu)
///     │   │   ├── Journal Input (EXEC)
///     │   │   ├── Journal List (VIEW) → Journal Detail (VIEW)
///     │   │   └── Document Management (EXEC)
///     │   └── Cash Log (Menu)
///     │       ├── Cash Log Input (EXEC)
///     │       └── Cash Log List (VIEW)
///     ├── [B. Ledger Management] (Menu)
///     │   ├── Ledger Aggregation Execution (EXEC)
///     │   ├── General Ledger (VIEW) → Account Detail (VIEW)
///     │   ├── AR Ledger (VIEW) → AR Detail (VIEW)
///     │   └── AP Ledger (VIEW) → AP Detail (VIEW)
///     ├── [C. Fixed Assets & Lease] (Menu)
///     │   ├── Fixed Asset List (VIEW) → Asset Detail (VIEW)
///     │   ├── Asset Registration (EXEC)
///     │   ├── Depreciation Execution (EXEC) → Depreciation Result (VIEW)
///     │   ├── Lease Contract List (VIEW) → Lease Detail (VIEW) → Lease Schedule (VIEW)
///     │   └── ROU Asset List (VIEW)
///     ├── [D. Monthly Closing] (Menu)
///     │   ├── Closing Preparation (EXEC) → Preparation Result (VIEW)
///     │   ├── Closing Lock (EXEC)
///     │   ├── Trial Balance Generation (EXEC) → Trial Balance (VIEW)
///     │   ├── Account Adjustment (EXEC) → Adjustment Journal List (VIEW)
///     │   ├── Valuation Execution (EXEC) → Valuation Result (VIEW)
///     │   ├── Notes Draft Generation (EXEC) → Notes Draft (VIEW)
///     │   └── Financial Statement Generation (EXEC) → [E. Financial Statements]
///     ├── [E. Financial Statements] (Menu)
///     │   ├── Balance Sheet (VIEW)
///     │   ├── P/L and OCI (VIEW)
///     │   ├── Cash Flow Statement (VIEW)
///     │   ├── Statement of Changes in Equity (VIEW)
///     │   └── Notes (Menu)
///     │       ├── Accounting Policies (VIEW)
///     │       ├── Revenue Breakdown (VIEW)
///     │       ├── Fixed Assets Notes (VIEW)
///     │       ├── Lease Notes (VIEW)
///     │       └── Financial Instruments Notes (VIEW)
///     ├── [F. Management Accounting] (Menu)
///     │   ├── Management Accounting Conversion (EXEC) → Conversion Result (VIEW)
///     │   ├── Business Status Report (VIEW)
///     │   ├── Flux Analysis (VIEW)
///     │   ├── KPI Trends (VIEW)
///     │   ├── Financial Safety Report (VIEW)
///     │   └── Profitability Report (VIEW)
///     ├── [G. Judgment Log & Audit Trail] (Menu)
///     │   ├── Judgment Log List (VIEW) → Judgment Log Detail (VIEW)
///     │   ├── Judgment Log Input (EXEC)
///     │   ├── Audit Log List (VIEW) → Audit Log Detail (VIEW)
///     │   └── Period Management (EXEC)
///     └── [H. Master Management] (Menu)
///         ├── Chart of Accounts (VIEW/EXEC)
///         ├── Subsidiary Accounts (VIEW/EXEC)
///         └── Business Partners (VIEW/EXEC)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Route {
    // ========== TOP ==========
    /// TOP: Dashboard - Main entry point
    Home,
    /// TOP: Maintenance - Maintenance mode entry point
    MaintenanceHome,
    /// Maintenance menu (child of MaintenanceHome)
    MaintenanceMenu,
    /// Rebuild projections action
    MaintenanceRebuildProjections,
    /// Clean event store
    MaintenanceCleanEventStore,

    // ========== A. Primary Records ==========
    /// A-01: Primary Records Menu
    PrimaryRecordsMenu,

    // A. Journal Entry
    /// A-02: Journal Input (EXEC)
    JournalEntry,
    /// A-03: Journal List (VIEW)
    JournalList,
    /// A-04: Journal Detail (VIEW)
    JournalDetail,
    /// A-05: Document Management (EXEC)
    DocumentManagement,

    // A. Cash Log
    /// A-06: Cash Log Input (EXEC)
    CashLogInput,
    /// A-07: Cash Log List (VIEW)
    CashLogList,

    // ========== B. Ledger Management ==========
    /// B-01: Ledger Management Menu
    LedgerMenu,
    /// B-02: Ledger Aggregation Execution (EXEC)
    LedgerAggregationExecution,
    /// B-03: General Ledger (VIEW)
    GeneralLedger,
    /// B-04: Account Detail (VIEW)
    AccountDetail,
    /// B-05: AR Ledger (VIEW)
    ArLedger,
    /// B-06: AR Detail (VIEW)
    ArDetail,
    /// B-07: AP Ledger (VIEW)
    ApLedger,
    /// B-08: AP Detail (VIEW)
    ApDetail,

    // ========== C. Fixed Assets & Lease ==========
    /// C-01: Fixed Assets & Lease Menu
    FixedAssetsMenu,
    /// C-02: Fixed Asset List (VIEW)
    FixedAssetList,
    /// C-03: Asset Detail (VIEW)
    AssetDetail,
    /// C-04: Asset Registration (EXEC)
    AssetRegistration,
    /// C-05: Depreciation Execution (EXEC)
    DepreciationExecution,
    /// C-06: Depreciation Result (VIEW)
    DepreciationResult,
    /// C-07: Lease Contract List (VIEW)
    LeaseContractList,
    /// C-08: Lease Contract Detail (VIEW)
    LeaseContractDetail,
    /// C-09: Lease Schedule (VIEW)
    LeaseSchedule,
    /// C-10: ROU Asset List (VIEW)
    RouAssetList,

    // ========== D. Monthly Closing ==========
    /// D-01: Monthly Closing Menu (Close Calendar)
    ClosingMenu,
    /// D-02: Closing Preparation Execution (EXEC)
    ClosingPreparationExecution,
    /// D-03: Closing Preparation Result (VIEW)
    ClosingPreparationResult,
    /// D-04: Closing Lock Execution (EXEC)
    ClosingLockExecution,
    /// D-05: Trial Balance Generation Execution (EXEC)
    TrialBalanceGenerationExecution,
    /// D-06: Trial Balance (VIEW)
    TrialBalance,
    /// D-07: Account Adjustment Execution (EXEC)
    AccountAdjustmentExecution,
    /// D-08: Adjustment Journal List (VIEW)
    AdjustmentJournalList,
    /// D-09: Valuation Execution (EXEC)
    ValuationExecution,
    /// D-10: Valuation Result (VIEW)
    ValuationResult,
    /// D-11: Notes Draft Generation Execution (EXEC)
    NotesDraftGenerationExecution,
    /// D-12: Notes Draft (VIEW)
    NotesDraft,
    /// D-13: Financial Statement Generation Execution (EXEC)
    FinancialStatementGenerationExecution,
    /// D-14: Materiality Evaluation (EXEC)
    MaterialityEvaluation,
    /// D-15: Ledger Consistency Verification (EXEC)
    LedgerConsistencyVerification,
    /// D-16: Comprehensive Financial Statements Generation (EXEC)
    ComprehensiveFinancialStatements,

    // ========== E. Financial Statements ==========
    /// E-01: Financial Statements Menu
    FinancialStatementsMenu,
    /// E-02: Balance Sheet (VIEW)
    BalanceSheet,
    /// E-03: P/L and OCI (VIEW)
    PlAndOci,
    /// E-04: Cash Flow Statement (VIEW)
    CashFlowStatement,
    /// E-05: Statement of Changes in Equity (VIEW)
    StatementOfChangesInEquity,
    /// E-06: Notes Menu
    NotesMenu,
    /// E-07: Accounting Policies (VIEW)
    AccountingPolicies,
    /// E-08: Revenue Breakdown (VIEW)
    RevenueBreakdown,
    /// E-09: Fixed Assets Notes (VIEW)
    FixedAssetsNotes,
    /// E-10: Lease Notes (VIEW)
    LeaseNotes,
    /// E-11: Financial Instruments Notes (VIEW)
    FinancialInstrumentsNotes,

    // ========== F. Management Accounting ==========
    /// F-01: Management Accounting Menu
    ManagementAccountingMenu,
    /// F-02: Management Accounting Conversion Execution (EXEC)
    ManagementAccountingConversionExecution,
    /// F-03: Conversion Result (VIEW)
    ConversionResult,
    /// F-04: Business Status Report (VIEW)
    BusinessStatusReport,
    /// F-05: Flux Analysis (VIEW)
    FluxAnalysis,
    /// F-06: KPI Trends (VIEW)
    KpiTrends,
    /// F-07: Financial Safety Report (VIEW)
    FinancialSafetyReport,
    /// F-08: Profitability Report (VIEW)
    ProfitabilityReport,

    // ========== G. Judgment Log & Audit Trail ==========
    /// G-01: Judgment Log List (VIEW)
    JudgmentLogList,
    /// G-02: Judgment Log Detail (VIEW)
    JudgmentLogDetail,
    /// G-03: Judgment Log Input (EXEC)
    JudgmentLogInput,
    /// G-04: Audit Log List (VIEW)
    AuditLogList,
    /// G-05: Audit Log Detail (VIEW)
    AuditLogDetail,
    /// G-06: Period Management (EXEC)
    PeriodManagement,

    // ========== H. Master Management ==========
    /// H-01: Master Management Menu
    MasterManagementMenu,
    /// H-02: Chart of Accounts (VIEW/EXEC)
    ChartOfAccounts,
    /// H-03: Subsidiary Accounts (VIEW/EXEC)
    SubsidiaryAccounts,
    /// H-04: Business Partners (VIEW/EXEC)
    BusinessPartners,
}
