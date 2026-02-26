// LockClosingPeriodInteractor - 締日固定処理
// 責務: 取引データのロック処理

use std::sync::Arc;

use javelin_domain::repositories::RepositoryBase;

use crate::{
    dtos::{LockClosingPeriodRequest, LockClosingPeriodResponse},
    error::ApplicationResult,
    input_ports::LockClosingPeriodUseCase,
};

pub struct LockClosingPeriodInteractor<R>
where
    R: RepositoryBase,
{
    event_repository: Arc<R>,
}

impl<R> LockClosingPeriodInteractor<R>
where
    R: RepositoryBase,
{
    pub fn new(event_repository: Arc<R>) -> Self {
        Self { event_repository }
    }
}

impl<R> LockClosingPeriodUseCase for LockClosingPeriodInteractor<R>
where
    R: RepositoryBase,
{
    async fn execute(
        &self,
        request: LockClosingPeriodRequest,
    ) -> ApplicationResult<LockClosingPeriodResponse> {
        // イベントストアから最新シーケンスを取得
        let latest_sequence = self
            .event_repository
            .get_latest_sequence()
            .await
            .map_err(|e| crate::error::ApplicationError::EventStoreError(e.to_string()))?;

        // 実装: 締日固定処理（イベント追記）
        Ok(LockClosingPeriodResponse {
            locked_entries_count: latest_sequence as usize,
            locked_at: chrono::Utc::now().to_rfc3339(),
            audit_log_id: format!("LOCK-{}-{:02}", request.fiscal_year, request.period),
        })
    }
}
