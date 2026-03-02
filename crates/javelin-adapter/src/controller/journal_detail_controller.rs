// JournalDetailController実装
// 仕訳詳細取得に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::input_ports::GetJournalEntryDetailUseCase;

use crate::navigation::PresenterRegistry;

/// 仕訳詳細取得コントローラ
///
/// 仕訳詳細取得に関するすべての操作を受け付ける。
/// ユースケースへの委譲のみを行い、ビジネスロジックは含まない。
pub struct JournalDetailController<G>
where
    G: GetJournalEntryDetailUseCase,
{
    get_detail_use_case: Arc<G>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<G> JournalDetailController<G>
where
    G: GetJournalEntryDetailUseCase,
{
    /// 新しいコントローラインスタンスを作成
    pub fn new(get_detail_use_case: Arc<G>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { get_detail_use_case, presenter_registry }
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
        _page_id: uuid::Uuid,
        entry_id: String,
    ) -> Result<(), String> {
        // UseCaseに委譲
        self.get_detail_use_case.execute(entry_id).await.map_err(|e| e.to_string())
    }
}
