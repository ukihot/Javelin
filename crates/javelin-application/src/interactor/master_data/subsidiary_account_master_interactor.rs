// SubsidiaryAccountMasterInteractor - 補助科目マスタ操作のユースケース

use std::sync::Arc;

use javelin_domain::{
    error::DomainResult,
    masters::{SubsidiaryAccountCode, SubsidiaryAccountMaster},
    repositories::SubsidiaryAccountMasterRepository,
};

/// 補助科目マスタ操作のInteractor
pub struct SubsidiaryAccountMasterInteractor<R: SubsidiaryAccountMasterRepository> {
    repository: Arc<R>,
}

impl<R: SubsidiaryAccountMasterRepository> SubsidiaryAccountMasterInteractor<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// 全補助科目マスタを取得
    pub async fn get_all(&self) -> DomainResult<Vec<SubsidiaryAccountMaster>> {
        self.repository.find_all().await
    }

    /// 補助科目マスタを取得
    pub async fn get(&self, code: String) -> DomainResult<SubsidiaryAccountMaster> {
        let code = SubsidiaryAccountCode::new(&code)?;
        self.repository.find_by_code(&code).await?.ok_or_else(|| {
            javelin_domain::error::DomainError::NotFound(format!(
                "補助科目マスタが見つかりません: {}",
                code.value()
            ))
        })
    }

    /// 補助科目マスタを作成
    pub async fn create(&self, account: SubsidiaryAccountMaster) -> DomainResult<()> {
        // 既存チェック
        if self.repository.find_by_code(account.code()).await?.is_some() {
            return Err(javelin_domain::error::DomainError::ValidationError(format!(
                "補助科目マスタが既に存在します: {}",
                account.code().value()
            )));
        }

        self.repository.save(&account).await
    }

    /// 補助科目マスタを更新
    pub async fn update(&self, account: SubsidiaryAccountMaster) -> DomainResult<()> {
        // 存在チェック
        if self.repository.find_by_code(account.code()).await?.is_none() {
            return Err(javelin_domain::error::DomainError::NotFound(format!(
                "補助科目マスタが見つかりません: {}",
                account.code().value()
            )));
        }

        self.repository.save(&account).await
    }

    /// 補助科目マスタを削除
    pub async fn delete(&self, code: String) -> DomainResult<()> {
        let code = SubsidiaryAccountCode::new(&code)?;

        // 存在チェック
        if self.repository.find_by_code(&code).await?.is_none() {
            return Err(javelin_domain::error::DomainError::NotFound(format!(
                "補助科目マスタが見つかりません: {}",
                code.value()
            )));
        }

        self.repository.delete(&code).await
    }
}
