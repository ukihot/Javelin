// SubsidiaryAccountMasterRepository - 補助科目マスタリポジトリトレイト

use crate::{
    error::DomainResult,
    masters::{AccountCode, SubsidiaryAccountCode, SubsidiaryAccountMaster},
};

/// 補助科目マスタリポジトリトレイト
#[allow(async_fn_in_trait)]
pub trait SubsidiaryAccountMasterRepository: Send + Sync {
    /// 補助科目マスタを取得
    async fn find_by_code(
        &self,
        code: &SubsidiaryAccountCode,
    ) -> DomainResult<Option<SubsidiaryAccountMaster>>;

    /// 親勘定科目に紐づく補助科目マスタを取得
    async fn find_by_parent_account(
        &self,
        parent_account_code: &AccountCode,
    ) -> DomainResult<Vec<SubsidiaryAccountMaster>>;

    /// すべての補助科目マスタを取得
    async fn find_all(&self) -> DomainResult<Vec<SubsidiaryAccountMaster>>;

    /// 補助科目マスタを保存
    async fn save(&self, subsidiary_account_master: &SubsidiaryAccountMaster) -> DomainResult<()>;

    /// 補助科目マスタを削除
    async fn delete(&self, code: &SubsidiaryAccountCode) -> DomainResult<()>;
}
