// Interactor - Command実装
// 責務: ドメイン操作調整
// 利用対象: Entity / ValueObject / DomainService / RepositoryTrait

pub mod closing;
pub mod journal_entry;
pub mod master_data;

pub use closing::{
    AdjustAccountsInteractor, ApplyIfrsValuationInteractor, ConsolidateLedgerInteractor,
    GenerateFinancialStatementsInteractor, GenerateNoteDraftInteractor,
    GenerateTrialBalanceInteractor, LockClosingPeriodInteractor, PrepareClosingInteractor,
};
pub use journal_entry::{
    ApproveJournalEntryInteractor, CancelJournalEntryInteractor, CorrectJournalEntryInteractor,
    CreateAdditionalEntryInteractor, CreateReclassificationEntryInteractor,
    CreateReplacementEntryInteractor, CreateReversalEntryInteractor,
    DeleteDraftJournalEntryInteractor, RegisterJournalEntryInteractor,
    RejectJournalEntryInteractor, ReverseJournalEntryInteractor, SubmitForApprovalInteractor,
    UpdateDraftJournalEntryInteractor,
};
pub use master_data::{
    LoadAccountMasterInteractor, LoadApplicationSettingsInteractor, LoadCompanyMasterInteractor,
    LoadSubsidiaryAccountMasterInteractor,
};

// テストモジュール
#[cfg(test)]
mod tests;
