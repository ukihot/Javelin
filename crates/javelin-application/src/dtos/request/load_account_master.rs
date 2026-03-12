// LoadAccountMasterRequest - 勘定科目マスタ取得リクエスト

use serde::{Deserialize, Serialize};

/// 勘定科目マスタ取得リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchAccountMasterRequest {
    /// フィルタ条件（オプション）
    pub filter: Option<String>,
    /// アクティブな科目のみ取得
    pub active_only: bool,
}

impl Default for FetchAccountMasterRequest {
    fn default() -> Self {
        Self { filter: None, active_only: true }
    }
}
