// CompanyMasterController - 会社マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::FetchCompanyMasterRequest, response::FetchCompanyMasterResponse},
    output_ports::CompanyMasterOutputPort,
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
    /// CQRS原則: クエリはQueryServiceを直接使用（Interactorを経由しない）
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

        // QueryServiceから直接データを取得
        let response = self
            .query_service
            .fetch_company_master(request)
            .await
            .map_err(|e| e.to_string())?;

        // Presenterに結果を渡す
        presenter.present_company_master(&response).await;

        Ok(response)
    }
}
