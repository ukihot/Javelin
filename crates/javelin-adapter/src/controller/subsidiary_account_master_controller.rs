// SubsidiaryAccountMasterController - 補助科目マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{
        request::LoadSubsidiaryAccountMasterRequest, response::LoadSubsidiaryAccountMasterResponse,
    },
    input_ports::LoadSubsidiaryAccountMasterInputPort,
};

use crate::navigation::PresenterRegistry;

/// 補助科目マスタコントローラ
pub struct SubsidiaryAccountMasterController<U>
where
    U: LoadSubsidiaryAccountMasterInputPort,
{
    load_use_case: Arc<U>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<U> SubsidiaryAccountMasterController<U>
where
    U: LoadSubsidiaryAccountMasterInputPort,
{
    pub fn new(load_use_case: Arc<U>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { load_use_case, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// 補助科目マスタを取得
    pub async fn handle_load_subsidiary_account_master(
        &self,
        _page_id: uuid::Uuid,
        request: LoadSubsidiaryAccountMasterRequest,
    ) -> Result<LoadSubsidiaryAccountMasterResponse, String> {
        // UseCaseに委譲
        self.load_use_case.execute(request).await.map_err(|e| e.to_string())
    }
}
