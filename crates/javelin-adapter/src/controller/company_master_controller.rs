// CompanyMasterController - 会社マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::LoadCompanyMasterRequest, response::LoadCompanyMasterResponse},
    input_ports::LoadCompanyMasterInputPort,
    interactor::master_data::LoadCompanyMasterInteractor,
};
use javelin_infrastructure::read::query_services::MasterDataLoaderImpl;

use crate::navigation::PresenterRegistry;

/// 会社マスタコントローラ
pub struct CompanyMasterController {
    query_service: Arc<MasterDataLoaderImpl>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl CompanyMasterController {
    pub fn new(
        query_service: Arc<MasterDataLoaderImpl>,
        presenter_registry: Arc<PresenterRegistry>,
    ) -> Self {
        Self { query_service, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// 会社マスタを取得
    pub async fn handle_load_company_master(
        &self,
        page_id: uuid::Uuid,
        request: LoadCompanyMasterRequest,
    ) -> Result<LoadCompanyMasterResponse, String> {
        // PresenterRegistryからpage_id用のPresenterを取得
        if let Some(company_master_presenter_arc) =
            self.presenter_registry.get_company_master_presenter(page_id)
        {
            // ArcからPresenterをclone
            let company_master_presenter = (*company_master_presenter_arc).clone();

            // このページ専用のInteractorを動的に作成
            let interactor = LoadCompanyMasterInteractor::new(
                Arc::clone(&self.query_service),
                company_master_presenter,
            );

            // 実行
            interactor.execute(request).await.map_err(|e| e.to_string())
        } else {
            Err(format!("CompanyMasterPresenter not found for page_id: {}", page_id))
        }
    }
}
