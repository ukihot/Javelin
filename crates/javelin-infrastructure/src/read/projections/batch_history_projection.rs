// BatchHistoryProjection - バッチ実行履歴Projection

use javelin_domain::batch::events::BatchExecutionEvent;
use serde::{Deserialize, Serialize};

use crate::{
    error::InfrastructureResult,
    event_stream::StoredEvent,
    read::projections::projection_trait::{Apply, ProjectionStrategy, ToReadModel},
};

/// バッチ実行履歴ReadModel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchHistoryReadModel {
    pub execution_id: String,
    pub batch_type: String,
    pub status: String,
    pub started_at: String,
    pub started_by: String,
    pub completed_at: Option<String>,
    pub duration_seconds: Option<u32>,
    pub processed_count: Option<usize>,
    pub result_summary: Option<String>,
    pub error_message: Option<String>,
}

/// バッチ実行履歴Projection
#[derive(Debug, Clone)]
pub struct BatchHistoryProjection {
    execution_id: String,
    batch_type: String,
    status: String,
    started_at: String,
    started_by: String,
    completed_at: Option<String>,
    duration_seconds: Option<u32>,
    processed_count: Option<usize>,
    result_summary: Option<String>,
    error_message: Option<String>,
}

impl BatchHistoryProjection {
    pub fn new(execution_id: String) -> Self {
        Self {
            execution_id,
            batch_type: String::new(),
            status: "Running".to_string(),
            started_at: String::new(),
            started_by: String::new(),
            completed_at: None,
            duration_seconds: None,
            processed_count: None,
            result_summary: None,
            error_message: None,
        }
    }
}

impl Apply<BatchExecutionEvent> for BatchHistoryProjection {
    fn apply(&mut self, event: BatchExecutionEvent) -> InfrastructureResult<()> {
        match event {
            BatchExecutionEvent::BatchStarted {
                execution_id,
                batch_type,
                started_at,
                started_by,
            } => {
                self.execution_id = execution_id;
                self.batch_type = batch_type;
                self.status = "Running".to_string();
                self.started_at = started_at.to_rfc3339();
                self.started_by = started_by;
            }
            BatchExecutionEvent::BatchCompleted {
                completed_at,
                duration_seconds,
                processed_count,
                result_summary,
                ..
            } => {
                self.status = "Completed".to_string();
                self.completed_at = Some(completed_at.to_rfc3339());
                self.duration_seconds = Some(duration_seconds);
                self.processed_count = Some(processed_count);
                self.result_summary = Some(result_summary);
            }
            BatchExecutionEvent::BatchFailed {
                failed_at, duration_seconds, error_message, ..
            } => {
                self.status = "Failed".to_string();
                self.completed_at = Some(failed_at.to_rfc3339());
                self.duration_seconds = Some(duration_seconds);
                self.error_message = Some(error_message);
            }
        }

        Ok(())
    }
}

impl ToReadModel for BatchHistoryProjection {
    type ReadModel = BatchHistoryReadModel;

    fn to_read_model(&self) -> Self::ReadModel {
        BatchHistoryReadModel {
            execution_id: self.execution_id.clone(),
            batch_type: self.batch_type.clone(),
            status: self.status.clone(),
            started_at: self.started_at.clone(),
            started_by: self.started_by.clone(),
            completed_at: self.completed_at.clone(),
            duration_seconds: self.duration_seconds,
            processed_count: self.processed_count,
            result_summary: self.result_summary.clone(),
            error_message: self.error_message.clone(),
        }
    }
}

/// BatchHistoryProjection戦略
pub struct BatchHistoryProjectionStrategy;

impl ProjectionStrategy for BatchHistoryProjectionStrategy {
    fn should_update(&self, event: &StoredEvent) -> bool {
        matches!(event.event_type.as_str(), "BatchStarted" | "BatchCompleted" | "BatchFailed")
    }

    fn batch_size(&self) -> usize {
        100
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn test_batch_started() {
        let mut projection = BatchHistoryProjection::new("batch-001".to_string());

        let event = BatchExecutionEvent::BatchStarted {
            execution_id: "batch-001".to_string(),
            batch_type: "LedgerConsolidation".to_string(),
            started_at: Utc::now(),
            started_by: "system".to_string(),
        };

        projection.apply(event).unwrap();

        let read_model = projection.to_read_model();
        assert_eq!(read_model.execution_id, "batch-001");
        assert_eq!(read_model.batch_type, "LedgerConsolidation");
        assert_eq!(read_model.status, "Running");
    }

    #[test]
    fn test_batch_completed() {
        let mut projection = BatchHistoryProjection::new("batch-002".to_string());

        let start_event = BatchExecutionEvent::BatchStarted {
            execution_id: "batch-002".to_string(),
            batch_type: "ClosingPreparation".to_string(),
            started_at: Utc::now(),
            started_by: "system".to_string(),
        };
        projection.apply(start_event).unwrap();

        let complete_event = BatchExecutionEvent::BatchCompleted {
            execution_id: "batch-002".to_string(),
            completed_at: Utc::now(),
            duration_seconds: 150,
            processed_count: 100,
            result_summary: "正常終了".to_string(),
        };
        projection.apply(complete_event).unwrap();

        let read_model = projection.to_read_model();
        assert_eq!(read_model.status, "Completed");
        assert_eq!(read_model.duration_seconds, Some(150));
        assert_eq!(read_model.processed_count, Some(100));
    }

    #[test]
    fn test_batch_failed() {
        let mut projection = BatchHistoryProjection::new("batch-003".to_string());

        let start_event = BatchExecutionEvent::BatchStarted {
            execution_id: "batch-003".to_string(),
            batch_type: "AccountAdjustment".to_string(),
            started_at: Utc::now(),
            started_by: "system".to_string(),
        };
        projection.apply(start_event).unwrap();

        let failed_event = BatchExecutionEvent::BatchFailed {
            execution_id: "batch-003".to_string(),
            failed_at: Utc::now(),
            duration_seconds: 30,
            error_message: "データ検証エラー".to_string(),
        };
        projection.apply(failed_event).unwrap();

        let read_model = projection.to_read_model();
        assert_eq!(read_model.status, "Failed");
        assert_eq!(read_model.error_message, Some("データ検証エラー".to_string()));
    }
}
