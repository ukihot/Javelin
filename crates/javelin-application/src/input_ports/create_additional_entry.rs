// 追加仕訳登録ユースケース - Input Port
// 目的: 計上不足または後日判明事項を補正する追加仕訳を登録する

use crate::{dtos::CreateAdditionalEntryRequest, error::ApplicationResult};

/// 追加仕訳登録ユースケース
#[allow(async_fn_in_trait)]
pub trait CreateAdditionalEntryUseCase: Send + Sync {
    async fn execute(&self, request: CreateAdditionalEntryRequest) -> ApplicationResult<()>;
}
