// LoadApplicationSettings - アプリケーション設定取得
// 責務: アプリケーション設定の取得

use crate::{
    dtos::{request::LoadApplicationSettingsRequest, response::LoadApplicationSettingsResponse},
    error::ApplicationResult,
};

/// アプリケーション設定取得Input Port
#[allow(async_fn_in_trait)]
pub trait LoadApplicationSettingsInputPort: Send + Sync {
    /// アプリケーション設定を取得
    async fn execute(
        &self,
        request: LoadApplicationSettingsRequest,
    ) -> ApplicationResult<LoadApplicationSettingsResponse>;
}
