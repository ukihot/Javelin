// 会社マスタQueryService trait

use javelin_domain::company::{CompanyCode, CompanyMaster};

use crate::{
    dtos::{request::FetchCompanyMasterRequest, response::FetchCompanyMasterResponse},
    error::ApplicationResult,
};

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

    /// 会社マスタを取得（DTO形式）
    /// Controller層から直接呼び出される
    async fn fetch_company_master(
        &self,
        request: FetchCompanyMasterRequest,
    ) -> ApplicationResult<FetchCompanyMasterResponse>;
}
