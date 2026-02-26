// JudgmentLog Entry - 会計判断の根拠記録DTO

use chrono::{DateTime, Utc};

/// 会計判断ログエントリ - IFRS評価処理の判断根拠を記録
#[derive(Debug, Clone)]
pub struct JudgmentLogEntry {
    /// 適用した会計基準（例：IFRS 9, IAS 36, IAS 37）
    pub accounting_standard: String,
    /// 使用した計算モデル
    pub model_used: String,
    /// 前提条件・仮定のリスト
    pub assumptions: Vec<String>,
    /// 感度分析結果
    pub sensitivity_analysis: Vec<String>,
    /// 判断実行者
    pub responsible_party: String,
    /// 記録日時
    pub timestamp: DateTime<Utc>,
}

impl JudgmentLogEntry {
    pub fn new(
        accounting_standard: String,
        model_used: String,
        assumptions: Vec<String>,
        sensitivity_analysis: Vec<String>,
        responsible_party: String,
    ) -> Self {
        Self {
            accounting_standard,
            model_used,
            assumptions,
            sensitivity_analysis,
            responsible_party,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judgment_log_entry_creation() {
        let entry = JudgmentLogEntry::new(
            "IFRS 9 Expected Credit Loss".to_string(),
            "Aging-based ECL method".to_string(),
            vec!["Current: 0.5% loss rate".to_string()],
            vec!["If loss rates +10%: 629k allowance".to_string()],
            "user1".to_string(),
        );

        assert_eq!(entry.accounting_standard, "IFRS 9 Expected Credit Loss");
        assert_eq!(entry.model_used, "Aging-based ECL method");
        assert_eq!(entry.assumptions.len(), 1);
        assert_eq!(entry.sensitivity_analysis.len(), 1);
    }
}
