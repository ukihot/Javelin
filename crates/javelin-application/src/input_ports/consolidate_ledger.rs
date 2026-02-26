// 4.2 元帳集約処理（週次）
// 目的: 日次登録された仕訳情報を勘定科目体系へ体系的に集約

use crate::{
    dtos::{ConsolidateLedgerRequest, ConsolidateLedgerResponse},
    error::ApplicationResult,
};

/// 元帳集約ユースケース
#[allow(async_fn_in_trait)]
pub trait ConsolidateLedgerUseCase: Send + Sync {
    async fn execute(
        &self,
        request: ConsolidateLedgerRequest,
    ) -> ApplicationResult<ConsolidateLedgerResponse>;
}
