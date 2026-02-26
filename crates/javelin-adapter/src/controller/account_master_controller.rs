// AccountMasterController - 勘定科目マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::LoadAccountMasterRequest, response::LoadAccountMasterResponse},
    input_ports::LoadAccountMasterInputPort,
    interactor::master_data::LoadAccountMasterInteractor,
};
use javelin_infrastructure::read::queries::MasterDataLoaderImpl;

use crate::navigation::PresenterRegistry;

/// 勘定科目マスタコントローラ（具体型版）
///
/// 型パラメータを削除し、具体的な型を直接保持することで
/// 他のControllerと統一したパターンに変更
pub struct AccountMasterController {
    query_service: Arc<MasterDataLoaderImpl>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl AccountMasterController {
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

    /// 勘定科目マスタを取得
    pub async fn handle_load_account_master(
        &self,
        page_id: uuid::Uuid,
        request: LoadAccountMasterRequest,
    ) -> Result<LoadAccountMasterResponse, String> {
        // PresenterRegistryからpage_id用のPresenterを取得
        if let Some(account_master_presenter_arc) =
            self.presenter_registry.get_account_master_presenter(page_id)
        {
            // ArcからPresenterをclone
            let account_master_presenter = (*account_master_presenter_arc).clone();

            // このページ専用のInteractorを動的に作成
            let interactor = LoadAccountMasterInteractor::new(
                Arc::clone(&self.query_service),
                account_master_presenter,
            );

            // 実行
            interactor.execute(request).await.map_err(|e| e.to_string())
        } else {
            Err(format!("AccountMasterPresenter not found for page_id: {}", page_id))
        }
    }
}
