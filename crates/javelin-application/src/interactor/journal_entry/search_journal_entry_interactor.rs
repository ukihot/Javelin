// SearchJournalEntryInteractor - 仕訳検索Interactor
// 検索条件のバリデーションとQueryService呼び出し

use std::sync::Arc;

use crate::{
    dtos::request::SearchCriteriaDto,
    error::{ApplicationError, ApplicationResult},
    input_ports::SearchJournalEntryUseCase,
    output_ports::SearchOutputPort,
    query_service::JournalEntrySearchQueryService,
};

/// 仕訳検索Interactor
///
/// 検索条件のバリデーションを行い、QueryServiceを呼び出して
/// 検索結果をOutputPortに通知する。
pub struct SearchJournalEntryInteractor<Q, O>
where
    Q: JournalEntrySearchQueryService,
    O: SearchOutputPort,
{
    query_service: Arc<Q>,
    output_port: Arc<O>,
}

impl<Q, O> SearchJournalEntryInteractor<Q, O>
where
    Q: JournalEntrySearchQueryService,
    O: SearchOutputPort,
{
    /// 新しいインスタンスを作成
    pub fn new(query_service: Arc<Q>, output_port: Arc<O>) -> Self {
        Self { query_service, output_port }
    }

    /// 検索条件のバリデーション
    fn validate_criteria(&self, criteria: &SearchCriteriaDto) -> ApplicationResult<()> {
        // 日付範囲検証
        if let (Some(from), Some(to)) = (&criteria.from_date, &criteria.to_date)
            && from > to
        {
            return Err(ApplicationError::ValidationError(
                "開始日付は終了日付以前である必要があります".to_string(),
            ));
        }

        // 日付形式検証（YYYY-MM-DD形式）
        if let Some(from_date) = &criteria.from_date
            && !Self::is_valid_date_format(from_date)
        {
            return Err(ApplicationError::ValidationError(
                "日付形式が不正です: YYYY-MM-DD形式で入力してください".to_string(),
            ));
        }
        if let Some(to_date) = &criteria.to_date
            && !Self::is_valid_date_format(to_date)
        {
            return Err(ApplicationError::ValidationError(
                "日付形式が不正です: YYYY-MM-DD形式で入力してください".to_string(),
            ));
        }

        // 金額範囲検証
        if let Some(min) = criteria.min_amount
            && min < 0.0
        {
            return Err(ApplicationError::ValidationError(
                "最小金額は0以上である必要があります".to_string(),
            ));
        }

        if let Some(max) = criteria.max_amount
            && max < 0.0
        {
            return Err(ApplicationError::ValidationError(
                "最大金額は0以上である必要があります".to_string(),
            ));
        }

        if let (Some(min), Some(max)) = (criteria.min_amount, criteria.max_amount)
            && min > max
        {
            return Err(ApplicationError::ValidationError(
                "最小金額は最大金額以下である必要があります".to_string(),
            ));
        }

        Ok(())
    }

    /// 日付形式が正しいかチェック（YYYY-MM-DD形式）
    fn is_valid_date_format(date_str: &str) -> bool {
        // 簡易的な形式チェック
        if date_str.len() != 10 {
            return false;
        }

        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return false;
        }

        // 年月日が数値かチェック（モダンプラクティス: is_ok_and使用）
        parts[0].parse::<u32>().is_ok()
            && parts[1].parse::<u32>().is_ok()
            && parts[2].parse::<u32>().is_ok()
    }
}

impl<Q, O> SearchJournalEntryUseCase for SearchJournalEntryInteractor<Q, O>
where
    Q: JournalEntrySearchQueryService,
    O: SearchOutputPort,
{
    async fn execute(&self, criteria: SearchCriteriaDto) -> ApplicationResult<()> {
        // 開始時刻を記録
        let start_time = std::time::Instant::now();

        // 進捗報告: 検索開始
        self.output_port.present_progress("検索条件を検証中...".to_string());

        // バリデーション
        self.validate_criteria(&criteria).inspect_err(|e| {
            self.output_port.present_validation_error(e.to_string());
        })?;

        // 進捗報告: 検索実行
        self.output_port.present_progress("仕訳データを検索中...".to_string());

        // 検索実行
        self.query_service
            .search(criteria)
            .await
            .map(|result| {
                // 進捗報告: 結果処理
                self.output_port
                    .present_progress(format!("{}件の仕訳を取得しました", result.total_count));

                if result.total_count == 0 {
                    self.output_port.present_no_results();
                } else {
                    self.output_port.present_search_result(result);
                }
            })
            .inspect_err(|e| {
                self.output_port
                    .present_validation_error(format!("検索処理中にエラーが発生しました: {}", e));
            })?;

        // 実行時間を計測して出力
        let elapsed = start_time.elapsed();
        let elapsed_ms = elapsed.as_millis() as usize;
        self.output_port.present_execution_time(elapsed_ms);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_date_format() {
        assert!(
            SearchJournalEntryInteractor::<MockQueryService, MockOutputPort>::is_valid_date_format(
                "2024-01-01"
            )
        );
        assert!(
            SearchJournalEntryInteractor::<MockQueryService, MockOutputPort>::is_valid_date_format(
                "2024-12-31"
            )
        );

        assert!(
            !SearchJournalEntryInteractor::<MockQueryService, MockOutputPort>::is_valid_date_format(
                "2024/01/01"
            )
        );
        assert!(
            !SearchJournalEntryInteractor::<MockQueryService, MockOutputPort>::is_valid_date_format(
                "2024-1-1"
            )
        );
        assert!(
            !SearchJournalEntryInteractor::<MockQueryService, MockOutputPort>::is_valid_date_format(
                "invalid"
            )
        );
    }

    // Mock implementations for testing
    struct MockQueryService;
    impl JournalEntrySearchQueryService for MockQueryService {
        async fn search(
            &self,
            _criteria: SearchCriteriaDto,
        ) -> ApplicationResult<crate::dtos::response::JournalEntrySearchResultDto> {
            Ok(crate::dtos::response::JournalEntrySearchResultDto::empty())
        }

        async fn get_voucher_numbers_by_fiscal_year(
            &self,
            _fiscal_year: u32,
        ) -> ApplicationResult<Vec<String>> {
            Ok(vec![])
        }

        async fn get_detail(
            &self,
            _entry_id: &str,
        ) -> ApplicationResult<Option<crate::dtos::response::JournalEntryDetail>> {
            Ok(None)
        }
    }

    struct MockOutputPort;
    impl SearchOutputPort for MockOutputPort {
        fn present_search_result(
            &self,
            _result: crate::dtos::response::JournalEntrySearchResultDto,
        ) {
        }
        fn present_validation_error(&self, _message: String) {}
        fn present_no_results(&self) {}
        fn present_progress(&self, _message: String) {}
        fn present_execution_time(&self, _elapsed_ms: usize) {}

        fn notify_error(&self, _error_message: String) {}
    }

    #[test]
    fn test_validate_criteria_date_range() {
        let query_service = Arc::new(MockQueryService);
        let output_port = Arc::new(MockOutputPort);
        let interactor = SearchJournalEntryInteractor::new(query_service, output_port);

        // 正常な日付範囲
        let criteria = SearchCriteriaDto::new()
            .with_from_date("2024-01-01".to_string())
            .with_to_date("2024-12-31".to_string());
        assert!(interactor.validate_criteria(&criteria).is_ok());

        // 不正な日付範囲（開始 > 終了）
        let criteria = SearchCriteriaDto::new()
            .with_from_date("2024-12-31".to_string())
            .with_to_date("2024-01-01".to_string());
        assert!(interactor.validate_criteria(&criteria).is_err());
    }

    #[test]
    fn test_validate_criteria_amount_range() {
        let query_service = Arc::new(MockQueryService);
        let output_port = Arc::new(MockOutputPort);
        let interactor = SearchJournalEntryInteractor::new(query_service, output_port);

        // 正常な金額範囲
        let criteria = SearchCriteriaDto::new().with_min_amount(10000.0).with_max_amount(100000.0);
        assert!(interactor.validate_criteria(&criteria).is_ok());

        // 負の最小金額
        let criteria = SearchCriteriaDto::new().with_min_amount(-1000.0);
        assert!(interactor.validate_criteria(&criteria).is_err());

        // 負の最大金額
        let criteria = SearchCriteriaDto::new().with_max_amount(-1000.0);
        assert!(interactor.validate_criteria(&criteria).is_err());

        // 不正な金額範囲（最小 > 最大）
        let criteria = SearchCriteriaDto::new().with_min_amount(100000.0).with_max_amount(10000.0);
        assert!(interactor.validate_criteria(&criteria).is_err());
    }

    #[test]
    fn test_validate_criteria_date_format() {
        let query_service = Arc::new(MockQueryService);
        let output_port = Arc::new(MockOutputPort);
        let interactor = SearchJournalEntryInteractor::new(query_service, output_port);

        // 正常な日付形式
        let criteria = SearchCriteriaDto::new().with_from_date("2024-01-01".to_string());
        assert!(interactor.validate_criteria(&criteria).is_ok());

        // 不正な日付形式
        let criteria = SearchCriteriaDto::new().with_from_date("2024/01/01".to_string());
        assert!(interactor.validate_criteria(&criteria).is_err());

        let criteria = SearchCriteriaDto::new().with_from_date("invalid".to_string());
        assert!(interactor.validate_criteria(&criteria).is_err());
    }
}
