// 4.8 IFRS評価処理（月次）
// 目的: 見積会計処理を実施し、将来キャッシュフローを反映

use crate::{
    dtos::{ApplyIfrsValuationRequest, ApplyIfrsValuationResponse},
    error::ApplicationResult,
};

/// IFRS評価ユースケース
#[allow(async_fn_in_trait)]
pub trait ApplyIfrsValuationUseCase: Send + Sync {
    async fn execute(
        &self,
        request: ApplyIfrsValuationRequest,
    ) -> ApplicationResult<ApplyIfrsValuationResponse>;
}
