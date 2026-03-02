// ApplicationSettingsController - アプリケーション設定コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::LoadApplicationSettingsRequest, response::LoadApplicationSettingsResponse},
    input_ports::LoadApplicationSettingsInputPort,
};

use crate::navigation::PresenterRegistry;

/// アプリケーション設定コントローラ
pub struct ApplicationSettingsController<U>
where
    U: LoadApplicationSettingsInputPort,
{
    load_use_case: Arc<U>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<U> ApplicationSettingsController<U>
where
    U: LoadApplicationSettingsInputPort,
{
    pub fn new(load_use_case: Arc<U>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { load_use_case, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// アプリケーション設定を取得
    pub async fn handle_load_application_settings(
        &self,
        _page_id: uuid::Uuid,
        request: LoadApplicationSettingsRequest,
    ) -> Result<LoadApplicationSettingsResponse, String> {
        // UseCaseに委譲
        self.load_use_case.execute(request).await.map_err(|e| e.to_string())
    }
}
