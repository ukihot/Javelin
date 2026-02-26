// CompanyMaster - 会社マスタ操作リクエスト

/// 会社マスタ取得リクエスト
#[derive(Debug, Clone)]
pub struct LoadCompanyMasterRequest {
    /// フィルタ条件（オプション）
    pub filter: Option<String>,
    /// アクティブのみ取得
    pub active_only: bool,
}

/// 会社マスタ登録リクエスト
#[derive(Debug, Clone)]
pub struct RegisterCompanyMasterRequest {
    pub code: String,
    pub name: String,
}

/// 会社マスタ更新リクエスト
#[derive(Debug, Clone)]
pub struct UpdateCompanyMasterRequest {
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

/// 会社マスタ削除リクエスト
#[derive(Debug, Clone)]
pub struct DeleteCompanyMasterRequest {
    pub code: String,
}
