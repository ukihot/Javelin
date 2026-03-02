// 収益認識モジュール（IFRS 15準拠）

pub mod entities;
pub mod events;
pub mod services;
pub mod values;

pub use entities::{Contract, PerformanceObligation};
pub use events::{RevenueRecognitionEvent, RevenueRecognitionEventType};
pub use services::RevenueRecognitionService;
pub use values::{
    ContractId, ContractStatus, PerformanceObligationId, RecognitionPattern, RecognitionTiming,
    StandaloneSellingPrice, TransactionPrice, VariableConsiderationMethod,
};
