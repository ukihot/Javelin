// 下書き仕訳更新ユースケース - Input Port
// 目的: 下書き状態の仕訳を更新する

use crate::{dtos::UpdateDraftJournalEntryRequest, error::ApplicationResult};

/// 下書き仕訳更新ユースケース
#[allow(async_fn_in_trait)]
pub trait UpdateDraftJournalEntryUseCase: Send + Sync {
    async fn execute(&self, request: UpdateDraftJournalEntryRequest) -> ApplicationResult<()>;
}
