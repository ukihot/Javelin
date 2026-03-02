// JournalEntryController実装
// 仕訳登録に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::{
    dtos::RegisterJournalEntryRequest, input_ports::RegisterJournalEntryUseCase,
};

use crate::navigation::PresenterRegistry;

/// 仕訳登録コントローラ
///
/// 仕訳登録に関するすべての操作を受け付ける。
/// ユースケースへの委譲のみを行い、ビジネスロジックは含まない。
pub struct JournalEntryController<R>
where
    R: RegisterJournalEntryUseCase,
{
    register_use_case: Arc<R>,
    presenter_registry: Arc<PresenterRegistry>,
}

impl<R> JournalEntryController<R>
where
    R: RegisterJournalEntryUseCase,
{
    /// 新しいコントローラインスタンスを作成
    pub fn new(register_use_case: Arc<R>, presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { register_use_case, presenter_registry }
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
        _page_id: uuid::Uuid,
        request: RegisterJournalEntryRequest,
    ) -> Result<(), String> {
        // UseCaseに委譲
        self.register_use_case.execute(request).await.map_err(|e| e.to_string())
    }
}
