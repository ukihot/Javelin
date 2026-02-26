// リスク分類 - 第3章 3.2

/// リスク分類（第3章 3.2）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskClassification {
    Low,      // 定型処理
    Medium,   // 見積含有
    High,     // 予測依存
    Critical, // 経営判断
}

/// 会計判断区分
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JudgmentType {
    Estimate, // 見積処理
    Judgment, // 判断処理
    Routine,  // 定型処理
}

/// 承認階層
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalLevel {
    Staff,            // 担当者
    Manager,          // 管理職
    FinancialOfficer, // 財務責任者
    CFO,              // CFO
}

/// リスク分類に基づく承認階層の決定
pub fn determine_approval_level(risk: &RiskClassification) -> ApprovalLevel {
    match risk {
        RiskClassification::Low => ApprovalLevel::Staff,
        RiskClassification::Medium => ApprovalLevel::Manager,
        RiskClassification::High => ApprovalLevel::FinancialOfficer,
        RiskClassification::Critical => ApprovalLevel::CFO,
    }
}
