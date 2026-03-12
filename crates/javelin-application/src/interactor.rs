// Interactor - Command実装
// 責務: ドメイン操作調整
// 利用対象: Entity / ValueObject / DomainService / RepositoryTrait

pub mod closing;
pub mod journal_entry;
pub mod master_data;
pub mod print_invoice_interactor;

pub use closing::{
    AdjustAccountsInteractor, ApplyIfrsValuationInteractor, ConsolidateLedgerInteractor,
    EvaluateMaterialityInteractor, GenerateComprehensiveFinancialStatementsInteractor,
    GenerateFinancialStatementsInteractor, GenerateNoteDraftInteractor,
    GenerateTrialBalanceInteractor, LockClosingPeriodInteractor, PrepareClosingInteractor,
    VerifyLedgerConsistencyInteractor,
};
pub use journal_entry::{
    ApproveJournalEntryInteractor, CancelJournalEntryInteractor, CorrectJournalEntryInteractor,
    CreateAdditionalEntryInteractor, CreateReclassificationEntryInteractor,
    CreateReplacementEntryInteractor, CreateReversalEntryInteractor,
    DeleteDraftJournalEntryInteractor, GetJournalEntryDetailInteractor,
    RegisterJournalEntryInteractor, RejectJournalEntryInteractor, ReverseJournalEntryInteractor,
    SearchJournalEntryInteractor, SubmitForApprovalInteractor, UpdateDraftJournalEntryInteractor,
};
pub use master_data::{
    FetchAccountMasterInteractor,
    // FetchApplicationSettingsInteractor, // NOTE: ApplicationSettings 集約が削除されたため無効化
    FetchCompanyMasterInteractor,
    FetchSubsidiaryAccountMasterInteractor,
};
pub use print_invoice_interactor::PrintInvoiceInteractor;

// テストモジュール
#[cfg(test)]
mod tests;
