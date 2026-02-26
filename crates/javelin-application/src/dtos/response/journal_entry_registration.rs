// 仕訳登録ユースケース - Response DTOs
// すべてのプロパティはプリミティブ型

/// 仕訳登録レスポンス（下書き作成）
#[derive(Debug, Clone)]
pub struct RegisterJournalEntryResponse {
    pub entry_id: String,
    pub status: String,
}

/// 承認申請レスポンス
#[derive(Debug, Clone)]
pub struct SubmitForApprovalResponse {
    pub entry_id: String,
    pub status: String,
    pub submitted_at: String, // ISO 8601 format
}

/// 承認レスポンス
#[derive(Debug, Clone)]
pub struct ApproveJournalEntryResponse {
    pub entry_id: String,
    pub entry_number: String,
    pub status: String,
    pub approved_at: String, // ISO 8601 format
}

/// 差戻しレスポンス
#[derive(Debug, Clone)]
pub struct RejectJournalEntryResponse {
    pub entry_id: String,
    pub status: String,
    pub rejected_at: String, // ISO 8601 format
}

/// 取消レスポンス
#[derive(Debug, Clone)]
pub struct ReverseJournalEntryResponse {
    pub entry_id: String,
    pub original_entry_id: String,
    pub status: String,
    pub reversed_at: String, // ISO 8601 format
}

/// 修正レスポンス
#[derive(Debug, Clone)]
pub struct CorrectJournalEntryResponse {
    pub entry_id: String,
    pub reversed_entry_id: String,
    pub status: String,
    pub corrected_at: String, // ISO 8601 format
}

/// 下書き更新レスポンス
#[derive(Debug, Clone)]
pub struct UpdateDraftJournalEntryResponse {
    pub entry_id: String,
    pub status: String,
    pub updated_at: String, // ISO 8601 format
}

/// 下書き削除レスポンス
#[derive(Debug, Clone)]
pub struct DeleteDraftJournalEntryResponse {
    pub entry_id: String,
    pub deleted_at: String, // ISO 8601 format
}
