// ClosingController - 月次決算処理コントローラ
// 責務: 月次決算関連のユースケースを呼び出す

use std::sync::Arc;

use javelin_application::{
    dtos::{
        AdjustAccountsRequest, AdjustAccountsResponse, ApplyIfrsValuationRequest,
        ApplyIfrsValuationResponse, ConsolidateLedgerRequest, ConsolidateLedgerResponse,
        GenerateFinancialStatementsRequest, GenerateFinancialStatementsResponse,
        GenerateNoteDraftRequest, GenerateNoteDraftResponse, GenerateTrialBalanceRequest,
        GenerateTrialBalanceResponse, LockClosingPeriodRequest, LockClosingPeriodResponse,
        PrepareClosingRequest, PrepareClosingResponse,
    },
    input_ports::{
        AdjustAccountsUseCase, ApplyIfrsValuationUseCase, ConsolidateLedgerUseCase,
        GenerateFinancialStatementsUseCase, GenerateNoteDraftUseCase, GenerateTrialBalanceUseCase,
        LockClosingPeriodUseCase, PrepareClosingUseCase,
    },
    interactor::{
        AdjustAccountsInteractor, ApplyIfrsValuationInteractor, ConsolidateLedgerInteractor,
        GenerateFinancialStatementsInteractor, GenerateNoteDraftInteractor,
        GenerateTrialBalanceInteractor, LockClosingPeriodInteractor, PrepareClosingInteractor,
    },
};
use javelin_infrastructure::{
    read::queries::LedgerQueryServiceImpl, write::event_store::ClosingEventStore,
};

use crate::{error::AdapterResult, presenter::LedgerPresenter};

/// 月次決算処理コントローラ（具体型版）
///
/// 型パラメータを削除し、具体的なInteractor型を直接保持することで
/// 型定義の複雑性を解消し、他のControllerと統一したパターンに変更
pub struct ClosingController {
    consolidate_ledger: Arc<ConsolidateLedgerInteractor<LedgerQueryServiceImpl>>,
    prepare_closing: Arc<PrepareClosingInteractor<LedgerQueryServiceImpl>>,
    lock_closing_period: Arc<LockClosingPeriodInteractor<ClosingEventStore>>,
    generate_trial_balance: Arc<GenerateTrialBalanceInteractor<LedgerQueryServiceImpl>>,
    generate_note_draft: Arc<GenerateNoteDraftInteractor<LedgerQueryServiceImpl>>,
    adjust_accounts: Arc<AdjustAccountsInteractor<ClosingEventStore, LedgerQueryServiceImpl>>,
    apply_ifrs_valuation: Arc<
        ApplyIfrsValuationInteractor<ClosingEventStore, LedgerQueryServiceImpl, LedgerPresenter>,
    >,
    generate_financial_statements:
        Arc<GenerateFinancialStatementsInteractor<LedgerQueryServiceImpl>>,
}

impl ClosingController {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        consolidate_ledger: Arc<ConsolidateLedgerInteractor<LedgerQueryServiceImpl>>,
        prepare_closing: Arc<PrepareClosingInteractor<LedgerQueryServiceImpl>>,
        lock_closing_period: Arc<LockClosingPeriodInteractor<ClosingEventStore>>,
        generate_trial_balance: Arc<GenerateTrialBalanceInteractor<LedgerQueryServiceImpl>>,
        generate_note_draft: Arc<GenerateNoteDraftInteractor<LedgerQueryServiceImpl>>,
        adjust_accounts: Arc<AdjustAccountsInteractor<ClosingEventStore, LedgerQueryServiceImpl>>,
        apply_ifrs_valuation: Arc<
            ApplyIfrsValuationInteractor<
                ClosingEventStore,
                LedgerQueryServiceImpl,
                LedgerPresenter,
            >,
        >,
        generate_financial_statements: Arc<
            GenerateFinancialStatementsInteractor<LedgerQueryServiceImpl>,
        >,
    ) -> Self {
        Self {
            consolidate_ledger,
            prepare_closing,
            lock_closing_period,
            generate_trial_balance,
            generate_note_draft,
            adjust_accounts,
            apply_ifrs_valuation,
            generate_financial_statements,
        }
    }

    /// 元帳集約処理
    pub async fn consolidate_ledger(
        &self,
        request: ConsolidateLedgerRequest,
    ) -> AdapterResult<ConsolidateLedgerResponse> {
        self.consolidate_ledger
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 締準備処理
    pub async fn prepare_closing(
        &self,
        request: PrepareClosingRequest,
    ) -> AdapterResult<PrepareClosingResponse> {
        self.prepare_closing
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 締日固定処理
    pub async fn lock_closing_period(
        &self,
        request: LockClosingPeriodRequest,
    ) -> AdapterResult<LockClosingPeriodResponse> {
        self.lock_closing_period
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 試算表生成処理
    pub async fn generate_trial_balance(
        &self,
        request: GenerateTrialBalanceRequest,
    ) -> AdapterResult<GenerateTrialBalanceResponse> {
        self.generate_trial_balance
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 注記草案生成処理
    pub async fn generate_note_draft(
        &self,
        request: GenerateNoteDraftRequest,
    ) -> AdapterResult<GenerateNoteDraftResponse> {
        self.generate_note_draft
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 勘定補正処理
    pub async fn adjust_accounts(
        &self,
        request: AdjustAccountsRequest,
    ) -> AdapterResult<AdjustAccountsResponse> {
        self.adjust_accounts
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// IFRS評価処理
    pub async fn apply_ifrs_valuation(
        &self,
        request: ApplyIfrsValuationRequest,
    ) -> AdapterResult<ApplyIfrsValuationResponse> {
        self.apply_ifrs_valuation
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 財務諸表生成処理
    pub async fn generate_financial_statements(
        &self,
        request: GenerateFinancialStatementsRequest,
    ) -> AdapterResult<GenerateFinancialStatementsResponse> {
        self.generate_financial_statements
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }
}
