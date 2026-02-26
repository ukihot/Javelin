// AccountMasterRepository - 勘定科目マスタリポジトリトレイト

use crate::{
    error::DomainResult,
    masters::{AccountCode, AccountMaster},
};

/// 勘定科目マスタリポジトリトレイト
#[allow(async_fn_in_trait)]
pub trait AccountMasterRepository: Send + Sync {
    /// 勘定科目マスタを取得
    async fn find_by_code(&self, code: &AccountCode) -> DomainResult<Option<AccountMaster>>;

    /// すべての勘定科目マスタを取得
    async fn find_all(&self) -> DomainResult<Vec<AccountMaster>>;

    /// 勘定科目マスタを保存
    async fn save(&self, account_master: &AccountMaster) -> DomainResult<()>;

    /// 勘定科目マスタを削除
    async fn delete(&self, code: &AccountCode) -> DomainResult<()>;
}
