// Javelin - Main Application Crate
// Orchestrates all layers

pub mod app;
pub mod app_builder;
pub mod app_error;
pub mod app_resolver;
pub mod app_setup;

// Re-export all layers for convenience
pub use javelin_adapter as adapter;
pub use javelin_application as application;
pub use javelin_domain as domain;
pub use javelin_infrastructure as infrastructure;
