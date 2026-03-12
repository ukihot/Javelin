// CompanyMasterController - 会社マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::FetchCompanyMasterRequest, response::FetchCompanyMasterResponse},
    query_service::CompanyMasterQueryService,
};

use crate::navigation::PresenterRegistry;

/// 会社マスタコントローラ
pub struct CompanyMasterController<Q>
where
    Q: CompanyMasterQueryService,
{
    query_service: Arc<Q>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<Q> CompanyMasterController<Q>
where
    Q: CompanyMasterQueryService,
{
    pub fn new(query_service: Arc<Q>, presenter_registry: Arc<PresenterRegistry>) -> Self {
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
        request: FetchCompanyMasterRequest,
    ) -> Result<FetchCompanyMasterResponse, String> {
        // PresenterRegistryから該当ページのPresenterを取得
        let presenter =
            self.presenter_registry.get_company_master_presenter(page_id).ok_or_else(|| {
                format!("Company master presenter not found for page_id: {}", page_id)
            })?;

        // 取得したPresenterを使って新しいInteractorを作成
        let interactor = javelin_application::interactor::FetchCompanyMasterInteractor::new(
            Arc::clone(&self.query_service),
            (*presenter).clone(),
        );

        // UseCaseに委譲
        use javelin_application::input_ports::FetchCompanyMasterInputPort;
        interactor.execute(request).await.map_err(|e| e.to_string())
    }
}
