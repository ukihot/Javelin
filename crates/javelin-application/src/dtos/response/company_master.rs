// CompanyMaster - 会社マスタ操作レスポンス

use serde::{Deserialize, Serialize};

/// 会社マスタ取得レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadCompanyMasterResponse {
    pub companies: Vec<CompanyMasterItem>,
}

/// 会社マスタ項目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyMasterItem {
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

/// 会社マスタ登録レスポンス
#[derive(Debug, Clone)]
pub struct RegisterCompanyMasterResponse {
    pub code: String,
    pub message: String,
}

/// 会社マスタ更新レスポンス
#[derive(Debug, Clone)]
pub struct UpdateCompanyMasterResponse {
    pub code: String,
    pub message: String,
}

/// 会社マスタ削除レスポンス
#[derive(Debug, Clone)]
pub struct DeleteCompanyMasterResponse {
    pub code: String,
    pub message: String,
}
