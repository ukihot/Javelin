// Interactor - Command実装（書き込み操作のみ）
// 責務: ドメイン操作調整
// 利用対象: Entity / ValueObject / DomainService / RepositoryTrait
//
// CQRS原則: Interactorはコマンド（書き込み）専用
// クエリ（読み取り）はQueryServiceを直接使用

pub mod closing;
pub mod journal_entry;
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
pub use print_invoice_interactor::PrintInvoiceInteractor;

// テストモジュール
#[cfg(test)]
mod tests;
