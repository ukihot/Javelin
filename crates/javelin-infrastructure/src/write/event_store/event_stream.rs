// Iterator指向のイベントストリーム設計
// Lazy evaluation + Zero-allocation replay

use std::sync::Arc;

use lmdb::{Cursor, Database, Environment, Transaction};
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    types::{AggregateId, Sequence},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub global_sequence: u64,
    pub event_type: String,
    pub aggregate_id: String,
    pub version: u64,
    pub timestamp: String,
    pub payload: Vec<u8>,
}

/// イベントストリームIterator - Lazy evaluation
pub struct EventStream {
    env: Arc<Environment>,
    db: Database,
    from_sequence: Sequence,
    aggregate_filter: Option<AggregateId>,
}

impl EventStream {
    pub fn new(
        env: Arc<Environment>,
        db: Database,
        from_sequence: Sequence,
        aggregate_filter: Option<AggregateId>,
    ) -> Self {
        Self { env, db, from_sequence, aggregate_filter }
    }

    /// Iteratorとして消費
    pub fn iter(self) -> EventStreamIterator {
        EventStreamIterator { stream: self, buffer: Vec::new(), exhausted: false }
    }

    /// バッチ読み込み（内部用）
    fn load_batch(&self, limit: usize) -> InfrastructureResult<Vec<StoredEvent>> {
        let env = Arc::clone(&self.env);
        let db = self.db;
        let from_seq = self.from_sequence.as_u64();
        let aggregate_filter = self.aggregate_filter;

        let txn = env.begin_ro_txn().map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        let cursor = txn
            .open_ro_cursor(db)
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        let mut events = Vec::new();
        let start_key = from_seq.to_be_bytes();

        use lmdb_sys as ffi;

        // Try to position cursor at or after the starting sequence
        // If the database is empty or no matching key exists, this will return NotFound
        match cursor.get(Some(&start_key), None, ffi::MDB_SET_RANGE) {
            Ok((Some(key), value)) => {
                // Process the first event
                if key.len() != 8 {
                    return Err(InfrastructureError::DeserializationFailed(
                        "Invalid key length".to_string(),
                    ));
                }
                let mut key_bytes = [0u8; 8];
                key_bytes.copy_from_slice(key);
                let seq = u64::from_be_bytes(key_bytes);

                if seq >= from_seq {
                    let event: StoredEvent = serde_json::from_slice(value)
                        .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;

                    // Aggregate filterが指定されている場合はフィルタリング
                    let matches_filter = if let Some(filter_id) = aggregate_filter {
                        event.aggregate_id == filter_id.to_string()
                    } else {
                        true
                    };

                    if matches_filter {
                        events.push(event);
                    }
                }

                // Process remaining events
                let mut count = 0;
                loop {
                    if events.len() >= limit {
                        break;
                    }

                    count += 1;
                    if count >= limit * 2 {
                        // Safety limit to avoid infinite loops
                        break;
                    }

                    match cursor.get(None, None, ffi::MDB_NEXT) {
                        Ok((Some(key), value)) => {
                            if key.len() != 8 {
                                return Err(InfrastructureError::DeserializationFailed(
                                    "Invalid key length".to_string(),
                                ));
                            }
                            let mut key_bytes = [0u8; 8];
                            key_bytes.copy_from_slice(key);
                            let seq = u64::from_be_bytes(key_bytes);

                            if seq < from_seq {
                                continue;
                            }

                            let event: StoredEvent =
                                serde_json::from_slice(value).map_err(|e| {
                                    InfrastructureError::DeserializationFailed(e.to_string())
                                })?;

                            // Aggregate filterが指定されている場合はフィルタリング
                            let matches_filter = if let Some(filter_id) = aggregate_filter {
                                event.aggregate_id == filter_id.to_string()
                            } else {
                                true
                            };

                            if matches_filter {
                                events.push(event);
                            }
                        }
                        Ok((None, _)) => {
                            // Unexpected None key
                            break;
                        }
                        Err(lmdb::Error::NotFound) => break,
                        Err(e) => return Err(InfrastructureError::LmdbError(e.to_string())),
                    }
                }
            }
            Ok((None, _)) => {
                // Unexpected None key from MDB_SET_RANGE
            }
            Err(lmdb::Error::NotFound) => {
                // Database is empty or no events at or after from_seq
                // Return empty vector
            }
            Err(e) => return Err(InfrastructureError::LmdbError(e.to_string())),
        }

        Ok(events)
    }
}

/// EventStream Iterator実装
pub struct EventStreamIterator {
    stream: EventStream,
    buffer: Vec<StoredEvent>,
    exhausted: bool,
}

impl Iterator for EventStreamIterator {
    type Item = InfrastructureResult<StoredEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        // バッファが空なら次のバッチをロード
        if self.buffer.is_empty() {
            match self.stream.load_batch(100) {
                Ok(events) if events.is_empty() => {
                    self.exhausted = true;
                    return None;
                }
                Ok(events) => {
                    self.buffer = events;
                    self.buffer.reverse(); // pop効率化のため逆順

                    // 次回のfrom_sequenceを更新
                    if let Some(last) = self.buffer.first() {
                        self.stream.from_sequence = Sequence::new(last.global_sequence + 1);
                    }
                }
                Err(e) => {
                    self.exhausted = true;
                    return Some(Err(e));
                }
            }
        }

        self.buffer.pop().map(Ok)
    }
}

/// イベントストリームビルダー
pub struct EventStreamBuilder {
    env: Arc<Environment>,
    db: Database,
    from_sequence: Sequence,
    aggregate_filter: Option<AggregateId>,
}

impl EventStreamBuilder {
    pub fn new(env: Arc<Environment>, db: Database) -> Self {
        Self { env, db, from_sequence: Sequence::new(0), aggregate_filter: None }
    }

    pub fn from_sequence(mut self, seq: Sequence) -> Self {
        self.from_sequence = seq;
        self
    }

    pub fn for_aggregate(mut self, id: AggregateId) -> Self {
        self.aggregate_filter = Some(id);
        self
    }

    pub fn build(self) -> EventStream {
        EventStream::new(self.env, self.db, self.from_sequence, self.aggregate_filter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_operations() {
        let seq = Sequence::new(100);
        assert_eq!(seq.as_u64(), 100);
        assert_eq!(seq.next().as_u64(), 101);
    }
}
