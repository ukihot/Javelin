// 現代Rust設計：型レベルでの境界定義
// LMDB × CQRS × Event Sourcing に最適化された型システム

use std::fmt;

use serde::{Deserialize, Serialize};

/// Aggregate ID - 固定長8バイト
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct AggregateId([u8; 8]);

impl AggregateId {
    pub fn new(id: [u8; 8]) -> Self {
        Self(id)
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        let bytes = s.as_bytes();
        if bytes.len() > 8 {
            return Err("AggregateId must be 8 bytes or less".to_string());
        }
        let mut arr = [0u8; 8];
        arr[..bytes.len()].copy_from_slice(bytes);
        Ok(Self(arr))
    }

    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }
}

impl fmt::Display for AggregateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // std::fmt::from_fn の代わりに直接実装
        let s = String::from_utf8_lossy(&self.0);
        write!(f, "{}", s.trim_end_matches('\0'))
    }
}

/// Event Key - [AggregateId(8)][Version(8)] = 16バイト固定
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct EventKey {
    aggregate_id: [u8; 8],
    version: [u8; 8],
}

impl EventKey {
    pub fn new(aggregate_id: AggregateId, version: u64) -> Self {
        Self { aggregate_id: *aggregate_id.as_bytes(), version: version.to_be_bytes() }
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        // SAFETY: EventKey は #[repr(C)] で連続配置が保証されている
        unsafe { &*(self as *const Self as *const [u8; 16]) }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let arr = bytes
            .as_array::<16>()
            .ok_or_else(|| "EventKey must be exactly 16 bytes".to_string())?;

        let aggregate_id =
            arr[..8].as_array::<8>().ok_or_else(|| "Invalid aggregate_id".to_string())?;
        let version = arr[8..].as_array::<8>().ok_or_else(|| "Invalid version".to_string())?;

        Ok(Self { aggregate_id: *aggregate_id, version: *version })
    }

    pub fn aggregate_id(&self) -> AggregateId {
        AggregateId::new(self.aggregate_id)
    }

    pub fn version(&self) -> u64 {
        u64::from_be_bytes(self.version)
    }
}

/// Sequence番号 - グローバル順序保証
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Sequence(u64);

impl Sequence {
    pub fn new(seq: u64) -> Self {
        Self(seq)
    }

    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    pub fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_be_bytes(bytes))
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "seq:{}", self.0)
    }
}

/// 楽観的ロック用の期待バージョン
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectedVersion(pub u64);

impl ExpectedVersion {
    pub fn any() -> Self {
        Self(u64::MAX)
    }

    pub fn exact(version: u64) -> Self {
        Self(version)
    }

    pub fn matches(&self, actual: u64) -> bool {
        self.0 == u64::MAX || self.0 == actual
    }
}

/// Snapshot戦略 - const genericsで型化
pub struct SnapshotPolicy<const EVERY: u64>;

impl<const EVERY: u64> SnapshotPolicy<EVERY> {
    pub fn should_snapshot(version: u64) -> bool {
        version.is_multiple_of(EVERY)
    }
}

/// イベントヘッダー - Zero-copy deserialize用
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EventHeader {
    pub version: u16,
    pub kind: u16,
    pub timestamp: u64,
}

impl EventHeader {
    pub fn new(version: u16, kind: u16) -> Self {
        Self {
            version,
            kind,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_id() {
        let id = AggregateId::parse("agg-001").unwrap();
        assert_eq!(id.to_string(), "agg-001");
    }

    #[test]
    fn test_event_key() {
        let agg_id = AggregateId::parse("agg-001").unwrap();
        let key = EventKey::new(agg_id, 42);

        assert_eq!(key.version(), 42);
        assert_eq!(key.aggregate_id(), agg_id);

        // Round-trip test
        let bytes = key.as_bytes();
        let key2 = EventKey::from_bytes(bytes).unwrap();
        assert_eq!(key, key2);
    }

    #[test]
    fn test_sequence() {
        let seq = Sequence::new(100);
        assert_eq!(seq.next().as_u64(), 101);
        assert_eq!(seq.to_string(), "seq:100");
    }

    #[test]
    fn test_expected_version() {
        let expected = ExpectedVersion::exact(5);
        assert!(expected.matches(5));
        assert!(!expected.matches(6));

        let any = ExpectedVersion::any();
        assert!(any.matches(5));
        assert!(any.matches(100));
    }

    #[test]
    fn test_snapshot_policy() {
        type Every100 = SnapshotPolicy<100>;

        assert!(Every100::should_snapshot(100));
        assert!(Every100::should_snapshot(200));
        assert!(!Every100::should_snapshot(150));
    }
}
