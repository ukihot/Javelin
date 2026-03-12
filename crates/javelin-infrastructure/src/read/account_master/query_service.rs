// AccountMasterQueryServiceImpl - 勘定科目マスタQueryService実装

use std::sync::Arc;

use javelin_application::{
    dtos::{
        request::FetchAccountMasterRequest,
        response::{AccountMasterItem, FetchAccountMasterResponse},
    },
    error::{ApplicationError, ApplicationResult},
    query_service::AccountMasterQueryService,
};
use javelin_domain::chart_of_accounts::{AccountCode, AccountMaster};

use super::projection::AccountMasterProjection;
use crate::read::infrastructure::db::ProjectionDb;

/// 勘定科目マスタQueryService実装
///
/// CQRS原則: ProjectionDBから勘定科目マスタデータを取得
pub struct AccountMasterQueryServiceImpl {
    projection: AccountMasterProjection,
}

impl AccountMasterQueryServiceImpl {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection: AccountMasterProjection::new(projection_db) }
    }
}

impl AccountMasterQueryService for AccountMasterQueryServiceImpl {
    async fn get_all(&self) -> ApplicationResult<Vec<AccountMaster>> {
        self.projection
            .get_all()
            .await
            .map_err(|e| ApplicationError::QueryExecutionFailed(e.to_string()))
    }

    async fn get_by_code(&self, code: &AccountCode) -> ApplicationResult<Option<AccountMaster>> {
        self.projection
            .get_by_code(code)
            .await
            .map_err(|e| ApplicationError::QueryExecutionFailed(e.to_string()))
    }

    async fn fetch_account_master(
        &self,
        request: FetchAccountMasterRequest,
    ) -> ApplicationResult<FetchAccountMasterResponse> {
        // フィルタ条件に基づいて勘定科目を取得
        let accounts = self.get_filtered(true, request.filter).await?;

        // ドメインモデルをDTOに変換
        let items: Vec<AccountMasterItem> = accounts
            .into_iter()
            .map(|account| AccountMasterItem {
                code: account.code().value().to_string(),
                name: account.name().value().to_string(),
                account_type: format!("{:?}", account.account_type()),
            })
            .collect();

        Ok(FetchAccountMasterResponse { accounts: items })
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_get_all_empty() {
        let temp_dir = TempDir::new().unwrap();
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
        let service = AccountMasterQueryServiceImpl::new(projection_db);

        let result = service.get_all().await.unwrap();
        assert_eq!(result.len(), 0);
    }
}
