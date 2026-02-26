// 取消ユースケース - Input Port
// 目的: 記帳済仕訳を取り消す

use crate::{dtos::ReverseJournalEntryRequest, error::ApplicationResult};

/// 取消ユースケース
#[allow(async_fn_in_trait)]
pub trait ReverseJournalEntryUseCase: Send + Sync {
    async fn execute(&self, request: ReverseJournalEntryRequest) -> ApplicationResult<()>;
}
