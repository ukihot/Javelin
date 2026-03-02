// SearchController実装
// 仕訳検索に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::{
    dtos::request::SearchCriteriaDto, input_ports::SearchJournalEntryUseCase,
};

use crate::navigation::PresenterRegistry;

/// 検索コントローラ
///
/// 仕訳検索に関するすべての操作を受け付ける。
/// ユースケースへの委譲のみを行い、ビジネスロジックは含まない。
pub struct SearchController<S>
where
    S: SearchJournalEntryUseCase,
{
    search_use_case: Arc<S>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<S> SearchController<S>
where
    S: SearchJournalEntryUseCase,
{
    /// 新しいコントローラインスタンスを作成
    pub fn new(search_use_case: Arc<S>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { search_use_case, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// 仕訳を検索
    ///
    /// # Arguments
    /// * `page_id` - ページインスタンスID（PresenterRegistry検索用）
    /// * `criteria` - 検索条件
    ///
    /// # Returns
    /// * `Ok(())` - 検索成功（結果はOutputPort経由で通知）
    /// * `Err(String)` - 検索失敗
    pub async fn handle_search(
        &self,
        _page_id: uuid::Uuid,
        criteria: SearchCriteriaDto,
    ) -> Result<(), String> {
        // UseCaseに委譲
        self.search_use_case.execute(criteria).await.map_err(|e| e.to_string())
    }
}
