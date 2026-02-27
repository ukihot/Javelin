// 仕訳検索用ReadModel
// 検索最適化されたデータ構造

use serde::{Deserialize, Serialize};

/// 仕訳検索用ReadModel
///
/// 検索に最適化されたデータ構造。
/// 取引日付、勘定科目、摘要などでインデックス化される。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JournalEntrySearchReadModel {
    pub entry_id: String,
    pub entry_number: Option<String>,
    pub transaction_date: String, // YYYY-MM-DD形式
    pub status: String,
    pub lines: Vec<JournalEntryLineReadModel>,
}

/// 仕訳明細検索用ReadModel
///
/// 検索に最適化された明細データ構造。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JournalEntryLineReadModel {
    pub line_number: u32,
    pub side: String, // "Debit" or "Credit"
    pub account_code: String,
    pub account_name: String, // マスタデータから取得
    pub amount: f64,
    pub description: Option<String>,
}

impl JournalEntrySearchReadModel {
    /// 新しいReadModelインスタンスを作成
    pub fn new(
        entry_id: String,
        entry_number: Option<String>,
        transaction_date: String,
        status: String,
        lines: Vec<JournalEntryLineReadModel>,
    ) -> Self {
        Self { entry_id, entry_number, transaction_date, status, lines }
    }

    /// 取引日付を取得
    pub fn transaction_date(&self) -> &str {
        &self.transaction_date
    }

    /// ステータスを取得
    pub fn status(&self) -> &str {
        &self.status
    }

    /// 明細リストを取得
    pub fn lines(&self) -> &[JournalEntryLineReadModel] {
        &self.lines
    }

    /// 指定された勘定科目を含むかチェック
    pub fn contains_account(&self, account_code: &str) -> bool {
        self.lines.iter().any(|line| line.account_code == account_code)
    }

    /// 指定された摘要を含むかチェック（大文字小文字非区別）
    pub fn contains_description(&self, search_text: &str) -> bool {
        let search_lower = search_text.to_lowercase();
        self.lines.iter().any(|line| {
            line.description
                .as_ref()
                .map(|desc| desc.to_lowercase().contains(&search_lower))
                .unwrap_or(false)
        })
    }

    /// 指定された借方貸方区分の明細を含むかチェック
    pub fn contains_side(&self, side: &str) -> bool {
        self.lines.iter().any(|line| line.side == side)
    }

    /// 指定された金額範囲の明細を含むかチェック
    pub fn contains_amount_in_range(&self, min: Option<f64>, max: Option<f64>) -> bool {
        self.lines.iter().any(|line| {
            let amount = line.amount;
            let min_ok = min.map(|m| amount >= m).unwrap_or(true);
            let max_ok = max.map(|m| amount <= m).unwrap_or(true);
            min_ok && max_ok
        })
    }
}

impl JournalEntryLineReadModel {
    /// 新しい明細ReadModelインスタンスを作成
    pub fn new(
        line_number: u32,
        side: String,
        account_code: String,
        account_name: String,
        amount: f64,
        description: Option<String>,
    ) -> Self {
        Self { line_number, side, account_code, account_name, amount, description }
    }

    /// 借方貸方区分を取得
    pub fn side(&self) -> &str {
        &self.side
    }

    /// 勘定科目コードを取得
    pub fn account_code(&self) -> &str {
        &self.account_code
    }

    /// 金額を取得
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// 摘要を取得
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_entry_search_read_model_creation() {
        let lines = vec![
            JournalEntryLineReadModel::new(
                1,
                "Debit".to_string(),
                "1000".to_string(),
                "現金".to_string(),
                100000.0,
                Some("売上入金".to_string()),
            ),
            JournalEntryLineReadModel::new(
                2,
                "Credit".to_string(),
                "4000".to_string(),
                "売上高".to_string(),
                100000.0,
                Some("商品販売".to_string()),
            ),
        ];

        let model = JournalEntrySearchReadModel::new(
            "JE001".to_string(),
            Some("EN-2024-001".to_string()),
            "2024-01-01".to_string(),
            "Posted".to_string(),
            lines,
        );

        assert_eq!(model.entry_id, "JE001");
        assert_eq!(model.entry_number, Some("EN-2024-001".to_string()));
        assert_eq!(model.transaction_date, "2024-01-01");
        assert_eq!(model.status, "Posted");
        assert_eq!(model.lines.len(), 2);
    }

    #[test]
    fn test_contains_account() {
        let lines = vec![
            JournalEntryLineReadModel::new(
                1,
                "Debit".to_string(),
                "1000".to_string(),
                "現金".to_string(),
                100000.0,
                None,
            ),
            JournalEntryLineReadModel::new(
                2,
                "Credit".to_string(),
                "4000".to_string(),
                "売上高".to_string(),
                100000.0,
                None,
            ),
        ];

        let model = JournalEntrySearchReadModel::new(
            "JE001".to_string(),
            None,
            "2024-01-01".to_string(),
            "Draft".to_string(),
            lines,
        );

        assert!(model.contains_account("1000"));
        assert!(model.contains_account("4000"));
        assert!(!model.contains_account("2000"));
    }

    #[test]
    fn test_contains_description_case_insensitive() {
        let lines = vec![JournalEntryLineReadModel::new(
            1,
            "Debit".to_string(),
            "1000".to_string(),
            "現金".to_string(),
            100000.0,
            Some("売上入金".to_string()),
        )];

        let model = JournalEntrySearchReadModel::new(
            "JE001".to_string(),
            None,
            "2024-01-01".to_string(),
            "Draft".to_string(),
            lines,
        );

        assert!(model.contains_description("売上"));
        assert!(model.contains_description("入金"));
        assert!(model.contains_description("売上入金"));
        assert!(!model.contains_description("仕入"));
    }

    #[test]
    fn test_contains_side() {
        let lines = vec![
            JournalEntryLineReadModel::new(
                1,
                "Debit".to_string(),
                "1000".to_string(),
                "現金".to_string(),
                100000.0,
                None,
            ),
            JournalEntryLineReadModel::new(
                2,
                "Credit".to_string(),
                "4000".to_string(),
                "売上高".to_string(),
                100000.0,
                None,
            ),
        ];

        let model = JournalEntrySearchReadModel::new(
            "JE001".to_string(),
            None,
            "2024-01-01".to_string(),
            "Draft".to_string(),
            lines,
        );

        assert!(model.contains_side("Debit"));
        assert!(model.contains_side("Credit"));
    }

    #[test]
    fn test_contains_amount_in_range() {
        let lines = vec![
            JournalEntryLineReadModel::new(
                1,
                "Debit".to_string(),
                "1000".to_string(),
                "現金".to_string(),
                50000.0,
                None,
            ),
            JournalEntryLineReadModel::new(
                2,
                "Credit".to_string(),
                "4000".to_string(),
                "売上高".to_string(),
                50000.0,
                None,
            ),
        ];

        let model = JournalEntrySearchReadModel::new(
            "JE001".to_string(),
            None,
            "2024-01-01".to_string(),
            "Draft".to_string(),
            lines,
        );

        // 範囲内
        assert!(model.contains_amount_in_range(Some(40000.0), Some(60000.0)));
        // 最小値のみ
        assert!(model.contains_amount_in_range(Some(40000.0), None));
        // 最大値のみ
        assert!(model.contains_amount_in_range(None, Some(60000.0)));
        // 範囲外
        assert!(!model.contains_amount_in_range(Some(60000.0), Some(70000.0)));
    }
}
