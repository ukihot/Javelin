// 補助科目マスタQueryService trait

use javelin_domain::chart_of_accounts::{
    AccountCode, SubsidiaryAccountCode, SubsidiaryAccountMaster,
};

use crate::{
    dtos::{
        request::FetchSubsidiaryAccountMasterRequest,
        response::FetchSubsidiaryAccountMasterResponse,
    },
    error::ApplicationResult,
};

/// 補助科目マスタQueryService
///
/// CQRS原則: 読み取り専用のQueryService
/// ProjectionDBから補助科目マスタデータを取得する
#[allow(async_fn_in_trait)]
pub trait SubsidiaryAccountMasterQueryService: Send + Sync {
    /// すべての補助科目マスタを取得
    async fn get_all(&self) -> ApplicationResult<Vec<SubsidiaryAccountMaster>>;

    /// コードで補助科目マスタを取得
    async fn get_by_code(
        &self,
        code: &SubsidiaryAccountCode,
    ) -> ApplicationResult<Option<SubsidiaryAccountMaster>>;

    /// 親勘定科目に紐づく補助科目マスタを取得
    async fn get_by_parent_account(
        &self,
        parent_account_code: &AccountCode,
    ) -> ApplicationResult<Vec<SubsidiaryAccountMaster>>;

    /// 補助科目マスタを取得（DTO形式）
    /// Controller層から直接呼び出される
    async fn fetch_subsidiary_account_master(
        &self,
        request: FetchSubsidiaryAccountMasterRequest,
    ) -> ApplicationResult<FetchSubsidiaryAccountMasterResponse>;
}
