// 請求書印刷ユースケース結果の出力ポート

use std::future::Future;

/// InvoicePrintOutputPort - 請求書印刷ユースケース結果の出力
pub trait InvoicePrintOutputPort: Send + Sync {
    /// 印刷開始を通知
    fn notify_print_started(&self) -> impl Future<Output = ()> + Send;

    /// 印刷成功を通知（ファイルパスを含む）
    fn notify_print_success(&self, file_path: String) -> impl Future<Output = ()> + Send;

    /// 印刷エラーを通知
    fn notify_print_error(&self, error_message: String) -> impl Future<Output = ()> + Send;

    /// 処理進捗を通知
    fn notify_progress(&self, message: String) -> impl Future<Output = ()> + Send;
}
