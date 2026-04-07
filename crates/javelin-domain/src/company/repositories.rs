// Company Repository - 組織体リポジトリ

pub mod company_master_repository;

// Re-exports

use super::entities::Organization;
use crate::common::RepositoryBase;

/// 組織体リポジトリトレイト
///
/// Organization集約を扱う専用リポジトリ。
/// RepositoryBaseを継承し、集約のロード/保存機能を提供。
///
/// # 責務
/// - Organization集約の保存
/// - Organization集約のロード
///
/// # インフラ層での実装
/// マスタデータのため、EventStoreではなくLMDBに直接保存
/// （イベントソーシング不要）
#[allow(async_fn_in_trait)]
pub trait OrganizationRepository: RepositoryBase<Organization> + Send + Sync {
    // 必要に応じて組織体固有のメソッドを追加可能
}
