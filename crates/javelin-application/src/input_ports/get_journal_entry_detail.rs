// GetJournalEntryDetailUseCase - 仕訳詳細取得ユースケース
// 仕訳IDを受け取り、詳細情報を取得する

use crate::error::ApplicationResult;

/// 仕訳詳細取得ユースケース
#[allow(async_fn_in_trait)]
pub trait GetJournalEntryDetailUseCase: Send + Sync {
    /// 仕訳詳細を取得
    ///
    /// # Arguments
    /// * `entry_id` - 仕訳ID
    ///
    /// # Returns
    /// * `Ok(())` - 取得成功（結果はOutputPort経由で通知）
    /// * `Err(ApplicationError)` - 取得失敗
    async fn execute(&self, entry_id: String) -> ApplicationResult<()>;
}
