// 財務諸表生成（Financial Statements Generation）
//
// 表示と測定の分離原則に基づき、財務諸表を生成する。
// 貸借対照表、損益計算書、包括利益計算書、株主資本等変動計算書、
// キャッシュフロー計算書の生成と整合性検証を提供。

pub mod entities;
pub mod events;
pub mod services;
pub mod values;

pub use entities::FinancialStatement;
pub use events::{FinancialStatementEvent, FinancialStatementEventType};
pub use services::{ConsistencyReport, CrossCheckReport, FinancialStatementService, Inconsistency};
pub use values::{
    CashFlowClassification, CurrentNonCurrentClassification, ExpenseClassification,
    FinancialStatementId, FinancialStatementItem, FinancialStatementType,
    OtherComprehensiveIncomeItem,
};
