// 洗替仕訳登録ユースケース - Input Port
// 目的: 既存評価額を一旦消去し再評価する洗替仕訳を登録する

use crate::{dtos::CreateReplacementEntryRequest, error::ApplicationResult};

/// 洗替仕訳登録ユースケース
#[allow(async_fn_in_trait)]
pub trait CreateReplacementEntryUseCase: Send + Sync {
    async fn execute(&self, request: CreateReplacementEntryRequest) -> ApplicationResult<()>;
}
