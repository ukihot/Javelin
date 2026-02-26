// 仕訳検索条件DTO
// 検索条件を構造化されたデータとして転送

/// 仕訳検索条件DTO
///
/// ユーザーが指定する検索条件を表現する。
/// すべてのフィールドはOptionalで、未指定の場合は条件として使用しない。
#[derive(Debug, Clone)]
pub struct SearchCriteriaDto {
    /// 取引日付範囲 - 開始日付（YYYY-MM-DD形式）
    pub from_date: Option<String>,

    /// 取引日付範囲 - 終了日付（YYYY-MM-DD形式）
    pub to_date: Option<String>,

    /// 摘要検索（部分一致、大文字小文字区別なし）
    pub description: Option<String>,

    /// 勘定科目コード
    pub account_code: Option<String>,

    /// 借方貸方区分（"Debit" | "Credit" | None(両方)）
    pub debit_credit: Option<String>,

    /// 金額範囲 - 最小金額
    pub min_amount: Option<f64>,

    /// 金額範囲 - 最大金額
    pub max_amount: Option<f64>,

    /// ページネーション - 取得件数上限（デフォルト100）
    pub limit: Option<u32>,

    /// ページネーション - オフセット（デフォルト0）
    pub offset: Option<u32>,
}

impl SearchCriteriaDto {
    /// 新しい検索条件DTOを作成
    pub fn new() -> Self {
        Self {
            from_date: None,
            to_date: None,
            description: None,
            account_code: None,
            debit_credit: None,
            min_amount: None,
            max_amount: None,
            limit: Some(100),
            offset: Some(0),
        }
    }

    /// ビルダーパターン: 開始日付を設定
    pub fn with_from_date(mut self, from_date: String) -> Self {
        self.from_date = Some(from_date);
        self
    }

    /// ビルダーパターン: 終了日付を設定
    pub fn with_to_date(mut self, to_date: String) -> Self {
        self.to_date = Some(to_date);
        self
    }

    /// ビルダーパターン: 摘要を設定
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// ビルダーパターン: 勘定科目コードを設定
    pub fn with_account_code(mut self, account_code: String) -> Self {
        self.account_code = Some(account_code);
        self
    }

    /// ビルダーパターン: 借方貸方区分を設定
    pub fn with_debit_credit(mut self, debit_credit: String) -> Self {
        self.debit_credit = Some(debit_credit);
        self
    }

    /// ビルダーパターン: 最小金額を設定
    pub fn with_min_amount(mut self, min_amount: f64) -> Self {
        self.min_amount = Some(min_amount);
        self
    }

    /// ビルダーパターン: 最大金額を設定
    pub fn with_max_amount(mut self, max_amount: f64) -> Self {
        self.max_amount = Some(max_amount);
        self
    }

    /// ビルダーパターン: 取得件数上限を設定
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// ビルダーパターン: オフセットを設定
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// すべての検索条件が未指定かチェック
    pub fn is_empty(&self) -> bool {
        self.from_date.is_none()
            && self.to_date.is_none()
            && self.description.is_none()
            && self.account_code.is_none()
            && self.debit_credit.is_none()
            && self.min_amount.is_none()
            && self.max_amount.is_none()
    }
}

impl Default for SearchCriteriaDto {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_criteria_dto_creation() {
        let criteria = SearchCriteriaDto::new();

        assert!(criteria.from_date.is_none());
        assert!(criteria.to_date.is_none());
        assert!(criteria.description.is_none());
        assert!(criteria.account_code.is_none());
        assert!(criteria.debit_credit.is_none());
        assert!(criteria.min_amount.is_none());
        assert!(criteria.max_amount.is_none());
        assert_eq!(criteria.limit, Some(100));
        assert_eq!(criteria.offset, Some(0));
    }

    #[test]
    fn test_search_criteria_dto_builder() {
        let criteria = SearchCriteriaDto::new()
            .with_from_date("2024-01-01".to_string())
            .with_to_date("2024-12-31".to_string())
            .with_description("売上".to_string())
            .with_account_code("1000".to_string())
            .with_debit_credit("Debit".to_string())
            .with_min_amount(10000.0)
            .with_max_amount(100000.0)
            .with_limit(50)
            .with_offset(10);

        assert_eq!(criteria.from_date, Some("2024-01-01".to_string()));
        assert_eq!(criteria.to_date, Some("2024-12-31".to_string()));
        assert_eq!(criteria.description, Some("売上".to_string()));
        assert_eq!(criteria.account_code, Some("1000".to_string()));
        assert_eq!(criteria.debit_credit, Some("Debit".to_string()));
        assert_eq!(criteria.min_amount, Some(10000.0));
        assert_eq!(criteria.max_amount, Some(100000.0));
        assert_eq!(criteria.limit, Some(50));
        assert_eq!(criteria.offset, Some(10));
    }

    #[test]
    fn test_is_empty() {
        let empty_criteria = SearchCriteriaDto::new();
        assert!(empty_criteria.is_empty());

        let non_empty_criteria = SearchCriteriaDto::new().with_from_date("2024-01-01".to_string());
        assert!(!non_empty_criteria.is_empty());
    }
}
