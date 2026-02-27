// CompanyMasterRepository - 会社マスタリポジトリトレイト

use crate::{
    error::DomainResult,
    masters::{CompanyCode, CompanyMaster},
};

/// 会社マスタリポジトリトレイト
///
/// CQRS原則: Repositoryはイベント永続化のみを担当
/// 読み取りはQueryServiceを使用すること
#[allow(async_fn_in_trait)]
pub trait CompanyMasterRepository: Send + Sync {
    /// 会社マスタを保存
    async fn save(&self, company_master: &CompanyMaster) -> DomainResult<()>;

    /// 会社マスタを削除
    async fn delete(&self, code: &CompanyCode) -> DomainResult<()>;
}
