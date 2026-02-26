// 差戻しユースケース - Input Port
// 目的: 承認待ち仕訳を差し戻す

use crate::{dtos::RejectJournalEntryRequest, error::ApplicationResult};

/// 差戻しユースケース
#[allow(async_fn_in_trait)]
pub trait RejectJournalEntryUseCase: Send + Sync {
    async fn execute(&self, request: RejectJournalEntryRequest) -> ApplicationResult<()>;
}
