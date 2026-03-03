// SearchController実装
// 仕訳検索に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::{
    dtos::request::SearchCriteriaDto,
    query_service::JournalEntrySearchQueryService,
};

use crate::navigation::PresenterRegistry;

/// 検索コントローラ
///
/// 仕訳検索に関するすべての操作を受け付ける。
/// ユースケースへの委譲のみを行い、ビジネスロジックは含まない。
pub struct SearchController<Q>
where
    Q: JournalEntrySearchQueryService,
{
    query_service: Arc<Q>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<Q> SearchController<Q>
where
    Q: JournalEntrySearchQueryService,
{
    /// 新しいコントローラインスタンスを作成
    pub fn new(query_service: Arc<Q>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { query_service, presenter_registry }
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
        page_id: uuid::Uuid,
        criteria: SearchCriteriaDto,
    ) -> Result<(), String> {
        // PresenterRegistryから該当ページのPresenterを取得
        let presenter = self
            .presenter_registry
            .get_search_presenter(page_id)
            .ok_or_else(|| format!("Search presenter not found for page_id: {}", page_id))?;

        // 取得したPresenterを使って新しいInteractorを作成
        let interactor = javelin_application::interactor::SearchJournalEntryInteractor::new(
            Arc::clone(&self.query_service),
            presenter,
        );

        // UseCaseに委譲
        use javelin_application::input_ports::SearchJournalEntryUseCase;
        interactor.execute(criteria).await.map_err(|e| e.to_string())
    }
}
