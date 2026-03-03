// AccountMasterController - 勘定科目マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::LoadAccountMasterRequest, response::LoadAccountMasterResponse},
    query_service::AccountMasterQueryService,
};

use crate::navigation::PresenterRegistry;

/// 勘定科目マスタコントローラ
pub struct AccountMasterController<Q>
where
    Q: AccountMasterQueryService,
{
    query_service: Arc<Q>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<Q> AccountMasterController<Q>
where
    Q: AccountMasterQueryService,
{
    pub fn new(query_service: Arc<Q>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { query_service, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// 勘定科目マスタを取得
    pub async fn handle_load_account_master(
        &self,
        page_id: uuid::Uuid,
        request: LoadAccountMasterRequest,
    ) -> Result<LoadAccountMasterResponse, String> {
        // PresenterRegistryから該当ページのPresenterを取得
        let presenter = self
            .presenter_registry
            .get_account_master_presenter(page_id)
            .ok_or_else(|| {
                format!("Account master presenter not found for page_id: {}", page_id)
            })?;

        // 取得したPresenterを使って新しいInteractorを作成
        let interactor = javelin_application::interactor::LoadAccountMasterInteractor::new(
            Arc::clone(&self.query_service),
            (*presenter).clone(),
        );

        // UseCaseに委譲
        use javelin_application::input_ports::LoadAccountMasterInputPort;
        interactor.execute(request).await.map_err(|e| e.to_string())
    }
}
