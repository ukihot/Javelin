// SubsidiaryAccountMasterRepository - 補助科目マスタリポジトリトレイト

use crate::{
    error::DomainResult,
    masters::{SubsidiaryAccountCode, SubsidiaryAccountMaster},
};

/// 補助科目マスタリポジトリトレイト
///
/// CQRS原則: Repositoryはイベント永続化のみを担当
/// 読み取りはQueryServiceを使用すること
#[allow(async_fn_in_trait)]
pub trait SubsidiaryAccountMasterRepository: Send + Sync {
    /// 補助科目マスタを保存
    async fn save(&self, subsidiary_account_master: &SubsidiaryAccountMaster) -> DomainResult<()>;

    /// 補助科目マスタを削除
    async fn delete(&self, code: &SubsidiaryAccountCode) -> DomainResult<()>;
}
