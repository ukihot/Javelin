// 勘定科目マスタQueryService trait

use javelin_domain::masters::{AccountCode, AccountMaster};

use crate::error::ApplicationResult;

/// 勘定科目マスタQueryService
///
/// CQRS原則: 読み取り専用のQueryService
/// ProjectionDBから勘定科目マスタデータを取得する
#[allow(async_fn_in_trait)]
pub trait AccountMasterQueryService: Send + Sync {
    /// すべての勘定科目マスタを取得
    async fn get_all(&self) -> ApplicationResult<Vec<AccountMaster>>;

    /// コードで勘定科目マスタを取得
    async fn get_by_code(&self, code: &AccountCode) -> ApplicationResult<Option<AccountMaster>>;
}
