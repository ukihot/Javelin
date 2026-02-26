// 再分類仕訳登録ユースケース - Input Port
// 目的: 測定額を変更せず表示区分のみ変更する再分類仕訳を登録する

use crate::{dtos::CreateReclassificationEntryRequest, error::ApplicationResult};

/// 再分類仕訳登録ユースケース
#[allow(async_fn_in_trait)]
pub trait CreateReclassificationEntryUseCase: Send + Sync {
    async fn execute(&self, request: CreateReclassificationEntryRequest) -> ApplicationResult<()>;
}
