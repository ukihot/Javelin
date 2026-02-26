// CompanyMasterInteractor - 会社マスタ操作のユースケース

use std::sync::Arc;

use javelin_domain::{
    masters::{CompanyCode, CompanyMaster, CompanyName},
    repositories::CompanyMasterRepository,
};

use crate::error::ApplicationResult;

/// 会社マスタ取得クエリ
#[derive(Debug, Clone)]
pub struct GetCompanyMastersQuery;

/// 会社マスタ登録リクエスト
#[derive(Debug, Clone)]
pub struct RegisterCompanyMasterRequest {
    pub code: String,
    pub name: String,
}

/// 会社マスタ更新リクエスト
#[derive(Debug, Clone)]
pub struct UpdateCompanyMasterRequest {
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

/// 会社マスタInteractor
pub struct CompanyMasterInteractor<R>
where
    R: CompanyMasterRepository,
{
    repository: Arc<R>,
}

impl<R> CompanyMasterInteractor<R>
where
    R: CompanyMasterRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// 全会社マスタを取得
    pub async fn get_all(
        &self,
        _query: GetCompanyMastersQuery,
    ) -> ApplicationResult<Vec<CompanyMaster>> {
        self.repository
            .find_all()
            .await
            .map_err(|e| crate::error::ApplicationError::QueryExecutionFailed(e.to_string()))
    }

    /// 会社マスタを登録
    pub async fn register(&self, request: RegisterCompanyMasterRequest) -> ApplicationResult<()> {
        let code = CompanyCode::new(request.code)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
        let name = CompanyName::new(request.name)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        // 重複チェック
        if self.repository.find_by_code(&code).await?.is_some() {
            return Err(crate::error::ApplicationError::ValidationError(format!(
                "会社コード {} は既に存在します",
                code.value()
            )));
        }

        let company_master = CompanyMaster::new(code, name, true);

        self.repository
            .save(&company_master)
            .await
            .map_err(|e| crate::error::ApplicationError::UseCaseExecutionFailed(e.to_string()))
    }

    /// 会社マスタを更新
    pub async fn update(&self, request: UpdateCompanyMasterRequest) -> ApplicationResult<()> {
        let code = CompanyCode::new(request.code)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        let _company_master = self.repository.find_by_code(&code).await?.ok_or_else(|| {
            crate::error::ApplicationError::ValidationError(format!(
                "会社コード {} が見つかりません",
                code.value()
            ))
        })?;

        let name = CompanyName::new(request.name)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        let updated = CompanyMaster::new(code, name, request.is_active);

        self.repository
            .save(&updated)
            .await
            .map_err(|e| crate::error::ApplicationError::UseCaseExecutionFailed(e.to_string()))
    }

    /// 会社マスタを削除
    pub async fn delete(&self, code: String) -> ApplicationResult<()> {
        let code = CompanyCode::new(code)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        self.repository
            .delete(&code)
            .await
            .map_err(|e| crate::error::ApplicationError::UseCaseExecutionFailed(e.to_string()))
    }
}
