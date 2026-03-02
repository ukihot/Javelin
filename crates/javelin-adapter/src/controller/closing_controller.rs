// ClosingController - 月次決算処理コントローラ
// 責務: 月次決算関連のユースケースを呼び出す

use std::sync::Arc;

use javelin_application::{
    dtos::{
        AdjustAccountsRequest, AdjustAccountsResponse, ApplyIfrsValuationRequest,
        ApplyIfrsValuationResponse, ConsolidateLedgerRequest, ConsolidateLedgerResponse,
        EvaluateMaterialityRequest, EvaluateMaterialityResponse,
        GenerateComprehensiveFinancialStatementsRequest,
        GenerateComprehensiveFinancialStatementsResponse, GenerateFinancialStatementsRequest,
        GenerateFinancialStatementsResponse, GenerateNoteDraftRequest, GenerateNoteDraftResponse,
        GenerateTrialBalanceRequest, GenerateTrialBalanceResponse, LockClosingPeriodRequest,
        LockClosingPeriodResponse, PrepareClosingRequest, PrepareClosingResponse,
        VerifyLedgerConsistencyRequest, VerifyLedgerConsistencyResponse,
    },
    input_ports::{
        AdjustAccountsUseCase, ApplyIfrsValuationUseCase, ConsolidateLedgerUseCase,
        EvaluateMaterialityUseCase, GenerateComprehensiveFinancialStatementsUseCase,
        GenerateFinancialStatementsUseCase, GenerateNoteDraftUseCase, GenerateTrialBalanceUseCase,
        LockClosingPeriodUseCase, PrepareClosingUseCase, VerifyLedgerConsistencyUseCase,
    },
    interactor::{
        AdjustAccountsInteractor, ApplyIfrsValuationInteractor, ConsolidateLedgerInteractor,
        GenerateFinancialStatementsInteractor, GenerateNoteDraftInteractor,
        GenerateTrialBalanceInteractor, LockClosingPeriodInteractor, PrepareClosingInteractor,
        closing::{
            EvaluateMaterialityInteractor, GenerateComprehensiveFinancialStatementsInteractor,
            VerifyLedgerConsistencyInteractor,
        },
    },
};
use javelin_infrastructure::{
    read::ledger::LedgerQueryServiceImpl, write::event_store::ClosingEventStore,
};

use crate::{
    error::AdapterResult,
    presenter::{
        ComprehensiveFinancialStatementsPresenter, LedgerConsistencyVerificationPresenter,
        LedgerPresenter, MaterialityEvaluationPresenter,
    },
};

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
    evaluate_materiality: Arc<EvaluateMaterialityInteractor>,
    verify_ledger_consistency: Arc<VerifyLedgerConsistencyInteractor>,
    generate_comprehensive_financial_statements:
        Arc<GenerateComprehensiveFinancialStatementsInteractor>,
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
        evaluate_materiality: Arc<EvaluateMaterialityInteractor>,
        verify_ledger_consistency: Arc<VerifyLedgerConsistencyInteractor>,
        generate_comprehensive_financial_statements: Arc<
            GenerateComprehensiveFinancialStatementsInteractor,
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
            evaluate_materiality,
            verify_ledger_consistency,
            generate_comprehensive_financial_statements,
        }
    }

    /// 元帳集約処理
    pub async fn consolidate_ledger(
        &self,
        request: ConsolidateLedgerRequest,
    ) -> AdapterResult<ConsolidateLedgerResponse> {
        self.consolidate_ledger.execute(request).await.map_err(
            |e: javelin_application::error::ApplicationError| {
                crate::error::AdapterError::ApplicationError(e)
            },
        )
    }

    /// 締準備処理
    pub async fn prepare_closing(
        &self,
        request: PrepareClosingRequest,
    ) -> AdapterResult<PrepareClosingResponse> {
        self.prepare_closing.execute(request).await.map_err(
            |e: javelin_application::error::ApplicationError| {
                crate::error::AdapterError::ApplicationError(e)
            },
        )
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

    /// 重要性判定処理
    pub async fn evaluate_materiality(
        &self,
        request: EvaluateMaterialityRequest,
    ) -> AdapterResult<EvaluateMaterialityResponse> {
        self.evaluate_materiality
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 元帳整合性検証処理
    pub async fn verify_ledger_consistency(
        &self,
        request: VerifyLedgerConsistencyRequest,
    ) -> AdapterResult<VerifyLedgerConsistencyResponse> {
        self.verify_ledger_consistency
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 包括的財務諸表生成処理
    pub async fn generate_comprehensive_financial_statements(
        &self,
        request: GenerateComprehensiveFinancialStatementsRequest,
    ) -> AdapterResult<GenerateComprehensiveFinancialStatementsResponse> {
        self.generate_comprehensive_financial_statements
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 重要性判定処理（Presenter経由）
    pub async fn handle_evaluate_materiality(
        &self,
        request: EvaluateMaterialityRequest,
        presenter: Arc<MaterialityEvaluationPresenter>,
    ) {
        presenter.notify_progress("重要性判定処理を開始します".to_string()).await;
        presenter.notify_progress("財務指標を取得中...".to_string()).await;
        presenter.notify_progress("金額的重要性を判定中...".to_string()).await;

        match self.evaluate_materiality.execute(request).await {
            Ok(response) => {
                presenter.notify_progress("重要性判定が完了しました".to_string()).await;
                presenter.present_result(response).await;
            }
            Err(e) => {
                presenter.notify_error(format!("重要性判定に失敗しました: {}", e)).await;
            }
        }
    }

    /// 元帳整合性検証処理（Presenter経由）
    pub async fn handle_verify_ledger_consistency(
        &self,
        request: VerifyLedgerConsistencyRequest,
        presenter: Arc<LedgerConsistencyVerificationPresenter>,
    ) {
        presenter.notify_progress("元帳整合性検証処理を開始します".to_string()).await;
        presenter.notify_progress("元帳データを取得中...".to_string()).await;
        presenter.notify_progress("基本整合性を検証中...".to_string()).await;

        match self.verify_ledger_consistency.execute(request).await {
            Ok(response) => {
                presenter.notify_progress("残高変動を分析中...".to_string()).await;
                presenter.notify_progress("異常値を検出中...".to_string()).await;
                presenter.notify_progress("仮勘定を分析中...".to_string()).await;
                presenter.notify_progress("元帳整合性検証が完了しました".to_string()).await;
                presenter.present_result(response).await;
            }
            Err(e) => {
                presenter.notify_error(format!("元帳整合性検証に失敗しました: {}", e)).await;
            }
        }
    }

    /// 包括的財務諸表生成処理（Presenter経由）
    pub async fn handle_generate_comprehensive_financial_statements(
        &self,
        request: GenerateComprehensiveFinancialStatementsRequest,
        presenter: Arc<ComprehensiveFinancialStatementsPresenter>,
    ) {
        presenter
            .notify_progress("包括的財務諸表生成処理を開始します".to_string())
            .await;
        presenter.notify_progress("元帳データを取得中...".to_string()).await;
        presenter.notify_progress("貸借対照表を生成中...".to_string()).await;

        match self.generate_comprehensive_financial_statements.execute(request).await {
            Ok(response) => {
                presenter.notify_progress("損益計算書を生成中...".to_string()).await;
                presenter.notify_progress("キャッシュフロー計算書を生成中...".to_string()).await;
                presenter.notify_progress("整合性を検証中...".to_string()).await;
                presenter.notify_progress("クロスチェックを実行中...".to_string()).await;
                presenter.notify_progress("包括的財務諸表生成が完了しました".to_string()).await;
                presenter.present_result(response).await;
            }
            Err(e) => {
                presenter.notify_error(format!("包括的財務諸表生成に失敗しました: {}", e)).await;
            }
        }
    }
}
