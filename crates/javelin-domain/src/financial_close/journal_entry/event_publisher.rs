// イベント発行機構
// ドメインイベントを外部システムに発行する責務を持つ

use super::events::JournalEntryEvent;
use crate::error::DomainResult;

/// イベント発行者トレイト
///
/// ドメインイベントを外部システム（EventStore、メッセージキュー等）に
/// 発行する責務を持つ。実装はインフラストラクチャ層で行う。
pub trait JournalEntryEventPublisher: Send + Sync {
    /// イベントを発行する
    ///
    /// # Arguments
    /// * `event` - 発行するドメインイベント
    ///
    /// # Returns
    /// 発行に成功した場合はOk(())、失敗した場合はErr
    fn publish(&self, event: JournalEntryEvent) -> DomainResult<()>;

    /// 複数のイベントをバッチで発行する
    ///
    /// # Arguments
    /// * `events` - 発行するドメインイベントのリスト
    ///
    /// # Returns
    /// すべての発行に成功した場合はOk(())、失敗した場合はErr
    fn publish_batch(&self, events: Vec<JournalEntryEvent>) -> DomainResult<()> {
        for event in events {
            self.publish(event)?;
        }
        Ok(())
    }
}

/// イベントコレクター
///
/// エンティティ内で発生したイベントを一時的に保持し、
/// トランザクション境界でまとめて発行するためのコレクター。
#[derive(Debug, Clone, Default)]
pub struct EventCollector {
    events: Vec<JournalEntryEvent>,
}

impl EventCollector {
    /// 新しいイベントコレクターを作成
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// イベントを追加
    pub fn add(&mut self, event: JournalEntryEvent) {
        self.events.push(event);
    }

    /// 収集したイベントを取得
    pub fn events(&self) -> &[JournalEntryEvent] {
        &self.events
    }

    /// 収集したイベントを消費して取得
    pub fn drain(&mut self) -> Vec<JournalEntryEvent> {
        std::mem::take(&mut self.events)
    }

    /// イベント数を取得
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// イベントが空かチェック
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// イベントをクリア
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn test_event_collector_new() {
        let collector = EventCollector::new();
        assert!(collector.is_empty());
        assert_eq!(collector.len(), 0);
    }

    #[test]
    fn test_event_collector_add() {
        let mut collector = EventCollector::new();

        let event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        collector.add(event);

        assert!(!collector.is_empty());
        assert_eq!(collector.len(), 1);
    }

    #[test]
    fn test_event_collector_drain() {
        let mut collector = EventCollector::new();

        let event1 = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        let event2 = JournalEntryEvent::ApprovalRequested {
            entry_id: "JE001".to_string(),
            requested_by: "user2".to_string(),
            requested_at: Utc::now(),
        };

        collector.add(event1);
        collector.add(event2);

        assert_eq!(collector.len(), 2);

        let events = collector.drain();
        assert_eq!(events.len(), 2);
        assert!(collector.is_empty());
    }

    #[test]
    fn test_event_collector_clear() {
        let mut collector = EventCollector::new();

        let event = JournalEntryEvent::DraftCreated {
            entry_id: "JE001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            voucher_number: "V001".to_string(),
            lines: vec![],
            created_by: "user1".to_string(),
            created_at: Utc::now(),
        };

        collector.add(event);
        assert_eq!(collector.len(), 1);

        collector.clear();
        assert!(collector.is_empty());
    }
}
