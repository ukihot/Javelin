// SubsidiaryAccountMasterController - 補助科目マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{
        request::LoadSubsidiaryAccountMasterRequest, response::LoadSubsidiaryAccountMasterResponse,
    },
    input_ports::LoadSubsidiaryAccountMasterInputPort,
    interactor::master_data::LoadSubsidiaryAccountMasterInteractor,
    query_service::SubsidiaryAccountMasterQueryService,
};

use crate::navigation::PresenterRegistry;

/// 補助科目マスタコントローラ
///
/// CQRS原則: 読み取りはQueryServiceを使用
pub struct SubsidiaryAccountMasterController<Q: SubsidiaryAccountMasterQueryService> {
    query_service: Arc<Q>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<Q: SubsidiaryAccountMasterQueryService> SubsidiaryAccountMasterController<Q> {
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
        // PresenterRegistryからpage_id用のPresenterを取得
        if let Some(subsidiary_account_master_presenter_arc) =
            self.presenter_registry.get_subsidiary_account_master_presenter(page_id)
        {
            // ArcからPresenterをclone
            let subsidiary_account_master_presenter =
                (*subsidiary_account_master_presenter_arc).clone();

            // このページ専用のInteractorを動的に作成
            let interactor = LoadSubsidiaryAccountMasterInteractor::new(
                Arc::clone(&self.query_service),
                subsidiary_account_master_presenter,
            );

            // 実行
            interactor.execute(request).await.map_err(|e| e.to_string())
        } else {
            Err(format!("SubsidiaryAccountMasterPresenter not found for page_id: {}", page_id))
        }
    }
}
