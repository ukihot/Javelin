// 承認申請ユースケース - Input Port
// 目的: 下書き仕訳を承認申請する

use crate::{dtos::SubmitForApprovalRequest, error::ApplicationResult};

/// 承認申請ユースケース
#[allow(async_fn_in_trait)]
pub trait SubmitForApprovalUseCase: Send + Sync {
    async fn execute(&self, request: SubmitForApprovalRequest) -> ApplicationResult<()>;
}
