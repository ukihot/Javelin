// 仕訳検索結果DTO
// 検索結果をアダプター層へ転送

/// 仕訳検索結果DTO
///
/// 検索結果の仕訳リストと総件数を表現する。
#[derive(Debug, Clone)]
pub struct JournalEntrySearchResultDto {
    /// 検索結果の仕訳リスト
    pub entries: Vec<JournalEntryItemDto>,

    /// 総件数（ページネーション前の全体件数）
    pub total_count: u32,
}

impl JournalEntrySearchResultDto {
    /// 新しい検索結果DTOを作成
    pub fn new(entries: Vec<JournalEntryItemDto>, total_count: u32) -> Self {
        Self { entries, total_count }
    }

    /// 空の検索結果を作成
    pub fn empty() -> Self {
        Self { entries: Vec::new(), total_count: 0 }
    }
}

/// 仕訳項目DTO
///
/// 検索結果の1件の仕訳を表現する。
#[derive(Debug, Clone)]
pub struct JournalEntryItemDto {
    /// 仕訳ID
    pub entry_id: String,

    /// 伝票番号（記帳済の場合のみ）
    pub entry_number: Option<String>,

    /// 取引日付（YYYY-MM-DD形式）
    pub transaction_date: String,

    /// ステータス
    pub status: String,

    /// 仕訳明細リスト
    pub lines: Vec<JournalEntryLineItemDto>,
}

impl JournalEntryItemDto {
    /// 新しい仕訳項目DTOを作成
    pub fn new(
        entry_id: String,
        entry_number: Option<String>,
        transaction_date: String,
        status: String,
        lines: Vec<JournalEntryLineItemDto>,
    ) -> Self {
        Self { entry_id, entry_number, transaction_date, status, lines }
    }
}

/// 仕訳明細項目DTO
///
/// 検索結果の仕訳明細を表現する。
#[derive(Debug, Clone)]
pub struct JournalEntryLineItemDto {
    /// 明細行番号
    pub line_number: u32,

    /// 借方貸方区分（"Debit" | "Credit"）
    pub side: String,

    /// 勘定科目コード
    pub account_code: String,

    /// 勘定科目名
    pub account_name: String,

    /// 金額
    pub amount: f64,

    /// 摘要
    pub description: Option<String>,
}

impl JournalEntryLineItemDto {
    /// 新しい明細項目DTOを作成
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_entry_search_result_dto_creation() {
        let line1 = JournalEntryLineItemDto::new(
            1,
            "Debit".to_string(),
            "1000".to_string(),
            "現金".to_string(),
            100000.0,
            Some("売上入金".to_string()),
        );

        let line2 = JournalEntryLineItemDto::new(
            2,
            "Credit".to_string(),
            "4000".to_string(),
            "売上高".to_string(),
            100000.0,
            Some("商品販売".to_string()),
        );

        let entry = JournalEntryItemDto::new(
            "JE001".to_string(),
            Some("EN-2024-001".to_string()),
            "2024-01-01".to_string(),
            "Posted".to_string(),
            vec![line1, line2],
        );

        let result = JournalEntrySearchResultDto::new(vec![entry], 1);

        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.total_count, 1);
        assert_eq!(result.entries[0].entry_id, "JE001");
        assert_eq!(result.entries[0].entry_number, Some("EN-2024-001".to_string()));
        assert_eq!(result.entries[0].transaction_date, "2024-01-01");
        assert_eq!(result.entries[0].status, "Posted");
        assert_eq!(result.entries[0].lines.len(), 2);
    }

    #[test]
    fn test_empty_result() {
        let result = JournalEntrySearchResultDto::empty();

        assert_eq!(result.entries.len(), 0);
        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_journal_entry_line_item_dto() {
        let line = JournalEntryLineItemDto::new(
            1,
            "Debit".to_string(),
            "1000".to_string(),
            "現金".to_string(),
            50000.0,
            Some("テスト摘要".to_string()),
        );

        assert_eq!(line.line_number, 1);
        assert_eq!(line.side, "Debit");
        assert_eq!(line.account_code, "1000");
        assert_eq!(line.account_name, "現金");
        assert_eq!(line.amount, 50000.0);
        assert_eq!(line.description, Some("テスト摘要".to_string()));
    }
}
