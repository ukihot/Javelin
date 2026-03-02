// 元帳整合性検証ユースケース

use crate::{
    dtos::{VerifyLedgerConsistencyRequest, VerifyLedgerConsistencyResponse},
    error::ApplicationResult,
};

pub trait VerifyLedgerConsistencyUseCase: Send + Sync {
    fn execute(
        &self,
        request: VerifyLedgerConsistencyRequest,
    ) -> impl std::future::Future<Output = ApplicationResult<VerifyLedgerConsistencyResponse>> + Send;
}
