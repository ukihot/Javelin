// 反対仕訳登録ユースケース - Input Port
// 目的: 既存残高または期間帰属を反転させる反対仕訳を登録する

use crate::{dtos::CreateReversalEntryRequest, error::ApplicationResult};

/// 反対仕訳登録ユースケース
#[allow(async_fn_in_trait)]
pub trait CreateReversalEntryUseCase: Send + Sync {
    async fn execute(&self, request: CreateReversalEntryRequest) -> ApplicationResult<()>;
}
