// Javelin Domain Layer
// ドメイン駆動設計に基づく集約構造

// 共通モジュール
pub mod common;
pub mod error;
pub mod event;

// 基本トレイト（全集約で使用）
pub mod entity;
pub mod value_object;

// 集約モジュール
pub mod accounting_period_ledger;
pub mod balance_tracking;
pub mod billing;
pub mod cash_flow;
pub mod chart_of_accounts;
pub mod company;
pub mod compliance_risk;
pub mod evidence;
pub mod journal_entry;
pub mod partner;
pub mod reconciliation;

// Re-exports
pub use common::*;
pub use error::*;
pub use event::*;
