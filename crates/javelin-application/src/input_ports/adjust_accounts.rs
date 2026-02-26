// 4.7 勘定補正処理（月次）
// 目的: 未整理残高や分類誤りを修正

use crate::{
    dtos::{AdjustAccountsRequest, AdjustAccountsResponse},
    error::ApplicationResult,
};

/// 勘定補正ユースケース
#[allow(async_fn_in_trait)]
pub trait AdjustAccountsUseCase: Send + Sync {
    async fn execute(
        &self,
        request: AdjustAccountsRequest,
    ) -> ApplicationResult<AdjustAccountsResponse>;
}
