// Application-wide Error
// Top-level error type for the entire application

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("[APP-1001] Application initialization failed")]
    InitializationFailed(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("[APP-1002] Failed to create data directory: {path}")]
    DataDirectoryCreationFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("[APP-1003] Feature not implemented: {0}")]
    NotImplemented(String),

    #[error("[APP-2001] Adapter error: {0}")]
    AdapterError(#[from] javelin_adapter::error::AdapterError),

    #[error("[APP-2002] Application error: {0}")]
    ApplicationError(#[from] javelin_application::error::ApplicationError),

    #[error("[APP-2003] Infrastructure error: {0}")]
    InfrastructureError(#[from] javelin_infrastructure::shared::error::InfrastructureError),

    #[error("[APP-9999] Unknown error: {0}")]
    Unknown(String),
}

pub type AppResult<T> = Result<T, AppError>;
