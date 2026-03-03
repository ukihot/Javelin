// SubsidiaryAccountMasterController - 補助科目マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{
        request::LoadSubsidiaryAccountMasterRequest, response::LoadSubsidiaryAccountMasterResponse,
    },
    query_service::SubsidiaryAccountMasterQueryService,
};

use crate::navigation::PresenterRegistry;

/// 補助科目マスタコントローラ
pub struct SubsidiaryAccountMasterController<Q>
where
    Q: SubsidiaryAccountMasterQueryService,
{
    query_service: Arc<Q>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<Q> SubsidiaryAccountMasterController<Q>
where
    Q: SubsidiaryAccountMasterQueryService,
{
    pub fn new(query_service: Arc<Q>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { query_service, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// 補助科目マスタを取得
    pub async fn handle_load_subsidiary_account_master(
        &self,
        page_id: uuid::Uuid,
        request: LoadSubsidiaryAccountMasterRequest,
    ) -> Result<LoadSubsidiaryAccountMasterResponse, String> {
        // PresenterRegistryから該当ページのPresenterを取得
        let presenter = self
            .presenter_registry
            .get_subsidiary_account_master_presenter(page_id)
            .ok_or_else(|| {
                format!("Subsidiary account master presenter not found for page_id: {}", page_id)
            })?;

        // 取得したPresenterを使って新しいInteractorを作成
        let interactor =
            javelin_application::interactor::LoadSubsidiaryAccountMasterInteractor::new(
                Arc::clone(&self.query_service),
                (*presenter).clone(),
            );

        // UseCaseに委譲
        use javelin_application::input_ports::LoadSubsidiaryAccountMasterInputPort;
        interactor.execute(request).await.map_err(|e| e.to_string())
    }
}
