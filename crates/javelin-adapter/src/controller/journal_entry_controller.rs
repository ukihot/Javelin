// JournalEntryController実装
// 仕訳登録に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::{
    dtos::RegisterJournalEntryRequest, query_service::JournalEntrySearchQueryService,
};
use javelin_infrastructure::write::event_store::EventStore;

use crate::navigation::PresenterRegistry;

/// 仕訳登録コントローラ
///
/// 仕訳登録に関するすべての操作を受け付ける。
/// ユースケースへの委譲のみを行い、ビジネスロジックは含まない。
pub struct JournalEntryController<Q>
where
    Q: JournalEntrySearchQueryService,
{
    event_store: Arc<EventStore>,
    query_service: Arc<Q>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<Q> JournalEntryController<Q>
where
    Q: JournalEntrySearchQueryService,
{
    /// 新しいコントローラインスタンスを作成
    pub fn new(
        event_store: Arc<EventStore>,
        query_service: Arc<Q>,
        presenter_registry: Arc<PresenterRegistry>,
    ) -> Self {
        Self { event_store, query_service, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    /// 仕訳を登録（下書き作成）
    ///
    /// # Arguments
    /// * `page_id` - ページインスタンスID（PresenterRegistry検索用）
    /// * `request` - 登録リクエスト
    ///
    /// # Returns
    /// * `Ok(())` - 登録成功（結果はOutputPort経由で通知）
    /// * `Err(String)` - 登録失敗
    pub async fn handle_register_journal_entry(
        &self,
        page_id: uuid::Uuid,
        request: RegisterJournalEntryRequest,
    ) -> Result<(), String> {
        // PresenterRegistryから該当ページのPresenterを取得
        let presenter = self
            .presenter_registry
            .get_journal_entry_presenter(page_id)
            .ok_or_else(|| format!("Journal entry presenter not found for page_id: {}", page_id))?;

        // 取得したPresenterを使って新しいInteractorを作成
        let interactor = javelin_application::interactor::RegisterJournalEntryInteractor::new(
            Arc::clone(&self.event_store),
            presenter,
            Arc::clone(&self.query_service),
        );

        // UseCaseに委譲
        use javelin_application::input_ports::RegisterJournalEntryUseCase;
        interactor.execute(request).await.map_err(|e| e.to_string())
    }
}
