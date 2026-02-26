// 承認ユースケース - Input Port
// 目的: 承認待ち仕訳を承認・記帳する

use crate::{dtos::ApproveJournalEntryRequest, error::ApplicationResult};

/// 承認ユースケース
#[allow(async_fn_in_trait)]
pub trait ApproveJournalEntryUseCase: Send + Sync {
    async fn execute(&self, request: ApproveJournalEntryRequest) -> ApplicationResult<()>;
}
