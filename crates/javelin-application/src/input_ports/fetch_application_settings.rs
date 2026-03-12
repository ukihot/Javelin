// LoadApplicationSettings - アプリケーション設定取得
// 責務: アプリケーション設定の取得

use crate::{
    dtos::{request::FetchApplicationSettingsRequest, response::FetchApplicationSettingsResponse},
    error::ApplicationResult,
};

/// アプリケーション設定取得Input Port
#[allow(async_fn_in_trait)]
pub trait FetchApplicationSettingsInputPort: Send + Sync {
    /// アプリケーション設定を取得
    async fn execute(
        &self,
        request: FetchApplicationSettingsRequest,
    ) -> ApplicationResult<FetchApplicationSettingsResponse>;
}
