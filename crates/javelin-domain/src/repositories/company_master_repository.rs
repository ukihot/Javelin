// CompanyMasterRepository - 会社マスタリポジトリトレイト

use crate::{
    error::DomainResult,
    masters::{CompanyCode, CompanyMaster},
};

/// 会社マスタリポジトリトレイト
#[allow(async_fn_in_trait)]
pub trait CompanyMasterRepository: Send + Sync {
    /// 会社マスタを取得
    async fn find_by_code(&self, code: &CompanyCode) -> DomainResult<Option<CompanyMaster>>;

    /// すべての会社マスタを取得
    async fn find_all(&self) -> DomainResult<Vec<CompanyMaster>>;

    /// 会社マスタを保存
    async fn save(&self, company_master: &CompanyMaster) -> DomainResult<()>;

    /// 会社マスタを削除
    async fn delete(&self, code: &CompanyCode) -> DomainResult<()>;
}
