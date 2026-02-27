// CompanyMasterQueryServiceImpl - 会社マスタQueryService実装

use std::sync::Arc;

use javelin_application::{
    error::{ApplicationError, ApplicationResult},
    query_service::CompanyMasterQueryService,
};
use javelin_domain::masters::{CompanyCode, CompanyMaster};

use super::projection::CompanyMasterProjection;
use crate::read::infrastructure::db::ProjectionDb;

/// 会社マスタQueryService実装
///
/// CQRS原則: ProjectionDBから会社マスタデータを取得
pub struct CompanyMasterQueryServiceImpl {
    projection: CompanyMasterProjection,
}

impl CompanyMasterQueryServiceImpl {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection: CompanyMasterProjection::new(projection_db) }
    }
}

impl CompanyMasterQueryService for CompanyMasterQueryServiceImpl {
    async fn get_all(&self) -> ApplicationResult<Vec<CompanyMaster>> {
        self.projection
            .get_all()
            .await
            .map_err(|e| ApplicationError::QueryExecutionFailed(e.to_string()))
    }

    async fn get_by_code(&self, code: &CompanyCode) -> ApplicationResult<Option<CompanyMaster>> {
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
        let service = CompanyMasterQueryServiceImpl::new(projection_db);

        let result = service.get_all().await.unwrap();
        assert_eq!(result.len(), 0);
    }
}
