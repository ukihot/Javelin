// SearchController実装
// 仕訳検索に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::dtos::request::SearchCriteriaDto;
use javelin_infrastructure::read::query_services::JournalEntrySearchQueryServiceImpl;

use crate::navigation::PresenterRegistry;

/// 検索コントローラ
///
/// 仕訳検索に関するすべての操作を受け付ける。
/// ユースケースへの委譲のみを行い、ビジネスロジックは含まない。
pub struct SearchController {
    query_service: Arc<JournalEntrySearchQueryServiceImpl>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl SearchController {
    /// 新しいコントローラインスタンスを作成
    pub fn new(
        query_service: Arc<JournalEntrySearchQueryServiceImpl>,
        presenter_registry: Arc<PresenterRegistry>,
    ) -> Self {
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
        use javelin_application::input_ports::SearchJournalEntryUseCase;

        // PresenterRegistryからpage_id用のPresenterを取得
        if let Some(presenter_arc) = self.presenter_registry.get_search_presenter(page_id) {
            // ArcからPresenterをclone
            let presenter = (*presenter_arc).clone();

            // このページ専用のInteractorを動的に作成
            let interactor =
                javelin_application::interactor::journal_entry::SearchJournalEntryInteractor::new(
                    Arc::clone(&self.query_service),
                    presenter.into(),
                );

            // 実行
            interactor.execute(criteria).await.map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!("SearchPresenter not found for page_id: {}", page_id))
        }
    }
}
