// AccountMaster - 勘定科目マスタ操作リクエスト

/// 勘定科目マスタ登録リクエスト
#[derive(Debug, Clone)]
pub struct RegisterAccountMasterRequest {
    pub code: String,
    pub name: String,
    pub account_type: String, // "Asset", "Liability", "Equity", "Revenue", "Expense"
}

/// 勘定科目マスタ更新リクエスト
#[derive(Debug, Clone)]
pub struct UpdateAccountMasterRequest {
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

/// 勘定科目マスタ削除リクエスト
#[derive(Debug, Clone)]
pub struct DeleteAccountMasterRequest {
    pub code: String,
}
