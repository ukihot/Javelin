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

    #[error("[D-9999] Unknown domain error: {0}")]
    Unknown(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
