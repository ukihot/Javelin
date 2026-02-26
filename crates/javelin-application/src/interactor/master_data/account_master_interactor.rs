// AccountMasterInteractor - 勘定科目マスタ操作のユースケース

use std::sync::Arc;

use javelin_domain::{
    masters::{AccountCode, AccountMaster, AccountName, AccountType},
    repositories::AccountMasterRepository,
};

use crate::error::ApplicationResult;

/// 勘定科目マスタ取得クエリ
#[derive(Debug, Clone)]
pub struct GetAccountMastersQuery;

/// 勘定科目マスタ登録リクエスト
#[derive(Debug, Clone)]
pub struct RegisterAccountMasterRequest {
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
}

/// 勘定科目マスタ更新リクエスト
#[derive(Debug, Clone)]
pub struct UpdateAccountMasterRequest {
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

/// 勘定科目マスタInteractor
pub struct AccountMasterInteractor<R>
where
    R: AccountMasterRepository,
{
    repository: Arc<R>,
}

impl<R> AccountMasterInteractor<R>
where
    R: AccountMasterRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// 全勘定科目マスタを取得
    pub async fn get_all(
        &self,
        _query: GetAccountMastersQuery,
    ) -> ApplicationResult<Vec<AccountMaster>> {
        self.repository
            .find_all()
            .await
            .map_err(|e| crate::error::ApplicationError::QueryExecutionFailed(e.to_string()))
    }

    /// 勘定科目マスタを登録
    pub async fn register(&self, request: RegisterAccountMasterRequest) -> ApplicationResult<()> {
        let code = AccountCode::new(request.code)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
        let name = AccountName::new(request.name)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        // 重複チェック
        if self.repository.find_by_code(&code).await?.is_some() {
            return Err(crate::error::ApplicationError::ValidationError(format!(
                "勘定科目コード {} は既に存在します",
                code.value()
            )));
        }

        let account_master = AccountMaster::new(code, name, request.account_type, true);

        self.repository
            .save(&account_master)
            .await
            .map_err(|e| crate::error::ApplicationError::UseCaseExecutionFailed(e.to_string()))
    }

    /// 勘定科目マスタを更新
    pub async fn update(&self, request: UpdateAccountMasterRequest) -> ApplicationResult<()> {
        let code = AccountCode::new(request.code)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        let account_master = self.repository.find_by_code(&code).await?.ok_or_else(|| {
            crate::error::ApplicationError::ValidationError(format!(
                "勘定科目コード {} が見つかりません",
                code.value()
            ))
        })?;

        let name = AccountName::new(request.name)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        let updated =
            AccountMaster::new(code, name, account_master.account_type(), request.is_active);

        self.repository
            .save(&updated)
            .await
            .map_err(|e| crate::error::ApplicationError::UseCaseExecutionFailed(e.to_string()))
    }

    /// 勘定科目マスタを削除
    pub async fn delete(&self, code: String) -> ApplicationResult<()> {
        let code = AccountCode::new(code)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        self.repository
            .delete(&code)
            .await
            .map_err(|e| crate::error::ApplicationError::UseCaseExecutionFailed(e.to_string()))
    }
}
