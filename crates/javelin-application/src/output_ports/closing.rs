/// ClosingOutputPort - 決算処理結果の出力
///
/// IFRS評価、勘定調整など決算処理関連のInteractorが使用する統一OutputPort。
/// 判断ログ、進捗通知、エラー通知を統合。
#[allow(async_fn_in_trait)]
pub trait ClosingOutputPort: Send + Sync {
    /// 判断ログを通知
    ///
    /// ECL（IFRS 9）、減損判定（IAS 36）、引当金（IAS 37）、棚卸資産評価（IAS 2）の
    /// 会計判断根拠（会計基準、計算モデル、前提条件、感度分析）をPresenter経由でビューに送信
    async fn notify_judgment_log(
        &self,
        judgment_type: String,
        accounting_standard: String,
        model_used: String,
        assumptions: Vec<String>,
        sensitivity_analysis: Vec<String>,
        timestamp: chrono::DateTime<chrono::Utc>,
    );

    /// 処理進捗を通知
    async fn notify_progress(&self, message: String);

    /// エラーを通知
    async fn notify_error(&self, error_message: String);
}
