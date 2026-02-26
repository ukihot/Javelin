// SearchJournalEntryUseCase - 仕訳検索ユースケース
// InputPort定義

use crate::{dtos::request::SearchCriteriaDto, error::ApplicationResult};

/// 仕訳検索ユースケース
///
/// 検索条件を受け取り、条件に合致する仕訳を検索する。
#[allow(async_fn_in_trait)]
pub trait SearchJournalEntryUseCase: Send + Sync {
    /// 仕訳を検索
    ///
    /// # Arguments
    /// * `criteria` - 検索条件
    ///
    /// # Returns
    /// * `ApplicationResult<()>` - 成功時はOutputPortを通じて結果を通知
    async fn execute(&self, criteria: SearchCriteriaDto) -> ApplicationResult<()>;
}
