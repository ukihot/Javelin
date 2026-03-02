// AccountMasterController - 勘定科目マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::LoadAccountMasterRequest, response::LoadAccountMasterResponse},
    input_ports::LoadAccountMasterInputPort,
};

use crate::navigation::PresenterRegistry;

/// 勘定科目マスタコントローラ
pub struct AccountMasterController<U>
where
    U: LoadAccountMasterInputPort,
{
    load_use_case: Arc<U>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<U> AccountMasterController<U>
where
    U: LoadAccountMasterInputPort,
{
    pub fn new(load_use_case: Arc<U>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { load_use_case, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// 勘定科目マスタを取得
    pub async fn handle_load_account_master(
        &self,
        _page_id: uuid::Uuid,
        request: LoadAccountMasterRequest,
    ) -> Result<LoadAccountMasterResponse, String> {
        // UseCaseに委譲
        self.load_use_case.execute(request).await.map_err(|e| e.to_string())
    }
}
