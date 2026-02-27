// LedgerController実装
// 元帳・試算表照会に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::query_service::{
    GetLedgerQuery, GetTrialBalanceQuery, LedgerQueryService,
};
use javelin_infrastructure::read::query_services::LedgerQueryServiceImpl;

/// 元帳コントローラ（具体型版）
///
/// 型パラメータを削除し、具体的なQueryService型を直接保持することで
/// 他のControllerと統一したパターンに変更
pub struct LedgerController {
    ledger_query_service: Arc<LedgerQueryServiceImpl>,
}

impl LedgerController {
    /// 新しいコントローラインスタンスを作成
    pub fn new(ledger_query_service: Arc<LedgerQueryServiceImpl>) -> Self {
        Self { ledger_query_service }
    }

    /// 元帳を取得
    pub async fn get_ledger(&self, query: GetLedgerQuery) -> Result<(), String> {
        self.ledger_query_service
            .get_ledger(query)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    /// 試算表を取得
    pub async fn get_trial_balance(&self, query: GetTrialBalanceQuery) -> Result<(), String> {
        self.ledger_query_service
            .get_trial_balance(query)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
