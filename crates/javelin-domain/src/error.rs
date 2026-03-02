// Domain Layer Errors
// エラーコード: D-xxxx

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("[D-1001] Invalid accounting period: month must be between 1 and 12")]
    InvalidAccountingPeriod,

    #[error("[D-1002] Invalid account code: code cannot be empty")]
    InvalidAccountCode,

    #[error("[D-1003] Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("[D-1004] Validation error: {0}")]
    ValidationError(String),

    #[error("[D-2001] Journal entry validation failed: debit and credit must match")]
    JournalEntryValidationFailed,

    #[error("[D-2002] Evidence reference is required")]
    EvidenceRequired,

    #[error("[D-2003] Invalid status transition")]
    InvalidStatusTransition,

    #[error("[D-3001] Entity not found: {0}")]
    EntityNotFound(String),

    #[error("[D-3002] Not found: {0}")]
    NotFound(String),

    #[error("[D-3003] Aggregate version conflict")]
    VersionConflict,

    #[error("[D-4001] Serialization failed: {0}")]
    SerializationFailed(String),

    #[error("[D-4002] Repository error: {0}")]
    RepositoryError(String),

    // Fixed Assets errors (D-5xxx)
    #[error("[D-5001] Invalid asset category")]
    InvalidAssetCategory,

    #[error("[D-5002] Invalid measurement model")]
    InvalidMeasurementModel,

    #[error("[D-5003] Invalid depreciation method")]
    InvalidDepreciationMethod,

    #[error("[D-5004] Invalid useful life")]
    InvalidUsefulLife,

    #[error("[D-5005] Invalid asset status")]
    InvalidAssetStatus,

    #[error("[D-5006] Invalid acquisition date")]
    InvalidAcquisitionDate,

    #[error("[D-5007] Invalid asset name")]
    InvalidAssetName,

    #[error("[D-5008] Invalid acquisition cost")]
    InvalidAcquisitionCost,

    #[error("[D-5009] Invalid component name")]
    InvalidComponentName,

    #[error("[D-5010] Invalid component cost")]
    InvalidComponentCost,

    #[error("[D-5011] Invalid residual value")]
    InvalidResidualValue,

    #[error("[D-5012] Duplicate component")]
    DuplicateComponent,

    #[error("[D-5013] Revaluation not allowed for cost model")]
    RevaluationNotAllowed,

    #[error("[D-5014] Invalid revaluation amount")]
    InvalidRevaluationAmount,

    #[error("[D-5015] Invalid impairment loss")]
    InvalidImpairmentLoss,

    #[error("[D-5016] Invalid impairment reversal")]
    InvalidImpairmentReversal,

    #[error("[D-5017] Excessive impairment reversal")]
    ExcessiveImpairmentReversal,

    #[error("[D-5018] Cannot change disposed asset status")]
    CannotChangeDisposedAssetStatus,

    #[error("[D-5019] Invalid depreciation amount")]
    InvalidDepreciationAmount,

    #[error("[D-5020] Excessive depreciation")]
    ExcessiveDepreciation,

    #[error(
        "[D-5021] Ledger inconsistency: asset carrying amount {asset_carrying_amount}, ledger balance {ledger_balance}"
    )]
    LedgerInconsistency { asset_carrying_amount: i64, ledger_balance: i64 },

    #[error("[D-5022] Unsupported depreciation method")]
    UnsupportedDepreciationMethod,

    #[error("[D-5023] Invalid discount rate")]
    InvalidDiscountRate,

    #[error("[D-5024] Invalid recoverable amount")]
    InvalidRecoverableAmount,

    // Revenue Recognition errors (D-6xxx)
    #[error("[D-6001] Invalid contract")]
    InvalidContract,

    #[error("[D-6002] Invalid performance obligation")]
    InvalidPerformanceObligation,

    #[error("[D-6003] Invalid transaction price")]
    InvalidTransactionPrice,

    #[error("[D-6004] Invalid standalone selling price")]
    InvalidStandaloneSellingPrice,

    #[error("[D-6005] Invalid revenue recognition pattern")]
    InvalidRevenueRecognitionPattern,

    // Foreign Currency errors (D-7xxx)
    #[error("[D-7001] Invalid functional currency")]
    InvalidFunctionalCurrency,

    #[error("[D-7002] Invalid exchange rate")]
    InvalidExchangeRate,

    #[error("[D-7003] Invalid monetary classification")]
    InvalidMonetaryClassification,

    // Carrying Amount errors (D-8xxx)
    #[error("[D-8001] Invalid carrying amount")]
    InvalidCarryingAmount,

    #[error("[D-8002] Invalid measurement basis")]
    InvalidMeasurementBasis,

    #[error("[D-8003] Invalid component type")]
    InvalidComponentType,

    #[error("[D-8004] Invalid measurement component")]
    InvalidMeasurementComponent,

    #[error("[D-8005] Invalid measurement change")]
    InvalidMeasurementChange,

    #[error("[D-8006] Invalid estimate change")]
    InvalidEstimateChange,

    #[error("[D-8007] Invalid presentation amount")]
    InvalidPresentationAmount,

    // Judgment Log errors (D-9xxx)
    #[error("[D-9001] Invalid judgment log")]
    InvalidJudgmentLog,

    #[error("[D-9002] Invalid calculation version")]
    InvalidCalculationVersion,

    // Management Accounting errors (D-10xxx)
    #[error("[D-10001] Invalid management accounting")]
    InvalidManagementAccounting,

    #[error("[D-10002] Invalid conversion")]
    InvalidConversion,

    #[error("[D-10003] Invalid KPI calculation")]
    InvalidKpiCalculation,

    // Materiality errors (D-11xxx)
    #[error("[D-11001] Invalid materiality judgment")]
    InvalidMateriality,

    #[error("[D-9999] Unknown domain error: {0}")]
    Unknown(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
