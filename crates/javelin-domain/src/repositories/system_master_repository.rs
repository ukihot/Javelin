// SystemMasterRepository - システムマスタリポジトリトレイト

use crate::{
    error::DomainResult,
    system_masters::{SystemMaster, SystemMasterId},
};

/// システムマスタリポジトリトレイト
///
/// CQRS原則: Repositoryはイベント永続化のみを担当
/// 読み取りはQueryServiceを使用すること
#[allow(async_fn_in_trait)]
pub trait SystemMasterRepository: Send + Sync {
    /// システムマスタを保存
    async fn save(&self, system_master: &SystemMaster) -> DomainResult<()>;

    /// システムマスタを削除
    async fn delete(&self, id: &SystemMasterId) -> DomainResult<()>;
}
