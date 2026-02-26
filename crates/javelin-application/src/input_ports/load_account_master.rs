// LoadAccountMaster - 勘定科目マスタ取得
// 責務: 勘定科目マスタの取得

use crate::{
    dtos::{LoadAccountMasterRequest, LoadAccountMasterResponse},
    error::ApplicationResult,
};

/// 勘定科目マスタ取得Input Port
#[allow(async_fn_in_trait)]
pub trait LoadAccountMasterInputPort: Send + Sync {
    /// 勘定科目マスタを取得
    async fn execute(
        &self,
        request: LoadAccountMasterRequest,
    ) -> ApplicationResult<LoadAccountMasterResponse>;
}
