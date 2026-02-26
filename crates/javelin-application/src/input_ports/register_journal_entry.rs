// 仕訳登録ユースケース - Input Port
// 目的: 仕訳を下書きとして登録する

use crate::{dtos::RegisterJournalEntryRequest, error::ApplicationResult};

/// 仕訳登録ユースケース
#[allow(async_fn_in_trait)]
pub trait RegisterJournalEntryUseCase: Send + Sync {
    async fn execute(&self, request: RegisterJournalEntryRequest) -> ApplicationResult<()>;
}
