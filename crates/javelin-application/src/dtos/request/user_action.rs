// ユーザ操作記録 - Request DTOs

#[derive(Debug, Clone)]
pub struct RecordUserActionRequest {
    pub user: String,
    pub location: String,
    pub action: String,
}
