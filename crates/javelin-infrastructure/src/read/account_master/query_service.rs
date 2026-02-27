// AccountMasterQueryServiceImpl - 勘定科目マスタQueryService実装

use std::sync::Arc;

use javelin_application::{
    error::{ApplicationError, ApplicationResult},
    query_service::AccountMasterQueryService,
};
use javelin_domain::masters::{AccountCode, AccountMaster};

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
