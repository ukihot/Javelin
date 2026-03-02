// 仕訳検索QueryService trait
// 検索専用のQueryService抽象化

use crate::{
    dtos::{request::SearchCriteriaDto, response::JournalEntrySearchResultDto},
    error::ApplicationResult,
};

/// 仕訳検索QueryService
///
/// 検索条件を受け取り、Projectionから仕訳データを取得する。
/// リポジトリにfindBy系メソッドを追加せず、検索専用のQueryServiceとして実装する。
#[allow(async_fn_in_trait)]
pub trait JournalEntrySearchQueryService: Send + Sync {
    /// 仕訳を検索
    ///
    /// # Arguments
    /// * `criteria` - 検索条件
    ///
    /// # Returns
    /// * `ApplicationResult<JournalEntrySearchResultDto>` - 検索結果
    async fn search(
        &self,
        criteria: SearchCriteriaDto,
    ) -> ApplicationResult<JournalEntrySearchResultDto>;

    /// 指定された会計年度の既存伝票番号リストを取得
    ///
    /// # Arguments
    /// * `fiscal_year` - 会計年度（例: 2024）
    ///
    /// # Returns
    /// * `ApplicationResult<Vec<String>>` - 伝票番号リスト
    async fn get_voucher_numbers_by_fiscal_year(
        &self,
        fiscal_year: u32,
    ) -> ApplicationResult<Vec<String>>;

    /// 仕訳詳細を取得
    ///
    /// # Arguments
    /// * `entry_id` - 仕訳ID
    ///
    /// # Returns
    /// * `ApplicationResult<Option<JournalEntryDetail>>` - 仕訳詳細（存在しない場合はNone）
    async fn get_detail(
        &self,
        entry_id: &str,
    ) -> ApplicationResult<Option<crate::dtos::response::JournalEntryDetail>>;
}
