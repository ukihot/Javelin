// SubsidiaryAccountMaster - 補助科目マスタ操作リクエスト

/// 補助科目マスタ取得リクエスト
#[derive(Debug, Clone)]
pub struct LoadSubsidiaryAccountMasterRequest {
    /// フィルタ条件（オプション）
    pub filter: Option<String>,
    /// アクティブのみ取得
    pub active_only: bool,
}

/// 補助科目マスタ登録リクエスト
#[derive(Debug, Clone)]
pub struct RegisterSubsidiaryAccountMasterRequest {
    pub code: String,
    pub name: String,
    pub parent_account_code: String,
}

/// 補助科目マスタ更新リクエスト
#[derive(Debug, Clone)]
pub struct UpdateSubsidiaryAccountMasterRequest {
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

/// 補助科目マスタ削除リクエスト
#[derive(Debug, Clone)]
pub struct DeleteSubsidiaryAccountMasterRequest {
    pub code: String,
}
