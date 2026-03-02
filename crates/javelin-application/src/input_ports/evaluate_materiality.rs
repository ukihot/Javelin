// 重要性判定ユースケース

use crate::{
    dtos::{EvaluateMaterialityRequest, EvaluateMaterialityResponse},
    error::ApplicationResult,
};

pub trait EvaluateMaterialityUseCase: Send + Sync {
    fn execute(
        &self,
        request: EvaluateMaterialityRequest,
    ) -> impl std::future::Future<Output = ApplicationResult<EvaluateMaterialityResponse>> + Send;
}
