// AccountingPeriodRepository実装
// Event Sourcingパターンに基づく会計期間の永続化

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    event_store::EventStore,
    types::Sequence,
};

/// 会計期間ドメインイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum AccountingPeriodEvent {
    /// 期間作成
    PeriodCreated {
        period_id: String,
        fiscal_year: u32,
        period: u32,
        start_date: String,
        end_date: String,
        created_by: String,
        created_at: String,
    },

    /// 期間締め
    PeriodClosed { period_id: String, closed_by: String, closed_at: String },

    /// 期間ロック
    PeriodLocked { period_id: String, locked_by: String, locked_at: String },

    /// 期間再オープン
    PeriodReopened { period_id: String, reason: String, reopened_by: String, reopened_at: String },
}

impl AccountingPeriodEvent {
    /// イベントタイプを取得
    pub fn event_type(&self) -> &str {
        match self {
            AccountingPeriodEvent::PeriodCreated { .. } => "PeriodCreated",
            AccountingPeriodEvent::PeriodClosed { .. } => "PeriodClosed",
            AccountingPeriodEvent::PeriodLocked { .. } => "PeriodLocked",
            AccountingPeriodEvent::PeriodReopened { .. } => "PeriodReopened",
        }
    }

    /// 集約IDを取得
    pub fn aggregate_id(&self) -> &str {
        match self {
            AccountingPeriodEvent::PeriodCreated { period_id, .. }
            | AccountingPeriodEvent::PeriodClosed { period_id, .. }
            | AccountingPeriodEvent::PeriodLocked { period_id, .. }
            | AccountingPeriodEvent::PeriodReopened { period_id, .. } => period_id,
        }
    }
}

/// AccountingPeriodRepository実装
///
/// Event Sourcingパターンに基づき、会計期間のイベントを
/// EventStoreに永続化する。
pub struct AccountingPeriodRepositoryImpl {
    event_store: Arc<EventStore>,
}

impl AccountingPeriodRepositoryImpl {
    /// 新しいリポジトリインスタンスを作成
    pub fn new(event_store: Arc<EventStore>) -> Self {
        Self { event_store }
    }

    /// イベントストリームから会計期間を復元
    pub async fn load_events(
        &self,
        period_id: &str,
    ) -> InfrastructureResult<Vec<AccountingPeriodEvent>> {
        let agg_id = crate::types::AggregateId::parse(period_id)
            .map_err(InfrastructureError::DeserializationFailed)?;
        let stream = self.event_store.stream_aggregate_events(agg_id, Sequence::new(0));

        // モダンプラクティス: 初期キャパシティを確保（会計期間イベントは少数）
        let mut events = Vec::with_capacity(8);
        for event_result in stream.iter() {
            let stored_event = event_result?;
            let event: AccountingPeriodEvent = serde_json::from_slice(&stored_event.payload)
                .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;
            events.push(event);
        }

        Ok(events)
    }

    /// 最新バージョンを取得
    pub async fn get_latest_version(&self, period_id: &str) -> InfrastructureResult<u64> {
        let events = self.load_events(period_id).await?;
        Ok(events.len() as u64)
    }
}

// DISABLED: Old RepositoryBase interface - AccountingPeriod should not use event sourcing
// impl RepositoryBase for AccountingPeriodRepositoryImpl {
// type Event = AccountingPeriodEvent;
//
// async fn append(&self, event: Self::Event) -> DomainResult<()> {
// ... implementation commented out ...
// }
//
// async fn append_events<T>(&self, aggregate_id: &str, events: Vec<T>) -> DomainResult<u64>
// where
// T: serde::Serialize + Send + 'static,
// {
// ... implementation commented out ...
// }
//
// async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>> {
// ... implementation commented out ...
// }
//
// async fn get_all_events(&self, from_sequence: u64) -> DomainResult<Vec<serde_json::Value>> {
// ... implementation commented out ...
// }
//
// async fn get_latest_sequence(&self) -> DomainResult<u64> {
// ... implementation commented out ...
// }
// }

// DomainEventトレイト実装
impl javelin_domain::event::DomainEvent for AccountingPeriodEvent {
    fn event_type(&self) -> &str {
        self.event_type()
    }

    fn aggregate_id(&self) -> &str {
        self.aggregate_id()
    }

    fn version(&self) -> u64 {
        0
    }
}
