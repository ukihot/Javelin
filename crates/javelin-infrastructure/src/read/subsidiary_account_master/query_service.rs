// SubsidiaryAccountMasterQueryServiceImpl - 補助科目マスタQueryService実装

use std::sync::Arc;

use javelin_application::{
    error::{ApplicationError, ApplicationResult},
    query_service::SubsidiaryAccountMasterQueryService,
};
use javelin_domain::chart_of_accounts::{
    AccountCode, SubsidiaryAccountCode, SubsidiaryAccountMaster,
};

use super::projection::SubsidiaryAccountMasterProjection;
use crate::read::infrastructure::db::ProjectionDb;

/// 補助科目マスタQueryService実装
///
/// CQRS原則: ProjectionDBから補助科目マスタデータを取得
pub struct SubsidiaryAccountMasterQueryServiceImpl {
    projection: SubsidiaryAccountMasterProjection,
}

impl SubsidiaryAccountMasterQueryServiceImpl {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection: SubsidiaryAccountMasterProjection::new(projection_db) }
    }
}

impl SubsidiaryAccountMasterQueryService for SubsidiaryAccountMasterQueryServiceImpl {
    async fn get_all(&self) -> ApplicationResult<Vec<SubsidiaryAccountMaster>> {
        self.projection
            .get_all()
            .await
            .map_err(|e| ApplicationError::QueryExecutionFailed(e.to_string()))
    }

    async fn get_by_code(
        &self,
        code: &SubsidiaryAccountCode,
    ) -> ApplicationResult<Option<SubsidiaryAccountMaster>> {
        self.projection
            .get_by_code(code)
            .await
            .map_err(|e| ApplicationError::QueryExecutionFailed(e.to_string()))
    }

    async fn get_by_parent_account(
        &self,
        parent_account_code: &AccountCode,
    ) -> ApplicationResult<Vec<SubsidiaryAccountMaster>> {
        self.projection
            .get_by_parent_account(parent_account_code)
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
        let service = SubsidiaryAccountMasterQueryServiceImpl::new(projection_db);

        let result = service.get_all().await.unwrap();
        assert_eq!(result.len(), 0);
    }
}
