/// AdjustAccountsOutputPort - 勘定調整結果の出力
///
/// 勘定科目の調整処理結果とそのプロセスを通知するOutputPort。
#[allow(async_fn_in_trait)]
pub trait AdjustAccountsOutputPort: Send + Sync {
    /// 調整処理結果を通知
    async fn notify_adjustment_result(&self, result_message: String);

    /// 処理進捗を通知
    async fn notify_progress(&self, message: String);

    /// エラーを通知
    async fn notify_error(&self, error_message: String);
}
