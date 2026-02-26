// Adapter Layer Errors
// エラーコード: V-xxxx

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("[V-1001] Terminal initialization failed")]
    TerminalInitFailed(#[source] std::io::Error),

    #[error("[V-1002] Terminal cleanup failed")]
    TerminalCleanupFailed(#[source] std::io::Error),

    #[error("[V-1003] Failed to enable raw mode")]
    RawModeEnableFailed(#[source] std::io::Error),

    #[error("[V-1004] Failed to disable raw mode")]
    RawModeDisableFailed(#[source] std::io::Error),

    #[error("[V-2001] Frame rendering failed")]
    RenderingFailed(String),

    #[error("[V-2002] Event polling failed")]
    EventPollingFailed(#[source] std::io::Error),

    #[error("[V-2003] Event read failed")]
    EventReadFailed(#[source] std::io::Error),

    #[error("[V-3001] Input validation failed: {0}")]
    InputValidationFailed(String),

    #[error("[V-3002] DTO conversion failed: {0}")]
    DtoConversionFailed(String),

    #[error("[V-3003] Page not found: {0}")]
    PageNotFound(String),

    #[error("[V-3004] Page not implemented: {0}")]
    PageNotImplemented(String),

    #[error("[V-4001] Application error: {0}")]
    ApplicationError(#[from] javelin_application::error::ApplicationError),

    #[error("[V-9999] Unknown adapter error: {0}")]
    Unknown(String),
}

pub type AdapterResult<T> = Result<T, AdapterError>;
