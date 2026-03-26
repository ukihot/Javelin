// 取引先集約
// 相手方の実体管理

pub mod domain_events;
pub mod domain_services;
pub mod entities;
pub mod values;

pub use domain_events::*;
pub use domain_services::*;
pub use entities::*;
pub use values::*;
