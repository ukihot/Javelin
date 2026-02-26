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
}
