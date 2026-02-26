// LoadAccountMasterResponse - 勘定科目マスタ取得レスポンス

use serde::{Deserialize, Serialize};

/// 勘定科目マスタ取得レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadAccountMasterResponse {
    /// 勘定科目リスト
    pub accounts: Vec<AccountMasterItem>,
}

/// 勘定科目マスタ項目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountMasterItem {
    /// 科目コード
    pub code: String,
    /// 科目名
    pub name: String,
    /// 科目タイプ
    pub account_type: String,
}
