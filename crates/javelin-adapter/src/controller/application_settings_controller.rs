// ApplicationSettingsController - アプリケーション設定コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::LoadApplicationSettingsRequest, response::LoadApplicationSettingsResponse},
    input_ports::LoadApplicationSettingsInputPort,
    interactor::master_data::LoadApplicationSettingsInteractor,
    query_service::ApplicationSettingsMasterQueryService,
};

use crate::navigation::PresenterRegistry;

/// アプリケーション設定コントローラ
pub struct ApplicationSettingsController<Q: ApplicationSettingsMasterQueryService> {
    query_service: Arc<Q>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<Q: ApplicationSettingsMasterQueryService> ApplicationSettingsController<Q> {
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
        // PresenterRegistryからpage_id用のPresenterを取得
        if let Some(application_settings_presenter_arc) =
            self.presenter_registry.get_application_settings_presenter(page_id)
        {
            // ArcからPresenterをclone
            let application_settings_presenter = (*application_settings_presenter_arc).clone();

            // このページ専用のInteractorを動的に作成
            let interactor = LoadApplicationSettingsInteractor::new(
                Arc::clone(&self.query_service),
                application_settings_presenter,
            );

            // 実行
            interactor.execute(request).await.map_err(|e| e.to_string())
        } else {
            Err(format!("ApplicationSettingsPresenter not found for page_id: {}", page_id))
        }
    }
}
