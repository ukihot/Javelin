// LoadSubsidiaryAccountMaster - 補助科目マスタ取得
// 責務: 補助科目マスタの取得

use crate::{
    dtos::{
        request::FetchSubsidiaryAccountMasterRequest,
        response::FetchSubsidiaryAccountMasterResponse,
    },
    error::ApplicationResult,
};

/// 補助科目マスタ取得Input Port
#[allow(async_fn_in_trait)]
pub trait FetchSubsidiaryAccountMasterInputPort: Send + Sync {
    /// 補助科目マスタを取得
    async fn execute(
        &self,
        request: FetchSubsidiaryAccountMasterRequest,
    ) -> ApplicationResult<FetchSubsidiaryAccountMasterResponse>;
}
