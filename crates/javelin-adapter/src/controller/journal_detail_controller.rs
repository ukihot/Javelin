// JournalDetailController実装
// 仕訳詳細取得に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::query_service::JournalEntrySearchQueryService;

use crate::navigation::PresenterRegistry;

/// 仕訳詳細取得コントローラ
///
/// 仕訳詳細取得に関するすべての操作を受け付ける。
/// ユースケースへの委譲のみを行い、ビジネスロジックは含まない。
pub struct JournalDetailController<Q>
where
    Q: JournalEntrySearchQueryService,
{
    query_service: Arc<Q>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<Q> JournalDetailController<Q>
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

    /// 仕訳詳細を取得
    ///
    /// # Arguments
    /// * `page_id` - ページインスタンスID（PresenterRegistry検索用）
    /// * `entry_id` - 仕訳ID
    ///
    /// # Returns
    /// * `Ok(())` - 取得成功（結果はOutputPort経由で通知）
    /// * `Err(String)` - 取得失敗
    pub async fn handle_get_journal_entry_detail(
        &self,
        page_id: uuid::Uuid,
        entry_id: String,
    ) -> Result<(), String> {
        // PresenterRegistryから該当ページのPresenterを取得
        let presenter = self
            .presenter_registry
            .get_journal_entry_presenter(page_id)
            .ok_or_else(|| format!("Journal entry presenter not found for page_id: {}", page_id))?;

        // 取得したPresenterを使って新しいInteractorを作成
        let interactor = javelin_application::interactor::GetJournalEntryDetailInteractor::new(
            Arc::clone(&self.query_service),
            presenter,
        );

        // UseCaseに委譲
        use javelin_application::input_ports::GetJournalEntryDetailUseCase;
        interactor.execute(entry_id).await.map_err(|e| e.to_string())
    }
}
