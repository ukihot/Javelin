// AccountMaster - 勘定科目マスタ操作レスポンス

/// 勘定科目マスタ登録レスポンス
#[derive(Debug, Clone)]
pub struct RegisterAccountMasterResponse {
    pub code: String,
    pub message: String,
}

/// 勘定科目マスタ更新レスポンス
#[derive(Debug, Clone)]
pub struct UpdateAccountMasterResponse {
    pub code: String,
    pub message: String,
}

/// 勘定科目マスタ削除レスポンス
#[derive(Debug, Clone)]
pub struct DeleteAccountMasterResponse {
    pub code: String,
    pub message: String,
}
