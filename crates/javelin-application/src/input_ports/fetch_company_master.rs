// LoadCompanyMaster - 会社マスタ取得
// 責務: 会社マスタの取得

use crate::{
    dtos::{request::FetchCompanyMasterRequest, response::FetchCompanyMasterResponse},
    error::ApplicationResult,
};

/// 会社マスタ取得Input Port
#[allow(async_fn_in_trait)]
pub trait FetchCompanyMasterInputPort: Send + Sync {
    /// 会社マスタを取得
    async fn execute(
        &self,
        request: FetchCompanyMasterRequest,
    ) -> ApplicationResult<FetchCompanyMasterResponse>;
}
