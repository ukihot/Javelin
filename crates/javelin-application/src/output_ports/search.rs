use crate::dtos::response::JournalEntrySearchResultDto;

/// SearchOutputPort - 仕訳検索結果の出力
pub trait SearchOutputPort: Send + Sync {
    /// 検索結果を出力
    fn present_search_result(&self, result: JournalEntrySearchResultDto);

    /// バリデーションエラーを出力
    fn present_validation_error(&self, message: String);

    /// 検索結果0件を出力
    fn present_no_results(&self);

    /// 進捗状況を出力
    fn present_progress(&self, message: String);

    /// 実行時間を出力（ミリ秒）
    fn present_execution_time(&self, elapsed_ms: usize);

    /// エラーを通知
    fn notify_error(&self, error_message: String);
}
