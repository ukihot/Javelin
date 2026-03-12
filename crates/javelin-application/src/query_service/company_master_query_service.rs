// 会社マスタQueryService trait

use javelin_domain::company::{CompanyCode, CompanyMaster};

use crate::error::ApplicationResult;

/// 会社マスタQueryService
///
/// CQRS原則: 読み取り専用のQueryService
/// ProjectionDBから会社マスタデータを取得する
#[allow(async_fn_in_trait)]
pub trait CompanyMasterQueryService: Send + Sync {
    /// すべての会社マスタを取得
    async fn get_all(&self) -> ApplicationResult<Vec<CompanyMaster>>;

    /// コードで会社マスタを取得
    async fn get_by_code(&self, code: &CompanyCode) -> ApplicationResult<Option<CompanyMaster>>;
}
