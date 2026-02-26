// リスク分類 - 第3章 3.2

/// 承認階層
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalLevel {
    Staff,            // 担当者
    Manager,          // 管理職
    FinancialOfficer, // 財務責任者
    CFO,              // CFO
}

/// リスク分類に基づく承認階層の決定
pub fn determine_approval_level(risk: &super::journal_entry::RiskClassification) -> ApprovalLevel {
    use super::journal_entry::RiskClassification;

    match risk {
        RiskClassification::Low => ApprovalLevel::Staff,
        RiskClassification::Medium => ApprovalLevel::Manager,
        RiskClassification::High => ApprovalLevel::FinancialOfficer,
        RiskClassification::Critical => ApprovalLevel::CFO,
    }
}
