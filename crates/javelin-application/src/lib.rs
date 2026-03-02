// Application Layer - ユースケース / Query / Projection制御
// 依存方向: → Domain

pub mod error;
pub mod interactor;
pub mod output_ports;
pub mod projection_builder;
pub mod query_service;

// DTOs - Request/Response data transfer objects
pub mod dtos {
    pub mod request;
    pub mod response;

    // Re-export all types from request and response modules
    // We use explicit re-exports to avoid ambiguous glob re-exports warning
    // since both modules have submodules with the same names

    // Request types
    pub use request::{
        AdjustAccountsRequest, ApplyIfrsValuationRequest, ApproveJournalEntryRequest,
        CancelJournalEntryRequest, ConsolidateLedgerRequest, CorrectJournalEntryRequest,
        CreateAdditionalEntryRequest, CreateReclassificationEntryRequest,
        CreateReplacementEntryRequest, CreateReversalEntryRequest, DeleteDraftJournalEntryRequest,
        EvaluateMaterialityRequest, FinancialMetrics,
        GenerateComprehensiveFinancialStatementsRequest, GenerateFinancialStatementsRequest,
        GenerateNoteDraftRequest, GenerateTrialBalanceRequest, GetJournalEntryQuery,
        JournalEntryLineDto, ListJournalEntriesQuery, LoadAccountMasterRequest,
        LockClosingPeriodRequest, PrepareClosingRequest, RegisterJournalEntryRequest,
        RejectJournalEntryRequest, ReverseJournalEntryRequest, StatementType,
        SubmitForApprovalRequest, UpdateDraftJournalEntryRequest, VerificationLevel,
        VerifyLedgerConsistencyRequest,
    };
    // Response types
    pub use response::{
        AccountBalanceDto, AccountBreakdownDto, AccountMasterItem, AccountReclassificationDto,
        AdjustAccountsResponse, AlertSeverity, AnomalyAlert, ApplyIfrsValuationResponse,
        ApprovalLevel as DtoApprovalLevel, ApprovalStatus, ApproveJournalEntryResponse,
        BalanceChange, BankReconciliationDifferenceDto, ConsistencyCheckResult,
        ConsolidateLedgerResponse, ContingentLiabilityDto, CorrectJournalEntryResponse,
        CrossCheckResult, DeleteDraftJournalEntryResponse, DiscrepancyDetail,
        EvaluateMaterialityResponse, FailedCheck, FairValueAdjustmentDto, FinancialIndicatorsDto,
        ForeignExchangeDifferenceDto, GenerateComprehensiveFinancialStatementsResponse,
        GenerateFinancialStatementsResponse, GenerateNoteDraftResponse,
        GenerateTrialBalanceResponse, GeneratedStatement, ImpairmentLossDto, InconsistencyDetail,
        InventoryWriteDownDto, JournalEntryDetail, JournalEntryLineDetail, JournalEntryListItem,
        JournalEntryListResult, LeaseMeasurementDto, LedgerDiscrepancyDto,
        LoadAccountMasterResponse, LockClosingPeriodResponse, PrepareClosingResponse,
        RegisterJournalEntryResponse, RejectJournalEntryResponse, ReverseJournalEntryResponse,
        StatementOfCashFlowsDto, StatementOfChangesInEquityDto, StatementOfFinancialPositionDto,
        StatementOfProfitOrLossDto, SubmitForApprovalResponse, TaxEffectAdjustmentDto,
        TemporaryAccountBalance, ThresholdInfo, UpdateDraftJournalEntryResponse,
        VerifyLedgerConsistencyResponse,
    };
}

// Input Ports - Use case trait definitions
pub mod input_ports {
    pub mod adjust_accounts;
    pub mod apply_ifrs_valuation;
    pub mod approve_journal_entry;
    pub mod cancel_journal_entry;
    pub mod consolidate_ledger;
    pub mod correct_journal_entry;
    pub mod create_additional_entry;
    pub mod create_reclassification_entry;
    pub mod create_replacement_entry;
    pub mod create_reversal_entry;
    pub mod delete_draft_journal_entry;
    pub mod evaluate_materiality;
    pub mod generate_comprehensive_financial_statements;
    pub mod generate_financial_statements;
    pub mod generate_note_draft;
    pub mod generate_trial_balance;
    pub mod get_journal_entry_detail;
    pub mod load_account_master;
    pub mod load_application_settings;
    pub mod load_company_master;
    pub mod load_subsidiary_account_master;
    pub mod lock_closing_period;
    pub mod prepare_closing;
    pub mod register_journal_entry;
    pub mod reject_journal_entry;
    pub mod reverse_journal_entry;
    pub mod search_journal_entry;
    pub mod submit_for_approval;
    pub mod update_draft_journal_entry;
    pub mod verify_ledger_consistency;

    // Re-export for convenience
    pub use adjust_accounts::*;
    pub use apply_ifrs_valuation::*;
    pub use approve_journal_entry::*;
    pub use cancel_journal_entry::*;
    pub use consolidate_ledger::*;
    pub use correct_journal_entry::*;
    pub use create_additional_entry::*;
    pub use create_reclassification_entry::*;
    pub use create_replacement_entry::*;
    pub use create_reversal_entry::*;
    pub use delete_draft_journal_entry::*;
    pub use evaluate_materiality::*;
    pub use generate_comprehensive_financial_statements::*;
    pub use generate_financial_statements::*;
    pub use generate_note_draft::*;
    pub use generate_trial_balance::*;
    pub use get_journal_entry_detail::*;
    pub use load_account_master::*;
    pub use load_application_settings::*;
    pub use load_company_master::*;
    pub use load_subsidiary_account_master::*;
    pub use lock_closing_period::*;
    pub use prepare_closing::*;
    pub use register_journal_entry::*;
    pub use reject_journal_entry::*;
    pub use reverse_journal_entry::*;
    pub use search_journal_entry::*;
    pub use submit_for_approval::*;
    pub use update_draft_journal_entry::*;
    pub use verify_ledger_consistency::*;
}
