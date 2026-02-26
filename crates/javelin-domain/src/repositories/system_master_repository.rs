// SystemMasterRepository - システムマスタリポジトリトレイト

use crate::{
    error::DomainResult,
    system_masters::{SystemMaster, SystemMasterId},
};

/// システムマスタリポジトリトレイト
#[allow(async_fn_in_trait)]
pub trait SystemMasterRepository: Send + Sync {
    /// システムマスタを取得
    async fn find_by_id(&self, id: &SystemMasterId) -> DomainResult<Option<SystemMaster>>;

    /// システムマスタを保存
    async fn save(&self, system_master: &SystemMaster) -> DomainResult<()>;

    /// デフォルトのシステムマスタを取得
    async fn find_default(&self) -> DomainResult<Option<SystemMaster>>;

    /// すべてのシステムマスタを取得
    async fn find_all(&self) -> DomainResult<Vec<SystemMaster>>;

    /// システムマスタを削除
    async fn delete(&self, id: &SystemMasterId) -> DomainResult<()>;
}
