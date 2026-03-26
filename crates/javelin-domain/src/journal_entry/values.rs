// JournalEntry Values - 仕訳伝票値オブジェクト
//
// 公開範囲：
// - 外部との境界で使用される値オブジェクト：pub use で再エクスポート
// - 集約内で使用される値オブジェクト：実装詳細のため pub use は制限しない
//   （ただし、集約外からの直接使用は想定されない）

// 外部公開：ルート集約のgetterで返される値オブジェクト
pub mod debit_credit;
pub mod entry_number;
pub mod journal_entry_type;
pub mod journal_status;
pub mod transaction_date;
pub mod user_id;
pub mod valuation;
pub mod voucher_number;

// 集約内で使用：他モジュール（domain_events等）からアクセスも許可
pub mod department_code;
pub mod description;
pub mod line_number;
pub mod sub_account_code;
pub mod tax_type;

// Re-exports: すべての値オブジェクトを公開
// （外部からの直接使用は想定されないが、集約内の複数モジュールからのアクセスを許可）
pub use debit_credit::*;
pub use department_code::*;
pub use description::*;
pub use entry_number::*;
pub use journal_entry_type::*;
pub use journal_status::{PeriodStatus, *};
pub use line_number::*;
pub use sub_account_code::*;
pub use tax_type::*;
pub use transaction_date::*;
pub use user_id::*;
pub use valuation::*;
pub use voucher_number::*;

// Money と Currency は common から再エクスポート
pub use crate::common::{Currency, Money};
