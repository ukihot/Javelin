// Storage Metrics - LMDB容量監視
// 目的: map_size枯渇の早期検知
// 用途: アラート、自動拡張判定

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    pub map_size: usize,
    pub used_size: usize,
    pub usage_percent: f64,
    pub page_size: usize,
    pub last_page_no: usize,
    pub entries: usize,
}

impl StorageMetrics {
    pub fn is_critical(&self) -> bool {
        self.usage_percent >= 90.0
    }

    pub fn is_warning(&self) -> bool {
        self.usage_percent >= 80.0
    }

    pub fn should_expand(&self) -> bool {
        self.usage_percent >= 75.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionLagMetrics {
    pub projection_name: String,
    pub projection_version: u32,
    pub latest_event_sequence: u64,
    pub last_processed_sequence: u64,
    pub lag: u64,
    pub lag_seconds: Option<f64>,
}

impl ProjectionLagMetrics {
    pub fn is_critical(&self) -> bool {
        self.lag > 10000
    }

    pub fn is_warning(&self) -> bool {
        self.lag > 1000
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DurabilityPolicy {
    /// 最大耐久性（デフォルト）
    /// - 全書き込みでfsync
    /// - クラッシュ時もデータ保証
    #[default]
    MaxDurability,

    /// バランス型
    /// - メタデータのみfsync
    /// - 性能: 中、リスク: 小
    Balanced,

    /// 最大性能
    /// - fsyncなし
    /// - 性能: 高、リスク: クラッシュ時に最新データ喪失可能
    MaxPerformance,
}
