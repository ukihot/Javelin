// Application Layer Errors
// エラーコード: A-xxxx

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("[A-1001] Use case execution failed: {0}")]
    UseCaseExecutionFailed(String),

    #[error("[A-1002] Validation failed: {0:?}")]
    ValidationFailed(Vec<String>),

    #[error("[A-1003] Validation error: {0}")]
    ValidationError(String),

    #[error("[A-2001] Query execution failed: {0}")]
    QueryExecutionFailed(String),

    #[error("[A-3001] Projection build failed: {0}")]
    ProjectionBuildFailed(String),

    #[error("[A-4001] Event store error: {0}")]
    EventStoreError(String),

    #[error("[A-4002] Projection database error: {0}")]
    ProjectionDatabaseError(String),

    #[error("[A-5001] Domain error: {0}")]
    DomainError(#[from] javelin_domain::error::DomainError),

    #[error("[A-9999] Unknown application error: {0}")]
    Unknown(String),
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;
