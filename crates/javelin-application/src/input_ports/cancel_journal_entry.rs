// 取消仕訳登録ユースケース - Input Port
// 目的: 既存仕訳の効力を無効化する取消仕訳を登録する

use crate::{dtos::CancelJournalEntryRequest, error::ApplicationResult};

/// 取消仕訳登録ユースケース
#[allow(async_fn_in_trait)]
pub trait CancelJournalEntryUseCase: Send + Sync {
    async fn execute(&self, request: CancelJournalEntryRequest) -> ApplicationResult<()>;
}
