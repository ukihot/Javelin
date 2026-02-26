// 4.3 締準備処理（月次）
// 目的: 当月の取引が適切に記録され、暫定財務数値を確立

use crate::{
    dtos::{PrepareClosingRequest, PrepareClosingResponse},
    error::ApplicationResult,
};

/// 締準備ユースケース
#[allow(async_fn_in_trait)]
pub trait PrepareClosingUseCase: Send + Sync {
    async fn execute(
        &self,
        request: PrepareClosingRequest,
    ) -> ApplicationResult<PrepareClosingResponse>;
}
