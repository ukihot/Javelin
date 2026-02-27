// AccountMasterRepository - 勘定科目マスタリポジトリトレイト

use crate::{
    error::DomainResult,
    masters::{AccountCode, AccountMaster},
};

/// 勘定科目マスタリポジトリトレイト
///
/// CQRS原則: Repositoryはイベント永続化のみを担当
/// 読み取りはQueryServiceを使用すること
#[allow(async_fn_in_trait)]
pub trait AccountMasterRepository: Send + Sync {
    /// 勘定科目マスタを保存
    async fn save(&self, account_master: &AccountMaster) -> DomainResult<()>;

    /// 勘定科目マスタを削除
    async fn delete(&self, code: &AccountCode) -> DomainResult<()>;
}
