// Projection適用をTraitで統一
// TryFrom による変換集約

use crate::{error::InfrastructureResult, event_stream::StoredEvent};

/// イベント適用Trait - CQRSの核
pub trait Apply<E> {
    fn apply(&mut self, event: E) -> InfrastructureResult<()>;
}

/// ReadModelへの変換Trait
pub trait ToReadModel {
    type ReadModel;

    fn to_read_model(&self) -> Self::ReadModel;
}

/// イベントからReadModelへの変換
pub trait ProjectEvent {
    type Event;
    type ReadModel;

    fn project(event: &Self::Event) -> InfrastructureResult<Self::ReadModel>;
}

/// Projection更新戦略
pub trait ProjectionStrategy {
    /// Projectionを更新すべきか判定
    fn should_update(&self, event: &StoredEvent) -> bool;

    /// バッチサイズ
    fn batch_size(&self) -> usize {
        100
    }
}

/// デフォルトProjection戦略
pub struct DefaultProjectionStrategy;

impl ProjectionStrategy for DefaultProjectionStrategy {
    fn should_update(&self, _event: &StoredEvent) -> bool {
        true // 全イベントを処理
    }
}

/// イベントタイプフィルタ戦略
pub struct EventTypeFilterStrategy {
    pub allowed_types: Vec<String>,
}

impl ProjectionStrategy for EventTypeFilterStrategy {
    fn should_update(&self, event: &StoredEvent) -> bool {
        self.allowed_types.contains(&event.event_type)
    }
}

/// Aggregate IDフィルタ戦略
pub struct AggregateFilterStrategy {
    pub aggregate_ids: Vec<String>,
}

impl ProjectionStrategy for AggregateFilterStrategy {
    fn should_update(&self, event: &StoredEvent) -> bool {
        self.aggregate_ids.contains(&event.aggregate_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_filter() {
        let strategy = EventTypeFilterStrategy {
            allowed_types: vec!["UserCreated".to_string(), "UserUpdated".to_string()],
        };

        let event1 = StoredEvent {
            global_sequence: 1,
            event_type: "UserCreated".to_string(),
            aggregate_id: "user-1".to_string(),
            version: 1,
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            payload: vec![],
        };

        let event2 = StoredEvent {
            global_sequence: 2,
            event_type: "OrderCreated".to_string(),
            aggregate_id: "order-1".to_string(),
            version: 1,
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            payload: vec![],
        };

        assert!(strategy.should_update(&event1));
        assert!(!strategy.should_update(&event2));
    }
}
