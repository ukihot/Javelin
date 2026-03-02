// CompanyMasterController - 会社マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::LoadCompanyMasterRequest, response::LoadCompanyMasterResponse},
    input_ports::LoadCompanyMasterInputPort,
};

use crate::navigation::PresenterRegistry;

/// 会社マスタコントローラ
pub struct CompanyMasterController<U>
where
    U: LoadCompanyMasterInputPort,
{
    load_use_case: Arc<U>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<U> CompanyMasterController<U>
where
    U: LoadCompanyMasterInputPort,
{
    pub fn new(load_use_case: Arc<U>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { load_use_case, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// 会社マスタを取得
    pub async fn handle_load_company_master(
        &self,
        _page_id: uuid::Uuid,
        request: LoadCompanyMasterRequest,
    ) -> Result<LoadCompanyMasterResponse, String> {
        // UseCaseに委譲
        self.load_use_case.execute(request).await.map_err(|e| e.to_string())
    }
}
