// 4.6 注記草案生成処理（月次）
// 目的: 財務諸表に付随する開示情報を整理

use crate::{
    dtos::{GenerateNoteDraftRequest, GenerateNoteDraftResponse},
    error::ApplicationResult,
};

/// 注記草案生成ユースケース
#[allow(async_fn_in_trait)]
pub trait GenerateNoteDraftUseCase: Send + Sync {
    async fn execute(
        &self,
        request: GenerateNoteDraftRequest,
    ) -> ApplicationResult<GenerateNoteDraftResponse>;
}
