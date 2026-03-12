// 勘定科目マスタQueryService trait

use javelin_domain::chart_of_accounts::{AccountCode, AccountMaster};

use crate::{
    dtos::{request::FetchAccountMasterRequest, response::FetchAccountMasterResponse},
    error::ApplicationResult,
};

/// 勘定科目マスタQueryService
///
/// CQRS原則: 読み取り専用のQueryService
/// ProjectionDBから勘定科目マスタデータを取得する
///
/// ディメンションテーブル（マスタデータ）として機能し、
/// ファクトテーブル（仕訳など）と結合して使用される
#[allow(async_fn_in_trait)]
pub trait AccountMasterQueryService: Send + Sync {
    /// すべての勘定科目マスタを取得
    async fn get_all(&self) -> ApplicationResult<Vec<AccountMaster>>;

    /// コードで勘定科目マスタを取得
    async fn get_by_code(&self, code: &AccountCode) -> ApplicationResult<Option<AccountMaster>>;

    /// 勘定科目マスタを取得（DTO形式）
    /// Controller層から直接呼び出される
    async fn fetch_account_master(
        &self,
        request: FetchAccountMasterRequest,
    ) -> ApplicationResult<FetchAccountMasterResponse>;

    /// フィルタリングして勘定科目マスタを取得
    ///
    /// # Arguments
    /// * `active_only` - アクティブな勘定科目のみを取得
    /// * `filter` - コードまたは名称での部分一致フィルタ
    async fn get_filtered(
        &self,
        active_only: bool,
        filter: Option<String>,
    ) -> ApplicationResult<Vec<AccountMaster>> {
        let accounts = self.get_all().await?;

        let filtered: Vec<AccountMaster> = accounts
            .into_iter()
            .filter(|acc| {
                // アクティブフィルタ
                if active_only && !acc.is_active() {
                    return false;
                }
                // テキストフィルタ
                if let Some(ref f) = filter {
                    let f_lower = f.to_lowercase();
                    return acc.code().value().to_lowercase().contains(&f_lower)
                        || acc.name().value().to_lowercase().contains(&f_lower);
                }
                true
            })
            .collect();

        Ok(filtered)
    }
}
