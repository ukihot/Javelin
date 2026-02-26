// 4.4 締日固定処理（月次）
// 目的: 当月の会計データを確定し、改竄防止

use crate::{
    dtos::{LockClosingPeriodRequest, LockClosingPeriodResponse},
    error::ApplicationResult,
};

/// 締日固定ユースケース
#[allow(async_fn_in_trait)]
pub trait LockClosingPeriodUseCase: Send + Sync {
    async fn execute(
        &self,
        request: LockClosingPeriodRequest,
    ) -> ApplicationResult<LockClosingPeriodResponse>;
}
