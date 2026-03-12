// LockClosingPeriodInteractor - 締日固定処理
// 責務: 取引データのロック処理

use crate::{
    dtos::{LockClosingPeriodRequest, LockClosingPeriodResponse},
    error::ApplicationResult,
    input_ports::LockClosingPeriodUseCase,
};

pub struct LockClosingPeriodInteractor {
    // NOTE: 現在はイベントストアを直接使用せず、簡易実装
}

impl LockClosingPeriodInteractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LockClosingPeriodInteractor {
    fn default() -> Self {
        Self::new()
    }
}

impl LockClosingPeriodUseCase for LockClosingPeriodInteractor {
    async fn execute(
        &self,
        request: LockClosingPeriodRequest,
    ) -> ApplicationResult<LockClosingPeriodResponse> {
        // 実装: 締日固定処理
        // TODO: 実際のロック処理を実装
        Ok(LockClosingPeriodResponse {
            locked_entries_count: 0,
            locked_at: chrono::Utc::now().to_rfc3339(),
            audit_log_id: format!("LOCK-{}-{:02}", request.fiscal_year, request.period),
        })
    }
}
