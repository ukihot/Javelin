// AccountMasterController - 勘定科目マスタコントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{request::FetchAccountMasterRequest, response::FetchAccountMasterResponse},
    output_ports::AccountMasterOutputPort,
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
    /// CQRS原則: クエリはQueryServiceを直接使用（Interactorを経由しない）
    pub async fn handle_load_account_master(
        &self,
        page_id: uuid::Uuid,
        request: FetchAccountMasterRequest,
    ) -> Result<FetchAccountMasterResponse, String> {
        // PresenterRegistryから該当ページのPresenterを取得
        let presenter =
            self.presenter_registry.get_account_master_presenter(page_id).ok_or_else(|| {
                format!("Account master presenter not found for page_id: {}", page_id)
            })?;

        // QueryServiceから直接データを取得
        let response = self
            .query_service
            .fetch_account_master(request)
            .await
            .map_err(|e| e.to_string())?;

        // Presenterに結果を渡す
        presenter.present_account_master(&response).await;

        Ok(response)
    }
}
