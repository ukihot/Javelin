// ユーザ操作記録 - Response DTOs
// すべてのプロパティはプリミティブ型

#[derive(Debug, Clone)]
pub struct RecordUserActionResponse {
    pub action_id: String,
    pub recorded_at: String, // ISO 8601 format
}
