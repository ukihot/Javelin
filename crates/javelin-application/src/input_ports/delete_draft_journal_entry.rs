// 下書き削除ユースケース - Input Port
// 目的: 下書き状態の仕訳を削除する

use crate::{dtos::DeleteDraftJournalEntryRequest, error::ApplicationResult};

/// 下書き削除ユースケース
#[allow(async_fn_in_trait)]
pub trait DeleteDraftJournalEntryUseCase: Send + Sync {
    async fn execute(&self, request: DeleteDraftJournalEntryRequest) -> ApplicationResult<()>;
}
