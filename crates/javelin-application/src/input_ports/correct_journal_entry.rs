// 修正ユースケース - Input Port
// 目的: 取消済仕訳を修正する

use crate::{dtos::CorrectJournalEntryRequest, error::ApplicationResult};

/// 修正ユースケース
#[allow(async_fn_in_trait)]
pub trait CorrectJournalEntryUseCase: Send + Sync {
    async fn execute(&self, request: CorrectJournalEntryRequest) -> ApplicationResult<()>;
}
