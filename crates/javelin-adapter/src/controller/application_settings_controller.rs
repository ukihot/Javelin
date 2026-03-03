// ApplicationSettingsController - アプリケーション設定コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::LoadApplicationSettingsRequest, response::LoadApplicationSettingsResponse},
    query_service::ApplicationSettingsMasterQueryService,
};

use crate::navigation::PresenterRegistry;

/// アプリケーション設定コントローラ
pub struct ApplicationSettingsController<Q>
where
    Q: ApplicationSettingsMasterQueryService,
{
    query_service: Arc<Q>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<Q> ApplicationSettingsController<Q>
where
    Q: ApplicationSettingsMasterQueryService,
{
    pub fn new(query_service: Arc<Q>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { query_service, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// アプリケーション設定を取得
    pub async fn handle_load_application_settings(
        &self,
        page_id: uuid::Uuid,
        request: LoadApplicationSettingsRequest,
    ) -> Result<LoadApplicationSettingsResponse, String> {
        // PresenterRegistryから該当ページのPresenterを取得
        let presenter = self
            .presenter_registry
            .get_application_settings_presenter(page_id)
            .ok_or_else(|| {
                format!("Application settings presenter not found for page_id: {}", page_id)
            })?;

        // 取得したPresenterを使って新しいInteractorを作成
        let interactor =
            javelin_application::interactor::LoadApplicationSettingsInteractor::new(
                Arc::clone(&self.query_service),
                (*presenter).clone(),
            );

        // UseCaseに委譲
        use javelin_application::input_ports::LoadApplicationSettingsInputPort;
        interactor.execute(request).await.map_err(|e| e.to_string())
    }
}
