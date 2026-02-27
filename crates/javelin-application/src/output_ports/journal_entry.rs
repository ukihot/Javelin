use crate::dtos::response::{
    ApproveJournalEntryResponse, CorrectJournalEntryResponse, DeleteDraftJournalEntryResponse,
    RegisterJournalEntryResponse, RejectJournalEntryResponse, ReverseJournalEntryResponse,
    SubmitForApprovalResponse, UpdateDraftJournalEntryResponse,
};

/// JournalEntryOutputPort - 仕訳ユースケース結果の出力
#[allow(async_fn_in_trait)]
pub trait JournalEntryOutputPort: Send + Sync {
    /// 仕訳登録結果を出力
    async fn present_register_result(&self, response: RegisterJournalEntryResponse);

    /// 処理進捗を通知
    async fn notify_progress(&self, message: String);

    /// エラーを通知
    async fn notify_error(&self, error_message: String);

    /// 下書き更新結果を出力
    async fn present_update_draft_result(&self, response: UpdateDraftJournalEntryResponse);

    /// 承認申請結果を出力
    async fn present_submit_for_approval_result(&self, response: SubmitForApprovalResponse);

    /// 承認結果を出力
    async fn present_approve_result(&self, response: ApproveJournalEntryResponse);

    /// 差戻し結果を出力
    async fn present_reject_result(&self, response: RejectJournalEntryResponse);

    /// 取消結果を出力
    async fn present_reverse_result(&self, response: ReverseJournalEntryResponse);

    /// 修正結果を出力
    async fn present_correct_result(&self, response: CorrectJournalEntryResponse);

    /// 削除結果を出力
    async fn present_delete_draft_result(&self, response: DeleteDraftJournalEntryResponse);
}

#[cfg(test)]
pub use mock::MockJournalEntryOutputPort;

// テスト用のモック実装
#[cfg(test)]
pub mod mock {
    use mockall::mock;

    use super::*;

    mock! {
        pub JournalEntryOutputPort {}

        #[allow(async_fn_in_trait)]
        impl JournalEntryOutputPort for JournalEntryOutputPort {
            async fn present_register_result(&self, response: RegisterJournalEntryResponse);
            async fn notify_progress(&self, message: String);
            async fn notify_error(&self, error_message: String);
            async fn present_update_draft_result(&self, response: UpdateDraftJournalEntryResponse);
            async fn present_submit_for_approval_result(&self, response: SubmitForApprovalResponse);
            async fn present_approve_result(&self, response: ApproveJournalEntryResponse);
            async fn present_reject_result(&self, response: RejectJournalEntryResponse);
            async fn present_reverse_result(&self, response: ReverseJournalEntryResponse);
            async fn present_correct_result(&self, response: CorrectJournalEntryResponse);
            async fn present_delete_draft_result(&self, response: DeleteDraftJournalEntryResponse);
        }
    }
}
