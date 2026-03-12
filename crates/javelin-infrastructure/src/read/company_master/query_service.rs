// CompanyMasterQueryServiceImpl - 会社マスタQueryService実装

use std::sync::Arc;

use javelin_application::{
    dtos::{
        request::FetchCompanyMasterRequest,
        response::{CompanyMasterItem, FetchCompanyMasterResponse},
    },
    error::{ApplicationError, ApplicationResult},
    query_service::CompanyMasterQueryService,
};
use javelin_domain::company::{CompanyCode, CompanyMaster};

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

    async fn fetch_company_master(
        &self,
        request: FetchCompanyMasterRequest,
    ) -> ApplicationResult<FetchCompanyMasterResponse> {
        // すべての会社マスタを取得
        let mut companies = self.get_all().await?;

        // フィルタ条件があれば適用
        if let Some(filter) = request.filter {
            let filter_lower = filter.to_lowercase();
            companies.retain(|company| {
                company.code().value().to_lowercase().contains(&filter_lower)
                    || company.name().value().to_lowercase().contains(&filter_lower)
            });
        }

        // ドメインモデルをDTOに変換
        let items: Vec<CompanyMasterItem> = companies
            .into_iter()
            .map(|company| CompanyMasterItem {
                code: company.code().value().to_string(),
                name: company.name().value().to_string(),
                is_active: company.is_active(),
            })
            .collect();

        Ok(FetchCompanyMasterResponse { companies: items })
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
