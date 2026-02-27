// Read-side (Query side) - CQRS

pub mod infrastructure;

pub mod account_master;
pub mod application_settings_master;
pub mod batch_history;
pub mod company_master;
pub mod journal_entry;
pub mod ledger;
pub mod subsidiary_account_master;
