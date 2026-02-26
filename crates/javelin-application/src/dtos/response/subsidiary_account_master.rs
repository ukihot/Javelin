// SubsidiaryAccountMaster - 補助科目マスタ操作レスポンス

use serde::{Deserialize, Serialize};

/// 補助科目マスタ取得レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadSubsidiaryAccountMasterResponse {
    pub accounts: Vec<SubsidiaryAccountMasterItem>,
}

/// 補助科目マスタ項目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsidiaryAccountMasterItem {
    pub code: String,
    pub name: String,
    pub parent_account_code: String,
    pub is_active: bool,
}

/// 補助科目マスタ登録レスポンス
#[derive(Debug, Clone)]
pub struct RegisterSubsidiaryAccountMasterResponse {
    pub code: String,
    pub message: String,
}

/// 補助科目マスタ更新レスポンス
#[derive(Debug, Clone)]
pub struct UpdateSubsidiaryAccountMasterResponse {
    pub code: String,
    pub message: String,
}

/// 補助科目マスタ削除レスポンス
#[derive(Debug, Clone)]
pub struct DeleteSubsidiaryAccountMasterResponse {
    pub code: String,
    pub message: String,
}
