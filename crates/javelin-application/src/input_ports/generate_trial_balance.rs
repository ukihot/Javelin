// 4.5 試算表生成処理（月次）
// 目的: 勘定残高の整合性を体系的に検証

use crate::{
    dtos::{GenerateTrialBalanceRequest, GenerateTrialBalanceResponse},
    error::ApplicationResult,
};

/// 試算表生成ユースケース
#[allow(async_fn_in_trait)]
pub trait GenerateTrialBalanceUseCase: Send + Sync {
    async fn execute(
        &self,
        request: GenerateTrialBalanceRequest,
    ) -> ApplicationResult<GenerateTrialBalanceResponse>;
}
